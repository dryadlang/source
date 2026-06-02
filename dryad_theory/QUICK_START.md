# Dryad C++ Reimplementation — Quick Start Guide

**Ready to start building? Follow this guide to begin Phase 0!** 🚀

---

## 🎯 Prerequisites

### Required Tools
- **C++ Compiler**: GCC 10+ or Clang 12+ (C++20 support required)
- **CMake**: Version 3.20 or higher
- **Git**: For version control
- **Google Test**: Testing framework (installed via package manager or CMake)

### Optional Tools
- **ccache**: Faster rebuilds
- **ninja**: Faster build system (alternative to make)
- **clang-format**: Code formatting
- **clang-tidy**: Static analysis

---

## 📦 Installation Instructions

### Ubuntu/Debian
```bash
sudo apt update
sudo apt install -y \
    build-essential \
    cmake \
    git \
    libgtest-dev \
    ccache \
    ninja-build \
    clang-format \
    clang-tidy

# Build Google Test (if not pre-built)
cd /usr/src/gtest
sudo cmake CMakeLists.txt
sudo make
sudo cp lib/*.a /usr/lib
```

### macOS
```bash
# Install Homebrew if not present
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install cmake googletest ccache ninja llvm
```

### Windows (WSL2)
```bash
# Use Ubuntu instructions in WSL2
wsl --install
# Then follow Ubuntu steps above
```

---

## 🏗️ Phase 0: Foundation — Step-by-Step

### Step 1: Initialize Repository (15 minutes)

```bash
# Navigate to your workspace
cd ~/projects  # or wherever you keep projects

# Create and initialize repository
mkdir dryad-cpp
cd dryad-cpp
git init
git branch -M main

# Create .gitignore
cat > .gitignore << 'EOF'
# Build artifacts
build/
*.o
*.a
*.so
*.dylib
*.dll

# CMake
CMakeCache.txt
CMakeFiles/
cmake_install.cmake
compile_commands.json
CTestTestfile.cmake

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db

# Test outputs
Testing/
*.log
EOF

# Create LICENSE
cat > LICENSE << 'EOF'
MIT License

Copyright (c) 2026 Dryad Development Team

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
EOF

# Create README
cat > README.md << 'EOF'
# Dryad Programming Language — C++ Implementation

**Version**: 2.0.0  
**Status**: Active Development  
**Architecture**: Minimal Intrinsics Runtime + Self-Hosting Stdlib

## Quick Start

```bash
# Build
cmake -B build
cmake --build build

# Run tests
cd build && ctest

# Run REPL (when implemented)
./build/bin/dryad repl
```

## Documentation

See `docs/` for architecture and implementation details.

For theoretical specification, see `../dryad_theory/dryad_theoretical_foundation_v2.pdf`

## License

MIT — See LICENSE file
EOF

# Initial commit
git add .
git commit -m "Initial commit: C++ reimplementation foundation"
```

### Step 2: Create Directory Structure (10 minutes)

```bash
# Create directory tree
mkdir -p include/dryad/{runtime,compiler,common}
mkdir -p src/{runtime,compiler,tools,common}
mkdir -p src/runtime/{core,intrinsics,gc,bytecode}
mkdir -p src/compiler/{lexer,parser,ast,codegen}
mkdir -p src/tools/{cli,repl}
mkdir -p tests/{unit,integration,stdlib}
mkdir -p tests/unit/{runtime,compiler}
mkdir -p stdlib/{runtime,core,async,high_level}
mkdir -p docs
mkdir -p examples
mkdir -p benchmarks
mkdir -p scripts

echo "✅ Directory structure created"
tree -L 2 .  # View structure
```

### Step 3: Create Root CMakeLists.txt (15 minutes)

