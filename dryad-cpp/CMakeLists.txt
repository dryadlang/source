cmake_minimum_required(VERSION 3.15)
project(Dryad VERSION 2.0.0 LANGUAGES CXX)

# =============================================================================
# Project Options
# =============================================================================
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

option(DRYAD_BUILD_TESTS "Build tests" ON)
option(DRYAD_BUILD_BENCHMARKS "Build benchmarks" OFF)
option(DRYAD_ENABLE_COVERAGE "Enable code coverage" OFF)
option(DRYAD_ENABLE_SANITIZERS "Enable sanitizers (ASAN, UBSAN)" OFF)

# =============================================================================
# Compiler Flags
# =============================================================================
if(MSVC)
    add_compile_options(/W4 /WX)
else()
    add_compile_options(
        -Wall -Wextra -Werror
        -Wno-unused-parameter
        -Wno-missing-field-initializers
    )
    
    if(CMAKE_BUILD_TYPE STREQUAL "Debug")
        add_compile_options(-g -O0)
    elseif(CMAKE_BUILD_TYPE STREQUAL "Release")
        add_compile_options(-O3 -DNDEBUG)
    endif()
endif()

# Sanitizers
if(DRYAD_ENABLE_SANITIZERS AND NOT MSVC)
    add_compile_options(-fsanitize=address,undefined)
    add_link_options(-fsanitize=address,undefined)
endif()

# Coverage
if(DRYAD_ENABLE_COVERAGE AND NOT MSVC)
    add_compile_options(--coverage)
    add_link_options(--coverage)
endif()

# =============================================================================
# Include Directories
# =============================================================================
include_directories(${PROJECT_SOURCE_DIR}/include)

# =============================================================================
# Core Libraries
# =============================================================================

# Common utilities
add_library(dryad_common STATIC
    src/common/utils.cpp
)
target_include_directories(dryad_common PUBLIC include)

# Runtime library
add_library(dryad_runtime STATIC
    src/runtime/value.cpp
    src/runtime/gc.cpp
    src/runtime/intrinsics.cpp
    src/runtime/intrinsics_registry.cpp
    src/runtime/intrinsics_network.cpp
    src/runtime/intrinsics_filesystem.cpp
    src/runtime/intrinsics_process.cpp
    src/runtime/module_loader.cpp
    src/runtime/environment.cpp
    src/runtime/function.cpp
    src/runtime/class.cpp
)
target_include_directories(dryad_runtime PUBLIC include)
target_link_libraries(dryad_runtime PUBLIC dryad_common)

# Compiler library
add_library(dryad_compiler STATIC
    src/compiler/token.cpp
    src/compiler/lexer.cpp
    src/compiler/parser.cpp
    src/compiler/ast.cpp
    src/compiler/interpreter.cpp
    src/compiler/evaluator.cpp
    src/compiler/statement_executor.cpp
    src/compiler/validator.cpp
    src/compiler/semantic_analyzer.cpp
    src/compiler/diagnostic.cpp
)
target_include_directories(dryad_compiler PUBLIC include)
target_link_libraries(dryad_compiler PUBLIC dryad_runtime)

# =============================================================================
# Executables
# =============================================================================

# Main CLI
add_executable(dryad
    src/tools/main.cpp
    src/tools/repl.cpp
)
target_link_libraries(dryad PRIVATE dryad_compiler dryad_runtime)
target_compile_definitions(dryad PRIVATE
    DRYAD_STDLIB_DIR="${PROJECT_SOURCE_DIR}/stdlib"
)

# Set output directory
set_target_properties(dryad PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin
)

