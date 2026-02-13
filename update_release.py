import os
import subprocess
import shutil

def compile_project():
    """
    Compile the project in release mode.
    """
    print("[INFO] Compiling project in release mode...")
    try:
        subprocess.run(["cargo", "build", "--release"], check=True, text=True)
        print("[INFO] Compilation completed successfully!")
    except subprocess.CalledProcessError:
        print("❌ Compilation failed!")
        return False
    return True

def update_release_folder():
    """
    Updates the dryad_release folder with binaries and documentation.
    """
    source_dir = "target/release"
    dest_dir = "dryad_release"

    # Create destination directory if it doesn't exist
    os.makedirs(dest_dir, exist_ok=True)

    # Copy main binaries
    binaries = ["dryad.exe", "oak.exe", "benchmark.exe"]
    print("[INFO] Copying binaries...")
    for binary in binaries:
        source_path = os.path.join(source_dir, binary)
        dest_path = os.path.join(dest_dir, binary)

        if os.path.isfile(source_path):
            shutil.copy2(source_path, dest_path)
            size_mb = os.path.getsize(dest_path) / (1024 * 1024)
            print(f"✅ {binary} copied ({size_mb:.2f} MB)")
        else:
            print(f"[WARNING] {binary} not found: {source_path}")

    # Copy important documentation
    docs = ["README.md", "benchmark.md", "DRYAD_ERROR_GUIDE.md"]
    print("[INFO] Copying important documentation...")
    for doc in docs:
        if os.path.isfile(doc):
            shutil.copy2(doc, os.path.join(dest_dir, os.path.basename(doc)))
            print(f"✅ {doc} copied")
        else:
            print(f"⚠️  {doc} not found")

    # Copy test Dryad files
    print("[INFO] Copying test Dryad files...")
    dryad_files = [f for f in os.listdir(".") if f.endswith(".dryad")]
    for file in dryad_files:
        shutil.copy2(file, os.path.join(dest_dir, file))
        print(f"✅ {file} copied")

    # List the final content of the release folder
    print("\n📂 Contents of dryad_release folder:")
    for item in os.listdir(dest_dir):
        size = os.path.getsize(os.path.join(dest_dir, item)) / 1024
        print(f"  - {item} ({size:.2f} KB)")
    
    print("[INFO] Release folder updated successfully!")

if __name__ == "__main__":
    if compile_project():
        update_release_folder()
    else:
        print("❌ Release update failed.")