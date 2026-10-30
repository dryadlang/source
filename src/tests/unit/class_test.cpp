#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/common/utils.h"

using namespace dryad;

namespace {

void run_program(const std::string& code) {
    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    if (parser.has_error()) {
        throw std::runtime_error("Parser error: " + parser.error_message());
    }
    Interpreter interpreter;
    interpreter.execute(program.get());
}

Value run_and_get(const std::string& code, const std::string& name) {
    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    if (parser.has_error()) {
        throw std::runtime_error("Parser error: " + parser.error_message());
    }
    Interpreter interpreter;
    interpreter.execute(program.get());
    return interpreter.global_env()->get(name);
}

} // namespace

TEST(ClassTest, SimpleClassDeclaration) {
    std::string code = R"(
        class Point {
            constructor(x, y) {
                this.x = x;
                this.y = y;
            }
            
            getX() {
                return this.x;
            }
        }
    )";
    
    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(tokens);
    auto program = parser.parse();
    
    ASSERT_FALSE(parser.has_error()) << "Parser error: " << parser.error_message();
}

TEST(ClassTest, NewExpression) {
    std::string code = R"(
        class Point {
            constructor(x, y) {
                this.x = x;
                this.y = y;
            }
        }
        let p = new Point(10, 20);
    )";
    
    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(tokens);
    auto program = parser.parse();
    
    ASSERT_FALSE(parser.has_error()) << "Parser error: " << parser.error_message();
}

TEST(ClassTest, ClassInstantiationAndMethodCall) {
    std::string code = R"(
class Point {
    constructor(x, y) {
        this.x = x;
        this.y = y;
    }
    
    getX() {
        return this.x;
    }
}

let p = new Point(10, 20);
let result = p.getX();
    )";
    
    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(tokens);
    auto program = parser.parse();
    
    ASSERT_FALSE(parser.has_error()) << "Parser error: " << parser.error_message();
    
    Interpreter interpreter;
    interpreter.execute(program.get());
    
    auto p = interpreter.global_env()->get("p");
    ASSERT_TRUE(p.is_instance());
    
    auto result = interpreter.global_env()->get("result");
    ASSERT_TRUE(result.is_integer());
    EXPECT_EQ(result.as_integer(), 10);
}

TEST(ClassTest, ExtendsInheritsAndOverridesMethods) {
    const std::string code = R"(
class Animal {
    speak() {
        return "generic";
    }
}

class Dog extends Animal {
    speak() {
        return "woof";
    }
}

let d = new Dog();
let sound = d.speak();
    )";

    auto sound = run_and_get(code, "sound");
    ASSERT_TRUE(sound.is_string());
    EXPECT_EQ(sound.as_string(), "woof");
}

TEST(ClassTest, ExtendsInheritsSuperMethodWhenNotOverridden) {
    const std::string code = R"(
class Animal {
    legs() {
        return 4;
    }
}

class Dog extends Animal {
    constructor() {
        this.name = "dog";
    }
}

let d = new Dog();
let count = d.legs();
    )";

    auto count = run_and_get(code, "count");
    ASSERT_TRUE(count.is_integer());
    EXPECT_EQ(count.as_integer(), 4);
}

TEST(ClassTest, AbstractClassCannotBeInstantiated) {
    const std::string code = R"(
abstract class Shape {
    abstract area();
}

let s = new Shape();
    )";

    EXPECT_THROW(run_program(code), DryadException);
}

TEST(ClassTest, ConcreteSubclassCanBeInstantiated) {
    const std::string code = R"(
abstract class Shape {
    abstract area();
}

class Circle extends Shape {
    constructor(radius) {
        this.radius = radius;
    }

    area() {
        return this.radius * this.radius * 3;
    }
}

let c = new Circle(2);
let size = c.area();
    )";

    auto size = run_and_get(code, "size");
    ASSERT_TRUE(size.is_integer());
    EXPECT_EQ(size.as_integer(), 12);
}

