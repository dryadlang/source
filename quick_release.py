import os
import subprocess
import shutil

def quick_build():
    """
    Quickly builds the project in debug mode and copies the binaries to dryad_release.
    """
    print("[INFO] Quick build (debug mode)...")

    # Step 1: Build the project in debug mode
    try:
        subprocess.run(["cargo", "build"], check=True, text=True)
        print("✅ Debug build completed successfully!")
    except subprocess.CalledProcessError:
        print("❌ Debug build failed!")
        return False

    # Step 2: Copy binaries
    binaries = ["dryad.exe", "oak.exe"]
    src_dir = "target/debug"
    dest_dir = "dryad_release"

    os.makedirs(dest_dir, exist_ok=True)

    for binary in binaries:
        src_path = os.path.join(src_dir, binary)
        dest_path = os.path.join(dest_dir, binary)

        if os.path.exists(src_path):
            shutil.copy2(src_path, dest_path)
            size_mb = os.path.getsize(dest_path) / (1024 * 1024)
            print(f"[INFO] {binary} updated ({size_mb:.2f} MB)")
        else:
            print(f"[WARNING] {binary} not found: {src_path}")

    print("[INFO] Quick build completed successfully!")
    return True

if __name__ == "__main__":
    quick_build()