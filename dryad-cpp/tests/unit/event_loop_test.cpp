#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/runtime/event_loop.h"

using namespace dryad;

TEST(EventLoopTest, ScheduleAndPump) {
    EventLoop::instance().clear();
    std::string source = R"(
        let ran = 0;
        function tick() {
            ran = 1;
            return null;
        }
        __runtime_schedule_microtask(tick);
    )";

    Lexer lexer(source);
    Parser parser(lexer.tokenize());
    auto program = parser.parse();
    ASSERT_FALSE(parser.has_error());

    Interpreter interpreter;
    interpreter.execute(program.get());
    EXPECT_EQ(interpreter.global_env()->get("ran").as_integer(), 1);
    EXPECT_EQ(EventLoop::instance().pending_count(), 0u);
}

TEST(EventLoopTest, DryadPendingCount) {
    std::string source = R"(
        import { EventLoop_schedule, EventLoop_pendingCount } from "@std/async/event_loop";
        function noop() { return null; }
        EventLoop_schedule(noop);
        EventLoop_schedule(noop);
        let pending = EventLoop_pendingCount();
    )";

    Lexer lexer(source);
    Parser parser(lexer.tokenize());
    auto program = parser.parse();
    ASSERT_FALSE(parser.has_error());

    Interpreter interpreter;
    interpreter.set_stdlib_root(DRYAD_STDLIB_DIR);
    EventLoop::instance().clear();
    interpreter.execute(program.get());
    EXPECT_EQ(interpreter.global_env()->get("pending").as_integer(), 2);
}
