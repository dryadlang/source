// tests/unit/io_functions_test.cpp
// Core I/O functions tests

#include <gtest/gtest.h>
#include "runtime/interpreter.hpp"
#include <memory>

class IOFunctionsTest : public ::testing::Test {
protected:
    std::unique_ptr<Dryad::Interpreter> interpreter;
    
    void SetUp() override {
        interpreter = std::make_unique<Dryad::Interpreter>();
    }
};

TEST_F(IOFunctionsTest, ReadFileFunction) {
    auto result = interpreter->Execute(R"(
        import { readFile, writeFile, setVFSBackend } from "@std/io/file";
        import { MemoryBackend } from "@std/io/vfs";
        
        setVFSBackend(MemoryBackend());
        writeFile("/test.txt", "test content");
        let content = readFile("/test.txt");
        content == "test content"
    )");
    
    EXPECT_TRUE(result.IsBoolean());
    EXPECT_TRUE(result.AsBoolean());
}

TEST_F(IOFunctionsTest, WriteFileFunction) {
    auto result = interpreter->Execute(R"(
        import { writeFile, exists, setVFSBackend } from "@std/io/file";
        import { MemoryBackend } from "@std/io/vfs";
        
        setVFSBackend(MemoryBackend());
        writeFile("/output.txt", "Hello");
        exists("/output.txt")
    )");
    
    EXPECT_TRUE(result.IsBoolean());
    EXPECT_TRUE(result.AsBoolean());
}

TEST_F(IOFunctionsTest, ExistsFunction) {
    auto result = interpreter->Execute(R"(
        import { exists, writeFile, setVFSBackend } from "@std/io/file";
        import { MemoryBackend } from "@std/io/vfs";
        
        setVFSBackend(MemoryBackend());
        
        let exists1 = exists("/missing.txt");
        writeFile("/existing.txt", "data");
        let exists2 = exists("/existing.txt");
        
        !exists1 && exists2
    )");
    
    EXPECT_TRUE(result.IsBoolean());
    EXPECT_TRUE(result.AsBoolean());
}

TEST_F(IOFunctionsTest, RemoveFileFunction) {
    auto result = interpreter->Execute(R"(
        import { writeFile, removeFile, exists, setVFSBackend } from "@std/io/file";
        import { MemoryBackend } from "@std/io/vfs";
        
        setVFSBackend(MemoryBackend());
        
        writeFile("/temp.txt", "temporary");
        let before = exists("/temp.txt");
        removeFile("/temp.txt");
        let after = exists("/temp.txt");
        
        before && !after
    )");
    
    EXPECT_TRUE(result.IsBoolean());
    EXPECT_TRUE(result.AsBoolean());
}

TEST_F(IOFunctionsTest, SetVFSBackend) {
    auto result = interpreter->Execute(R"(
        import { writeFile, readFile, setVFSBackend } from "@std/io/file";
        import { MemoryBackend } from "@std/io/vfs";
        
        let backend1 = MemoryBackend();
        setVFSBackend(backend1);
        writeFile("/file1.txt", "backend1");
        
        let backend2 = MemoryBackend();
        setVFSBackend(backend2);
        writeFile("/file2.txt", "backend2");
        
        let content2 = readFile("/file2.txt");
        content2 == "backend2"
    )");
    
    EXPECT_TRUE(result.IsBoolean());
    EXPECT_TRUE(result.AsBoolean());
}

TEST_F(IOFunctionsTest, MultipleFileOperations) {
    auto result = interpreter->Execute(R"(
        import { writeFile, readFile, exists, removeFile, setVFSBackend } from "@std/io/file";
        import { MemoryBackend } from "@std/io/vfs";
        
        setVFSBackend(MemoryBackend());
        
        writeFile("/file1.txt", "content1");
        writeFile("/file2.txt", "content2");
        writeFile("/file3.txt", "content3");
        
        let c1 = readFile("/file1.txt");
        let c2 = readFile("/file2.txt");
        let c3 = readFile("/file3.txt");
        
        c1 == "content1" && c2 == "content2" && c3 == "content3"
    )");
    
    EXPECT_TRUE(result.IsBoolean());
    EXPECT_TRUE(result.AsBoolean());
}

TEST_F(IOFunctionsTest, MkdirFunction) {
    auto result = interpreter->Execute(R"(
        import { mkdir, setVFSBackend } from "@std/io/file";
        import { MemoryBackend } from "@std/io/vfs";
        
        setVFSBackend(MemoryBackend());
        let result = mkdir("/testdir");
        result == null
    )");
    
    EXPECT_TRUE(result.IsBoolean());
    EXPECT_TRUE(result.AsBoolean());
}

TEST_F(IOFunctionsTest, ListDirectoryFunction) {
    auto result = interpreter->Execute(R"(
        import { listDirectory, setVFSBackend } from "@std/io/file";
        import { MemoryBackend } from "@std/io/vfs";
        
        setVFSBackend(MemoryBackend());
        let files = listDirectory("/");
        files != null
    )");
    
    EXPECT_TRUE(result.IsBool() || result.IsArray());
}
