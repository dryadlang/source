#include <gtest/gtest.h>
#include <filesystem>
#include <fstream>
#include <functional>
#include <map>
#include <regex>
#include <set>
#include <string>
#include <vector>

using namespace std::filesystem;

namespace {

std::map<std::string, std::set<std::string>> load_imports() {
    std::map<std::string, std::set<std::string>> graph;
    const std::regex import_re(R"(@std/([A-Za-z0-9_]+)(?:/|"))");
    for (const auto& entry : recursive_directory_iterator(DRYAD_STDLIB_DIR)) {
        if (!entry.is_regular_file() || entry.path().extension() != ".dryad") {
            continue;
        }
        const auto rel = relative(entry.path(), DRYAD_STDLIB_DIR);
        auto part = rel.begin();
        if (part == rel.end() || part->string() != "@std") {
            continue;
        }
        ++part;
        if (part == rel.end()) {
            continue;
        }
        const std::string module = std::next(part) == rel.end() ? path(*part).stem().string() : part->string();

        std::ifstream in(entry.path());
        std::string line;
        while (std::getline(in, line)) {
            if (line.find("from \"") == std::string::npos) {
                continue;
            }
            for (std::sregex_iterator it(line.begin(), line.end(), import_re), end; it != end; ++it) {
                if ((*it)[1].str() != module) {
                    graph[module].insert((*it)[1].str());
                }
            }
        }
        graph.try_emplace(module);
    }
    return graph;
}

std::set<std::string> allowed_imports(const std::string& module) {
    if (module == "bytes") return {"core", "buffers"};
    if (module == "buffers") return {"core"};
    return {}; // no rule yet for other modules in this wave
}

} // namespace

TEST(StdlibLayersTest, BytesAndBuffersRespectLayers) {
    auto graph = load_imports();
    ASSERT_EQ(graph.count("bytes"), 1u);
    ASSERT_EQ(graph.count("buffers"), 1u);
    EXPECT_EQ(graph["bytes"].count("buffers"), 1u);
    EXPECT_EQ(graph["bytes"].count("core"), 1u);
    EXPECT_EQ(graph["buffers"].count("core"), 1u);
    for (const auto& [module, imports] : graph) {
        const auto allowed = allowed_imports(module);
        if (allowed.empty()) {
            continue;
        }
        for (const auto& dep : imports) {
            EXPECT_TRUE(allowed.count(dep) == 1)
                << module << " may not import @std/" << dep;
        }
    }
}

TEST(StdlibLayersTest, NoImportCycles) {
    auto graph = load_imports();
    std::map<std::string, int> state; // 0 unvisited, 1 visiting, 2 done
    std::function<bool(const std::string&)> visit = [&](const std::string& node) {
        if (state[node] == 1) return false;
        if (state[node] == 2) return true;
        state[node] = 1;
        for (const auto& dep : graph[node]) {
            if (graph.count(dep) && !visit(dep)) return false;
        }
        state[node] = 2;
        return true;
    };
    for (const auto& entry : graph) {
        EXPECT_TRUE(visit(entry.first)) << "import cycle involving @std/" << entry.first;
    }
}
