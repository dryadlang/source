#include "dryad/runtime/thread_manager.h"
#include "dryad/runtime/function.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/common/utils.h"
#include <chrono>
#include <thread>

namespace dryad {

std::mutex ThreadManager::mutex_;
std::string ThreadManager::stdlib_root_;
std::unordered_map<int64_t, std::shared_ptr<ThreadJob>> ThreadManager::jobs_;
std::unordered_map<int64_t, std::unique_ptr<ThreadPool>> ThreadManager::pools_;
int64_t ThreadManager::next_job_id_ = 1;
int64_t ThreadManager::next_pool_id_ = 1;

namespace {

Value execute_dryad_job(const Value& function, const std::vector<Value>& arguments) {
    if (!function.is_function()) {
        throw DryadException("thread job requires a function");
    }

    Interpreter interpreter;
    const std::string& root = ThreadManager::stdlib_root();
    if (!root.empty()) {
        interpreter.set_stdlib_root(root);
    }

    return function.as_function()->call(&interpreter, arguments);
}

void complete_job(const std::shared_ptr<ThreadJob>& job, bool success, Value result,
                  const std::string& error) {
    {
        std::lock_guard<std::mutex> lock(job->mutex);
        job->success = success;
        job->result = std::move(result);
        job->error = error;
        job->done = true;
    }
    job->cv.notify_all();
}

void wait_for_job(const std::shared_ptr<ThreadJob>& job) {
    std::unique_lock<std::mutex> lock(job->mutex);
    job->cv.wait(lock, [&]() { return job->done.load(); });
}

} // namespace

void ThreadManager::set_stdlib_root(const std::string& path) {
    std::lock_guard<std::mutex> lock(mutex_);
    stdlib_root_ = path;
}

const std::string& ThreadManager::stdlib_root() {
    std::lock_guard<std::mutex> lock(mutex_);
    return stdlib_root_;
}

int64_t ThreadManager::spawn(Value function, const std::vector<Value>& arguments) {
    auto job = std::make_shared<ThreadJob>();
    job->function = std::move(function);
    job->arguments = arguments;

    int64_t id = 0;
    {
        std::lock_guard<std::mutex> lock(mutex_);
        id = next_job_id_++;
        jobs_[id] = job;
    }

    std::thread([job]() {
        try {
            Value result = execute_dryad_job(job->function, job->arguments);
            complete_job(job, true, std::move(result), "");
        } catch (const std::exception& e) {
            complete_job(job, false, Value(), e.what());
        } catch (...) {
            complete_job(job, false, Value(), "unknown thread error");
        }
    }).detach();

    return id;
}

Value ThreadManager::join(int64_t job_id) {
    std::shared_ptr<ThreadJob> job;
    {
        std::lock_guard<std::mutex> lock(mutex_);
        auto it = jobs_.find(job_id);
        if (it == jobs_.end()) {
            throw DryadException("invalid thread job id: " + std::to_string(job_id));
        }
        job = it->second;
        jobs_.erase(it);
    }

    wait_for_job(job);

    if (!job->success) {
        throw DryadException(job->error.empty() ? "thread job failed" : job->error);
    }
    return job->result;
}

ThreadPool::ThreadPool(size_t worker_count)
    : worker_count_(worker_count == 0 ? 1 : worker_count) {
    workers_.reserve(worker_count_);
    for (size_t i = 0; i < worker_count_; ++i) {
        workers_.emplace_back([this]() { worker_loop(); });
    }
}

ThreadPool::~ThreadPool() {
    {
        std::lock_guard<std::mutex> lock(queue_mutex_);
        stopping_ = true;
    }
    queue_cv_.notify_all();
    for (auto& worker : workers_) {
        if (worker.joinable()) {
            worker.join();
        }
    }
}

int64_t ThreadPool::submit(Value function, const std::vector<Value>& arguments) {
    auto job = std::make_shared<ThreadJob>();
    job->function = std::move(function);
    job->arguments = arguments;

    int64_t id = 0;
    {
        std::lock_guard<std::mutex> lock(jobs_mutex_);
        id = next_pool_job_id_++;
        jobs_[id] = job;
    }

    PoolJob pool_job;
    pool_job.job = job;

    {
        std::lock_guard<std::mutex> lock(queue_mutex_);
        queue_.push_back(std::move(pool_job));
    }
    queue_cv_.notify_one();
    return id;
}

Value ThreadPool::wait(int64_t job_id) {
    std::shared_ptr<ThreadJob> job;
    {
        std::lock_guard<std::mutex> lock(jobs_mutex_);
        auto it = jobs_.find(job_id);
        if (it == jobs_.end()) {
            throw DryadException("invalid pool job id: " + std::to_string(job_id));
        }
        job = it->second;
    }

    wait_for_job(job);

    {
        std::lock_guard<std::mutex> lock(jobs_mutex_);
        jobs_.erase(job_id);
    }

    if (!job->success) {
        throw DryadException(job->error.empty() ? "pool job failed" : job->error);
    }
    return job->result;
}

void ThreadPool::worker_loop() {
    while (true) {
        PoolJob pool_job;
        {
            std::unique_lock<std::mutex> lock(queue_mutex_);
            queue_cv_.wait(lock, [&]() { return stopping_ || !queue_.empty(); });
            if (stopping_ && queue_.empty()) {
                return;
            }
            pool_job = std::move(queue_.front());
            queue_.erase(queue_.begin());
        }

        auto job = pool_job.job;
        if (!job) {
            continue;
        }

        try {
            Value result = execute_dryad_job(job->function, job->arguments);
            complete_job(job, true, std::move(result), "");
        } catch (const std::exception& e) {
            complete_job(job, false, Value(), e.what());
        } catch (...) {
            complete_job(job, false, Value(), "unknown pool job error");
        }
    }
}

int64_t ThreadManager::create_pool(size_t worker_count) {
    std::lock_guard<std::mutex> lock(mutex_);
    int64_t id = next_pool_id_++;
    pools_[id] = std::make_unique<ThreadPool>(worker_count);
    return id;
}

void ThreadManager::destroy_pool(int64_t pool_handle) {
    std::lock_guard<std::mutex> lock(mutex_);
    pools_.erase(pool_handle);
}

int64_t ThreadManager::pool_submit(int64_t pool_handle, Value function,
                                   const std::vector<Value>& arguments) {
    ThreadPool* pool = nullptr;
    {
        std::lock_guard<std::mutex> lock(mutex_);
        auto it = pools_.find(pool_handle);
        if (it == pools_.end()) {
            throw DryadException("invalid thread pool handle: " + std::to_string(pool_handle));
        }
        pool = it->second.get();
    }
    return pool->submit(std::move(function), arguments);
}

Value ThreadManager::pool_wait(int64_t pool_handle, int64_t job_id) {
    ThreadPool* pool = nullptr;
    {
        std::lock_guard<std::mutex> lock(mutex_);
        auto it = pools_.find(pool_handle);
        if (it == pools_.end()) {
            throw DryadException("invalid thread pool handle: " + std::to_string(pool_handle));
        }
        pool = it->second.get();
    }
    return pool->wait(job_id);
}

int64_t ThreadManager::hardware_concurrency() {
    const unsigned int count = std::thread::hardware_concurrency();
    return count == 0 ? 1 : static_cast<int64_t>(count);
}

void ThreadManager::sleep_ms(int64_t milliseconds) {
    if (milliseconds <= 0) {
        return;
    }
    std::this_thread::sleep_for(std::chrono::milliseconds(milliseconds));
}

} // namespace dryad