```bash
cat > CMakeLists.txt << 'EOF'
cmake_minimum_required(VERSION 3.20)
project(Dryad VERSION 2.0.0 LANGUAGES CXX)

# C++20 standard required
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Export compile commands for clangd/LSP
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Compiler flags
if(CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
    add_compile_options(
        -Wall
        -Wextra
        -Wpedantic
        -Werror
        -Wno-unused-parameter
    )
endif()

# Build types
if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Debug)
endif()

message(STATUS "Build type: ${CMAKE_BUILD_TYPE}")

# Options
option(DRYAD_BUILD_TESTS "Build test suite" ON)
option(DRYAD_BUILD_BENCHMARKS "Build benchmarks" OFF)
option(DRYAD_ENABLE_ASAN "Enable AddressSanitizer" OFF)
option(DRYAD_ENABLE_COVERAGE "Enable code coverage" OFF)
option(DRYAD_ENABLE_JIT "Enable JIT compilation (requires LLVM)" OFF)
option(DRYAD_ENABLE_AOT "Enable AOT compilation (requires LLVM)" OFF)

# AddressSanitizer
if(DRYAD_ENABLE_ASAN)
    add_compile_options(-fsanitize=address -fno-omit-frame-pointer)
    add_link_options(-fsanitize=address)
    message(STATUS "AddressSanitizer enabled")
endif()

# Code coverage
if(DRYAD_ENABLE_COVERAGE)
    add_compile_options(--coverage)
    add_link_options(--coverage)
    message(STATUS "Code coverage enabled")
endif()

# Find dependencies
if(DRYAD_BUILD_TESTS)
    find_package(GTest REQUIRED)
    include(GoogleTest)
endif()

if(DRYAD_ENABLE_JIT OR DRYAD_ENABLE_AOT)
    find_package(LLVM REQUIRED CONFIG)
    message(STATUS "Found LLVM ${LLVM_PACKAGE_VERSION}")
    include_directories(${LLVM_INCLUDE_DIRS})
endif()

# Subdirectories
add_subdirectory(src)

if(DRYAD_BUILD_TESTS)
    enable_testing()
    add_subdirectory(tests)
endif()

if(DRYAD_BUILD_BENCHMARKS)
    add_subdirectory(benchmarks)
endif()

# Install targets (later)
# install(TARGETS dryad_cli DESTINATION bin)

# Print summary
message(STATUS "")
message(STATUS "Dryad v${PROJECT_VERSION} Configuration Summary:")
message(STATUS "  C++ Standard: ${CMAKE_CXX_STANDARD}")
message(STATUS "  Compiler: ${CMAKE_CXX_COMPILER_ID} ${CMAKE_CXX_COMPILER_VERSION}")
message(STATUS "  Build Type: ${CMAKE_BUILD_TYPE}")
message(STATUS "  Build Tests: ${DRYAD_BUILD_TESTS}")
message(STATUS "  Build Benchmarks: ${DRYAD_BUILD_BENCHMARKS}")
message(STATUS "  AddressSanitizer: ${DRYAD_ENABLE_ASAN}")
message(STATUS "  Code Coverage: ${DRYAD_ENABLE_COVERAGE}")
message(STATUS "  JIT Support: ${DRYAD_ENABLE_JIT}")
message(STATUS "  AOT Support: ${DRYAD_ENABLE_AOT}")
message(STATUS "")
EOF

echo "✅ Root CMakeLists.txt created"
```

### Step 4: Create src/CMakeLists.txt (10 minutes)

```bash
cat > src/CMakeLists.txt << 'EOF'
# Runtime library
add_subdirectory(runtime)

# Compiler library
add_subdirectory(compiler)

# Common utilities
add_subdirectory(common)

# CLI tools
add_subdirectory(tools)
EOF

# Create stub CMakeLists for subdirectories
cat > src/runtime/CMakeLists.txt << 'EOF'
add_library(dryad_runtime STATIC
    # Will add .cpp files here
)

target_include_directories(dryad_runtime PUBLIC
    ${PROJECT_SOURCE_DIR}/include
)

# Link dependencies
# target_link_libraries(dryad_runtime PRIVATE ...)
EOF

cat > src/compiler/CMakeLists.txt << 'EOF'
add_library(dryad_compiler STATIC
    # Will add .cpp files here
)

target_include_directories(dryad_compiler PUBLIC
    ${PROJECT_SOURCE_DIR}/include
)

target_link_libraries(dryad_compiler PRIVATE
    dryad_runtime
)
EOF

cat > src/common/CMakeLists.txt << 'EOF'
add_library(dryad_common STATIC
    # Will add .cpp files here
)

target_include_directories(dryad_common PUBLIC
    ${PROJECT_SOURCE_DIR}/include
)
EOF

cat > src/tools/CMakeLists.txt << 'EOF'
# CLI executable (main entry point)
add_executable(dryad_cli
    cli/main.cpp
)

target_link_libraries(dryad_cli PRIVATE
    dryad_runtime
    dryad_compiler
    dryad_common
)

set_target_properties(dryad_cli PROPERTIES
    OUTPUT_NAME dryad
    RUNTIME_OUTPUT_DIRECTORY ${PROJECT_BINARY_DIR}/bin
)

# REPL (later)
# add_executable(dryad_repl repl/repl.cpp)
# target_link_libraries(dryad_repl PRIVATE ...)
EOF

echo "✅ src/ CMakeLists.txt files created"
```

