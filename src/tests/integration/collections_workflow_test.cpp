#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"
#include <stdexcept>

using namespace dryad;

namespace {

Value run_workflow(const std::string& source) {
    Lexer lexer(source);
    Parser parser(lexer.tokenize());
    auto program = parser.parse();
    if (parser.has_error()) {
        throw std::runtime_error("Parser error: " + parser.error_message());
    }

    Interpreter interpreter;
    interpreter.set_stdlib_root(DRYAD_STDLIB_DIR);
    interpreter.execute(program.get());
    return interpreter.global_env()->get("ok");
}

} // namespace

TEST(CollectionsWorkflow, ListMapSetTogether) {
    const std::string source = R"(
        import { List, Map, Set, Queue } from "@std/collections";

        let list = new List();
        list.add(1);
        list.add(2);
        list.add(3);

        let doubled = list.map(function(x) {
            return x * 2;
        });

        let map = new Map();
        map.put("age", 30);
        map.put("score", 99);

        let unique = new Set();
        unique.add("foo");
        unique.add("foo");
        unique.add("bar");

        let filtered = list.filter(function(x) {
            return x > 1;
        });

        let total = list.reduce(function(acc, x) {
            return acc + x;
        }, 0);

        let queue = new Queue(2);
        queue.enqueue(100);
        queue.enqueue(200);
        queue.enqueue(300);

        let ok = doubled.count == 3 &&
                 doubled.get(0) == 2 &&
                 map.get("age") == 30 &&
                 unique.size == 2 &&
                 filtered.count == 2 &&
                 total == 6 &&
                 queue.dequeue() == 100 &&
                 queue.peek() == 200;
    )";

    EXPECT_TRUE(run_workflow(source).is_truthy());
}