# =============================================================================
# Testing
# =============================================================================
if(DRYAD_BUILD_TESTS)
    enable_testing()
    
    # Download and configure Google Test
    include(FetchContent)
    FetchContent_Declare(
        googletest
        GIT_REPOSITORY https://github.com/google/googletest.git
        GIT_TAG v1.14.0
    )
    # For Windows: Prevent overriding the parent project's compiler/linker settings
    set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
    FetchContent_MakeAvailable(googletest)
    
    include(GoogleTest)
    
    # Unit tests
    add_executable(unit_tests
        tests/unit/sample_test.cpp
        tests/unit/value_test.cpp
        tests/unit/lexer_test.cpp
        tests/unit/parser_test.cpp
        tests/unit/interpreter_test.cpp
        tests/unit/intrinsics_test.cpp
        tests/unit/internal_keyword_test.cpp
        tests/unit/exceptions_test.cpp
        tests/unit/disposable_test.cpp
        tests/unit/array_test.cpp
        tests/unit/object_test.cpp
        tests/unit/class_test.cpp
        tests/unit/validator_test.cpp
        tests/unit/poo_foundation_test.cpp
        tests/unit/lsp_foundation_test.cpp
        tests/unit/buffer_intrinsics_test.cpp
        tests/unit/buffer_test.cpp
        tests/unit/vfs_test.cpp
        tests/unit/io_functions_test.cpp
        tests/unit/filehandle_test.cpp
        tests/unit/io_errors_test.cpp
        tests/unit/io_concurrency_test.cpp
        tests/unit/io_integration_test.cpp
        tests/unit/import_test.cpp
    )
    target_link_libraries(unit_tests PRIVATE 
        gtest_main
        dryad_runtime
        dryad_compiler
    )
    # Allow tests to locate stdlib source files (loaded at runtime by interpreter)
    target_compile_definitions(unit_tests PRIVATE
        DRYAD_STDLIB_DIR="${PROJECT_SOURCE_DIR}/stdlib"
        DRYAD_TEST_DIR="${PROJECT_SOURCE_DIR}/tests"
    )
    gtest_discover_tests(unit_tests)
    
    # Integration tests
    add_executable(integration_tests
        tests/integration/sample_integration_test.cpp
        tests/integration/intrinsics_integration_test.cpp
        tests/integration/buffer_workflow_test.cpp
        tests/integration/vfs_workflow_test.cpp
        tests/integration/core_foundation_test.cpp
    )
    target_link_libraries(integration_tests PRIVATE
        gtest_main
        dryad_compiler
        dryad_runtime
    )
    target_compile_definitions(integration_tests PRIVATE
        DRYAD_STDLIB_DIR="${PROJECT_SOURCE_DIR}/stdlib"
        DRYAD_EXAMPLE_DIR="${PROJECT_SOURCE_DIR}/examples"
    )
    gtest_discover_tests(integration_tests)
endif()

# =============================================================================
# Benchmarks
# =============================================================================
if(DRYAD_BUILD_BENCHMARKS)
    # TODO: Add Google Benchmark integration
    message(STATUS "Benchmarks enabled (not yet implemented)")
endif()

# =============================================================================
# Installation
# =============================================================================
install(TARGETS dryad RUNTIME DESTINATION bin)
install(DIRECTORY stdlib/ DESTINATION share/dryad/stdlib)
install(FILES LICENSE README.md DESTINATION share/doc/dryad)

# =============================================================================
# Summary
# =============================================================================
message(STATUS "")
message(STATUS "Dryad Configuration Summary:")
message(STATUS "  Version:            ${PROJECT_VERSION}")
message(STATUS "  Build Type:         ${CMAKE_BUILD_TYPE}")
message(STATUS "  C++ Standard:       ${CMAKE_CXX_STANDARD}")
message(STATUS "  Build Tests:        ${DRYAD_BUILD_TESTS}")
message(STATUS "  Build Benchmarks:   ${DRYAD_BUILD_BENCHMARKS}")
message(STATUS "  Enable Coverage:    ${DRYAD_ENABLE_COVERAGE}")
message(STATUS "  Enable Sanitizers:  ${DRYAD_ENABLE_SANITIZERS}")
message(STATUS "")