### Step 5: Create Placeholder main.cpp (5 minutes)

```bash
cat > src/tools/cli/main.cpp << 'EOF'
#include <iostream>

int main(int argc, char** argv) {
    std::cout << "Dryad Programming Language v2.0.0\n";
    std::cout << "C++ Implementation — Coming Soon!\n";
    return 0;
}
EOF

echo "✅ Placeholder CLI created"
```

### Step 6: Create Test Infrastructure (10 minutes)

```bash
cat > tests/CMakeLists.txt << 'EOF'
# Unit tests
add_subdirectory(unit)

# Integration tests (later)
# add_subdirectory(integration)
EOF

cat > tests/unit/CMakeLists.txt << 'EOF'
# Sample test to verify framework works
add_executable(sample_test
    sample_test.cpp
)

target_link_libraries(sample_test PRIVATE
    GTest::gtest
    GTest::gtest_main
)

gtest_discover_tests(sample_test)
EOF

cat > tests/unit/sample_test.cpp << 'EOF'
#include <gtest/gtest.h>

TEST(Sample, AlwaysPasses) {
    EXPECT_EQ(1 + 1, 2);
}

TEST(Sample, StringComparison) {
    std::string hello = "Hello, World!";
    EXPECT_EQ(hello, "Hello, World!");
}
EOF

echo "✅ Test infrastructure created"
```

### Step 7: First Build! (5 minutes)

```bash
# Configure
cmake -B build -DCMAKE_BUILD_TYPE=Debug

# Build
cmake --build build

# Run tests
cd build && ctest --output-on-failure

# Run CLI
./bin/dryad

# Expected output:
# Dryad Programming Language v2.0.0
# C++ Implementation — Coming Soon!

echo "✅ First build successful!"
```

### Step 8: Set Up CI/CD (15 minutes)

```bash
mkdir -p .github/workflows

cat > .github/workflows/ci.yml << 'EOF'
name: CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]

jobs:
  build-and-test:
    name: ${{ matrix.os }} - ${{ matrix.build_type }}
    runs-on: ${{ matrix.os }}
    
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-latest, macos-latest]
        build_type: [Debug, Release]
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install dependencies (Ubuntu)
      if: matrix.os == 'ubuntu-latest'
      run: |
        sudo apt-get update
        sudo apt-get install -y cmake g++ libgtest-dev ninja-build
    
    - name: Install dependencies (macOS)
      if: matrix.os == 'macos-latest'
      run: |
        brew install cmake googletest ninja
    
    - name: Configure CMake
      run: |
        cmake -B build -G Ninja \
          -DCMAKE_BUILD_TYPE=${{ matrix.build_type }} \
          -DDRYAD_BUILD_TESTS=ON \
          -DDRYAD_ENABLE_ASAN=${{ matrix.build_type == 'Debug' && 'ON' || 'OFF' }}
    
    - name: Build
      run: cmake --build build
    
    - name: Run tests
      run: cd build && ctest --output-on-failure
    
    - name: Upload artifacts
      uses: actions/upload-artifact@v3
      if: matrix.build_type == 'Release'
      with:
        name: dryad-${{ matrix.os }}
        path: build/bin/dryad

  coverage:
    name: Code Coverage
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install dependencies
      run: |
        sudo apt-get update
        sudo apt-get install -y cmake g++ libgtest-dev lcov
    
    - name: Configure with coverage
      run: |
        cmake -B build \
          -DCMAKE_BUILD_TYPE=Debug \
          -DDRYAD_BUILD_TESTS=ON \
          -DDRYAD_ENABLE_COVERAGE=ON
    
    - name: Build and test
      run: |
        cmake --build build
        cd build && ctest
    
    - name: Generate coverage report
      run: |
        lcov --capture --directory build --output-file coverage.info
        lcov --remove coverage.info '/usr/*' --output-file coverage.info
        lcov --list coverage.info
    
    - name: Upload to Codecov
      uses: codecov/codecov-action@v3
      with:
        files: ./coverage.info
        fail_ci_if_error: false
EOF

echo "✅ CI/CD configured"
```

