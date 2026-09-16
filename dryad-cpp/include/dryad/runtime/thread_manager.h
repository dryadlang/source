#ifndef DRYAD_RUNTIME_THREAD_MANAGER_H
#define DRYAD_RUNTIME_THREAD_MANAGER_H

#include "dryad/runtime/value.h"
#include <atomic>
#include <condition_variable>
#include <cstdint>
#include <memory>
#include <mutex>
#include <string>
#include <thread>
#include <unordered_map>
#include <vector>

namespace dryad {

struct ThreadJob {
    Value function;
    std::vector<Value> arguments;
    std::atomic<bool> done{false};
    std::atomic<bool> success{false};
    Value result;
    std::string error;
    std::mutex mutex;
    std::condition_variable cv;
};

struct ThreadPool {
    explicit ThreadPool(size_t worker_count);
    ~ThreadPool();

    int64_t submit(Value function, const std::vector<Value>& arguments);
    Value wait(int64_t job_id);

    size_t worker_count() const { return worker_count_; }

private:
    struct PoolJob {
        std::shared_ptr<ThreadJob> job;
    };

    void worker_loop();

    size_t worker_count_;
    std::vector<std::thread> workers_;
    std::mutex queue_mutex_;
    std::vector<PoolJob> queue_;
    std::condition_variable queue_cv_;
    std::atomic<bool> stopping_{false};

    std::mutex jobs_mutex_;
    std::unordered_map<int64_t, std::shared_ptr<ThreadJob>> jobs_;
    int64_t next_pool_job_id_ = 1;
};

class ThreadManager {
public:
    static void set_stdlib_root(const std::string& path);
    static const std::string& stdlib_root();

    static int64_t spawn(Value function, const std::vector<Value>& arguments);
    static Value join(int64_t job_id);

    static int64_t create_pool(size_t worker_count);
    static void destroy_pool(int64_t pool_handle);
    static int64_t pool_submit(int64_t pool_handle, Value function,
                               const std::vector<Value>& arguments);
    static Value pool_wait(int64_t pool_handle, int64_t job_id);

    static int64_t hardware_concurrency();
    static void sleep_ms(int64_t milliseconds);

private:
    static std::mutex mutex_;
    static std::string stdlib_root_;
    static std::unordered_map<int64_t, std::shared_ptr<ThreadJob>> jobs_;
    static std::unordered_map<int64_t, std::unique_ptr<ThreadPool>> pools_;
    static int64_t next_job_id_;
    static int64_t next_pool_id_;
};

} // namespace dryad

#endif // DRYAD_RUNTIME_THREAD_MANAGER_H
