// tests/unit/vfs_test.cpp
// Virtual File System tests

#include <gtest/gtest.h>
#include "runtime/interpreter.hpp"
#include <memory>
#include <string>

class VFSTest : public ::testing::Test {
protected:
    std::unique_ptr<Dryad::Interpreter> interpreter;
    
    void SetUp() override {
        interpreter = std::make_unique<Dryad::Interpreter>();
    }
};

// Test VFS interface definition
TEST_F(VFSTest, BackendInterfaceDefined) {
    // Test that FileSystemBackend protocol is defined
    auto result = interpreter->Execute(R"(
        import { FileSystemBackend } from "@std/io/vfs";
        FileSystemBackend()
    )");
    
    EXPECT_TRUE(result.IsString());
    EXPECT_EQ(result.AsString(), "FileSystemBackend protocol");
}

// Test VFS creation with MemoryBackend
TEST_F(VFSTest, VFSCreationWithMemoryBackend) {
    auto result = interpreter->Execute(R"(
        import { VFS, MemoryBackend } from "@std/io/vfs";
        let backend = MemoryBackend();
        let vfs = VFS(backend);
        vfs != null
    )");
    
    EXPECT_TRUE(result.IsBoolean());
    EXPECT_TRUE(result.AsBoolean());
}

// Test basic write/read through VFS
TEST_F(VFSTest, VFSBasicWriteRead) {
    auto result = interpreter->Execute(R"(
        import { VFS, MemoryBackend, VFS_writeFile, VFS_readFile } from "@std/io/vfs";
        let backend = MemoryBackend();
        let vfs = VFS(backend);
        
        VFS_writeFile(vfs, "/test.txt", "Hello");
        let content = VFS_readFile(vfs, "/test.txt");
        content == "Hello"
    )");
    
    EXPECT_TRUE(result.IsBoolean());
    EXPECT_TRUE(result.AsBoolean());
}

// Test file existence check
TEST_F(VFSTest, VFSExistsCheck) {
    auto result = interpreter->Execute(R"(
        import { VFS, MemoryBackend, VFS_writeFile, VFS_exists } from "@std/io/vfs";
        let backend = MemoryBackend();
        let vfs = VFS(backend);
        
        let exists1 = VFS_exists(vfs, "/missing.txt");
        VFS_writeFile(vfs, "/test.txt", "data");
        let exists2 = VFS_exists(vfs, "/test.txt");
        
        !exists1 && exists2
    )");
    
    EXPECT_TRUE(result.IsBoolean());
    EXPECT_TRUE(result.AsBoolean());
}

// Test file removal
TEST_F(VFSTest, VFSRemoveFile) {
    auto result = interpreter->Execute(R"(
        import { VFS, MemoryBackend, VFS_writeFile, VFS_removeFile, VFS_exists } from "@std/io/vfs";
        let backend = MemoryBackend();
        let vfs = VFS(backend);
        
        VFS_writeFile(vfs, "/temp.txt", "temporary");
        let before = VFS_exists(vfs, "/temp.txt");
        VFS_removeFile(vfs, "/temp.txt");
        let after = VFS_exists(vfs, "/temp.txt");
        
        before && !after
    )");
    
    EXPECT_TRUE(result.IsBoolean());
    EXPECT_TRUE(result.AsBoolean());
}

// Test VFS disposal
TEST_F(VFSTest, VFSDisposal) {
    auto result = interpreter->Execute(R"(
        import { VFS, MemoryBackend, VFS_dispose } from "@std/io/vfs";
        let backend = MemoryBackend();
        let vfs = VFS(backend);
        
        VFS_dispose(vfs);
        
        // After disposal, vfs["disposed"] should be true
        vfs["disposed"] == true
    )");
    
    EXPECT_TRUE(result.IsBoolean());
    EXPECT_TRUE(result.AsBoolean());
}

// Test multiple operations
TEST_F(VFSTest, MultipleFileOperations) {
    auto result = interpreter->Execute(R"(
        import { VFS, MemoryBackend, VFS_writeFile, VFS_readFile } from "@std/io/vfs";
        let backend = MemoryBackend();
        let vfs = VFS(backend);
        
        VFS_writeFile(vfs, "/file1.txt", "content1");
        VFS_writeFile(vfs, "/file2.txt", "content2");
        VFS_writeFile(vfs, "/file3.txt", "content3");
        
        let c1 = VFS_readFile(vfs, "/file1.txt");
        let c2 = VFS_readFile(vfs, "/file2.txt");
        let c3 = VFS_readFile(vfs, "/file3.txt");
        
        c1 == "content1" && c2 == "content2" && c3 == "content3"
    )");
    
    EXPECT_TRUE(result.IsBoolean());
    EXPECT_TRUE(result.AsBoolean());
}

// Test MemoryBackend independence
TEST_F(VFSTest, MemoryBackendIndependence) {
    auto result = interpreter->Execute(R"(
        import { VFS, MemoryBackend, VFS_writeFile, VFS_readFile, VFS_exists } from "@std/io/vfs";
        
        // Create two separate backends
        let backend1 = MemoryBackend();
        let vfs1 = VFS(backend1);
        
        let backend2 = MemoryBackend();
        let vfs2 = VFS(backend2);
        
        // Write different data to each
        VFS_writeFile(vfs1, "/data.txt", "vfs1");
        VFS_writeFile(vfs2, "/data.txt", "vfs2");
        
        let data1 = VFS_readFile(vfs1, "/data.txt");
        let data2 = VFS_readFile(vfs2, "/data.txt");
        
        data1 == "vfs1" && data2 == "vfs2"
    )");
    
    EXPECT_TRUE(result.IsBoolean());
    EXPECT_TRUE(result.AsBoolean());
}

// Test NativeBackend creation
TEST_F(VFSTest, NativeBackendCreation) {
    auto result = interpreter->Execute(R"(
        import { NativeBackend } from "@std/io/vfs";
        let backend = NativeBackend();
        backend != null
    )");
    
    EXPECT_TRUE(result.IsBoolean());
    EXPECT_TRUE(result.AsBoolean());
}