### Step 9: Commit and Push (5 minutes)

```bash
# Add all files
git add .

# Commit
git commit -m "Phase 0: Foundation complete

- CMake build system configured
- Directory structure created
- Sample test passing
- CI/CD pipeline set up
- Ready for Phase 1 (Lexer & Parser)"

# Create develop branch
git checkout -b develop

# Push (after setting up remote)
# git remote add origin https://github.com/yourusername/dryad-cpp.git
# git push -u origin main
# git push -u origin develop

echo "✅ Phase 0 complete!"
```

---

## ✅ Phase 0 Checklist

Verify you've completed all tasks:

- [x] Repository initialized
- [x] .gitignore created
- [x] LICENSE added (MIT)
- [x] README.md created
- [x] Directory structure set up
- [x] Root CMakeLists.txt created
- [x] Subdirectory CMakeLists.txt created
- [x] Placeholder CLI (main.cpp)
- [x] Test infrastructure set up
- [x] Sample test passing
- [x] CI/CD pipeline configured
- [x] First build successful
- [x] Initial commit made

**If all checked, Phase 0 is DONE! 🎉**

---

## 🚀 Next Steps: Phase 1 (Lexer & Parser)

Now that foundation is complete, proceed to:

1. **Implement Value type** (see REWORK_OVERVIEW.md Section 5.1, Task 0.5)
2. **Create Token definitions** (see REWORK_OVERVIEW.md Section 5.2, Task 1.1)
3. **Build Lexer** (see REWORK_OVERVIEW.md Section 5.2, Task 1.2)
4. **Build Parser** (see REWORK_OVERVIEW.md Section 5.2, Task 1.3)

Follow the detailed task breakdown in `TASK_CHECKLIST.md`

---

## 🛠️ Development Workflow

### Daily Development Cycle

```bash
# 1. Pull latest changes
git checkout develop
git pull

# 2. Create feature branch
git checkout -b feature/lexer

# 3. Make changes, write tests first (TDD)
# Edit files...

# 4. Build and test locally
cmake --build build
cd build && ctest

# 5. Run with AddressSanitizer
cmake -B build-asan -DDRYAD_ENABLE_ASAN=ON
cmake --build build-asan
cd build-asan && ctest

# 6. Format code
find src include -name '*.cpp' -o -name '*.hpp' | xargs clang-format -i

# 7. Commit
git add .
git commit -m "Implement lexer token recognition"

# 8. Push and create PR
git push -u origin feature/lexer
# Then create PR on GitHub
```

### Weekly Review

Every Friday:
```bash
# Generate progress report
cmake --build build
cd build && ctest
lcov --capture --directory . --output-file coverage.info
lcov --list coverage.info | grep "Total"

# Update TASK_CHECKLIST.md with completed tasks
# Update weekly progress in REWORK_OVERVIEW.md
```

---

## 📚 Documentation

### Key Documents

- **REWORK_OVERVIEW.md** — Complete implementation strategy
- **TASK_CHECKLIST.md** — Granular task tracking (262 tasks)
- **dryad_theoretical_foundation_v2.pdf** — Language specification
- **V2_REVISION_SUMMARY.md** — Architecture changes

### Code Documentation

Use Doxygen-style comments:

```cpp
/**
 * @brief Virtual Machine for executing Dryad code
 * 
 * The VM maintains execution state including stack, heap,
 * and instruction pointer. It can execute bytecode or interpret
 * AST directly.
 * 
 * @example
 * VM vm;
 * vm.load_script("print(42);");
 * vm.execute();
 */
class VM {
    // ...
};
```