TEST(ClassTest, IncompleteSubclassRejectedAtDefinition) {
    const std::string code = R"(
abstract class Shape {
    abstract area();
}

class Incomplete extends Shape {
    constructor() {
        this.x = 1;
    }
}
    )";

    EXPECT_THROW(run_program(code), DryadException);
}

TEST(ClassTest, AbstractMethodOnlyAllowedInAbstractClass) {
    const std::string code = R"(
class Bad {
    abstract area();
}
    )";

    EXPECT_THROW(run_program(code), DryadException);
}

TEST(ClassTest, AbstractClassWithConcreteMethod) {
    const std::string code = R"(
abstract class Stream {
    abstract close();

    dispose() {
        this.close();
    }
}

class MemoryStream extends Stream {
    constructor() {
        this.open = true;
    }

    close() {
        this.open = false;
    }
}

let s = new MemoryStream();
s.dispose();
let stillOpen = s.open;
    )";

    auto stillOpen = run_and_get(code, "stillOpen");
    ASSERT_TRUE(stillOpen.is_boolean());
    EXPECT_FALSE(stillOpen.as_boolean());
}

TEST(ClassTest, SuperConstructorInitializesParentField) {
    const std::string code = R"(
class Animal {
    constructor(name) {
        this.name = name;
    }
}

class Dog extends Animal {
    constructor(name, breed) {
        super(name);
        this.breed = breed;
    }
}

let d = new Dog("Rex", "Shepherd");
let name = d.name;
let breed = d.breed;
    )";

    auto name = run_and_get(code, "name");
    auto breed = run_and_get(code, "breed");
    EXPECT_EQ(name.as_string(), "Rex");
    EXPECT_EQ(breed.as_string(), "Shepherd");
}

TEST(ClassTest, SuperMethodCallUsesParentImplementation) {
    const std::string code = R"(
class Counter {
    constructor() {
        this.value = 1;
    }

    next() {
        return this.value;
    }
}

class DoubleCounter extends Counter {
    constructor() {
        super();
        this.value = 2;
    }

    next() {
        let base = super.next();
        return base * 2;
    }
}

let c = new DoubleCounter();
let first = c.next();
    )";

    auto first = run_and_get(code, "first");
    ASSERT_TRUE(first.is_integer());
    EXPECT_EQ(first.as_integer(), 4);
}

TEST(ClassTest, MultiLevelSuperChain) {
    const std::string code = R"(
class A {
    constructor() {
        this.a_part = "A";
    }

    chain() {
        return this.a_part;
    }
}

class B extends A {
    constructor() {
        super();
        this.b_part = "B";
    }

    chain() {
        return super.chain() + "->" + this.b_part;
    }
}

class C extends B {
    constructor() {
        super();
        this.c_part = "C";
    }

    chain() {
        return super.chain() + "->" + this.c_part;
    }
}

let c = new C();
let chain = c.chain();
    )";

    auto chain = run_and_get(code, "chain");
    EXPECT_EQ(chain.as_string(), "A->B->C");
}

TEST(ClassTest, CircularInheritanceRejected) {
    const std::string code = R"(
class A {
    constructor() {}
}

class B extends A {
    constructor() {
        super();
    }
}

class A extends B {
    constructor() {
        super();
    }
}
    )";

    EXPECT_THROW(run_program(code), DryadException);
}

TEST(ClassTest, RecursiveFunctionHitsDepthLimit) {
    const std::string code = R"(
function recurse(n) {
    return recurse(n + 1);
}

recurse(0);
    )";

    EXPECT_THROW(run_program(code), DryadException);
}

TEST(ClassTest, SuperOutsideConstructorRejected) {
    const std::string code = R"(
class Parent {
    constructor() {}
}

class Child extends Parent {
    constructor() {
        super();
    }

    bad() {
        super();
    }
}

let c = new Child();
c.bad();
    )";

    EXPECT_THROW(run_program(code), DryadException);
}