---

## 🐛 Troubleshooting

### Build Fails: "GTest not found"

```bash
# Ubuntu/Debian
sudo apt-get install libgtest-dev
cd /usr/src/gtest
sudo cmake CMakeLists.txt
sudo make
sudo cp lib/*.a /usr/lib

# macOS
brew install googletest
```

### Build Fails: "C++20 not supported"

```bash
# Check compiler version
g++ --version  # Need GCC 10+
clang++ --version  # Need Clang 12+

# Upgrade if needed (Ubuntu)
sudo add-apt-repository ppa:ubuntu-toolchain-r/test
sudo apt update
sudo apt install g++-11

# Use newer compiler
cmake -B build -DCMAKE_CXX_COMPILER=g++-11
```

### Tests Fail: "AddressSanitizer: heap-use-after-free"

```bash
# This is expected during development
# Fix memory safety issues in code
# Common causes:
# - Dangling pointers
# - Use-after-free
# - Double-free
# - Buffer overflow

# Disable ASAN temporarily to see test logic errors
cmake -B build -DDRYAD_ENABLE_ASAN=OFF
```

---

## 💡 Tips for Success

### 1. TDD Discipline
**Always write tests first!**
```cpp
// WRONG: Write implementation first
void VM::execute() { /* implementation */ }

// RIGHT: Write test first
TEST(VM, ExecuteSimpleExpression) {
    VM vm;
    vm.load_script("1 + 1");
    Value result = vm.execute();
    EXPECT_EQ(result.as_number(), 2);
}
// Now implement to make test pass
```

### 2. Small Commits
**Commit after each small task:**
```bash
# Good commits
git commit -m "Add Token class with location tracking"
git commit -m "Implement number literal lexing"
git commit -m "Add lexer tests for keywords"

# Bad commits (too large)
git commit -m "Implement entire lexer"
git commit -m "Fix everything"
```

### 3. Incremental Building
**Don't try to build everything at once:**
```cpp
// WRONG: Implement all opcodes at once
enum class Opcode { /* 50 opcodes */ };
void execute() { switch(opcode) { /* 50 cases */ } }

// RIGHT: Implement incrementally
enum class Opcode { PUSH, POP, ADD };  // Start with 3
void execute() { 
    switch(opcode) {
        case PUSH: /* implement */
        case ADD: /* implement */
        // Add more later
    }
}
```

### 4. Use Branch Protection
**Never commit directly to main:**
```bash
# Set up in GitHub:
# Settings → Branches → Branch protection rules
# - Require pull request reviews
# - Require status checks (CI must pass)
# - Require up-to-date branches
```

### 5. Keep Documentation Updated
**Document as you code:**
```cpp
// Update comments immediately when changing code
class Lexer {
    /**
     * @brief Scans next token from source
     * 
     * Skips whitespace and comments, then identifies
     * the next meaningful token. Returns Eof token
     * when source is exhausted.
     * 
     * @return Next token from input stream
     */
    Token next_token();
};
```

---

## 🎓 Learning Resources

### C++20 Features to Use
- **Concepts**: Template constraints
- **Modules**: Faster compilation (future)
- **std::format**: Better string formatting
- **Ranges**: Functional programming
- **Coroutines**: For async/await implementation

### Recommended Reading
- "Crafting Interpreters" by Bob Nystrom
- "Engineering a Compiler" by Cooper & Torczon
- "Modern C++ Design" by Alexandrescu
- LLVM Documentation (for AOT phase)

---

## 🎉 Success!

**You've completed Phase 0!** The foundation is solid. Time to build the actual language.

**Next**: Start Phase 1 by implementing the Value type. See `REWORK_OVERVIEW.md` Section 5.1, Task 0.5 for detailed instructions.

**Remember**: 
- Test first, code second
- Small commits, frequent pushes
- Review TASK_CHECKLIST.md weekly
- Ask for help when stuck

**Good luck building Dryad v2.0!** 🚀

---

**Document Version**: 1.0  
**Last Updated**: May 27, 2026  
**Next Phase**: Phase 1 — Lexer & Parser
