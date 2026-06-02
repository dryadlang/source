import os
import subprocess
import shutil
from datetime import datetime
import hashlib

# Utility function to calculate SHA256 checksum of a file
def calculate_checksum(file_path):
    hash_sha256 = hashlib.sha256()
    with open(file_path, "rb") as f:
        for chunk in iter(lambda: f.read(4096), b""):
            hash_sha256.update(chunk)
    return hash_sha256.hexdigest()

def deploy_release(version="dev", create_zip=False, generate_checksums=False, output_dir="dryad_release"):
    """
    Automates the deployment process for releases, including generating checksums and optionally creating a zip.
    """
    print(f"[INFO] Deploying Dryad Release - Version: {version}")
    
    # Step 1: Build the project
    print("[INFO] Compiling optimized release version...")
    try:
        subprocess.run(["cargo", "build", "--release"], check=True, text=True)
    except subprocess.CalledProcessError:
        print("❌ Compilation failed!")
        return False

    # Step 2: Prepare versioned directory
    if version != "dev":
        output_dir = f"{output_dir}_v{version}"

    if os.path.exists(output_dir):
        shutil.rmtree(output_dir)
    os.makedirs(output_dir, exist_ok=True)
    print(f"📁 Creating release directory: {output_dir}")

    release_info = {
        "Version": version,
        "BuildDate": datetime.now().strftime("%Y-%m-%d %H:%M:%S"),
        "BuildMachine": os.environ.get("COMPUTERNAME", "Unknown"),
        "RustVersion": subprocess.run(["rustc", "--version"], capture_output=True, text=True).stdout.strip(),
        "Binaries": [],
        "Checksums": {}
    }

    # Step 3: Copy binaries
    binaries = [
        {"name": "dryad.exe", "description": "Dryad Language Interpreter", "essential": True},
        {"name": "oak.exe", "description": "Oak Development Tool", "essential": False},
        {"name": "benchmark.exe", "description": "Dryad Benchmark Suite", "essential": False}
    ]

    print("📦 Copying and verifying binaries...")
    for binary in binaries:
        source_path = os.path.join("target", "release", binary["name"])
        dest_path = os.path.join(output_dir, binary["name"])

        if os.path.isfile(source_path):
            shutil.copy2(source_path, dest_path)
            size_mb = os.path.getsize(dest_path) / (1024 * 1024)
            checksum = calculate_checksum(dest_path)

            binary_info = {
                "Name": binary["name"],
                "Description": binary["description"],
                "Size": f"{size_mb:.2f} MB",
                "Checksum": checksum
            }

            release_info["Binaries"].append(binary_info)
            release_info["Checksums"][binary["name"]] = checksum
            
            print(f"  ✅ {binary['name']} ({size_mb:.2f} MB)")
            if generate_checksums:
                print(f"     SHA256: {checksum}")
        else:
            if binary["essential"]:
                print(f"  ❌ ERROR: Essential binary {binary['name']} not found!")
                return False
            else:
                print(f"  ⚠️  Optional binary {binary['name']} not found")

    # Step 4: Copy documentation
    print("📚 Copying documentation...")
    docs = ["README.md", "benchmark.md", "DRYAD_ERROR_GUIDE.md", "BUILD_SCRIPTS_README.md"]
    for doc in docs:
        if os.path.isfile(doc):
            shutil.copy2(doc, os.path.join(output_dir, os.path.basename(doc)))
            print(f"  ✅ {doc} copied")
        else:
            print(f"  ⚠️  {doc} not found")

    # Step 5: Copy examples
    print("📂 Copying example Dryad files...")
    examples = [f for f in os.listdir(".") if f.endswith(".dryad")]
    for example in examples:
        shutil.copy2(example, os.path.join(output_dir, example))
        print(f"  ✅ {example} copied")

    # Step 6: Generate version info
    version_info_path = os.path.join(output_dir, "VERSION.md")
    version_info = f"""
# Dryad Language Release v{version}

**Release Date:** {release_info['BuildDate']}
**Build Machine:** {release_info['BuildMachine']}
**Rust Version:** {release_info['RustVersion']}

## Included Binaries
"""

    for binary in release_info["Binaries"]:
        version_info += f"### {binary['Name']}\n"
        version_info += f"- **Description:** {binary['Description']}\n"
        version_info += f"- **Size:** {binary['Size']}\n"
        if generate_checksums:
            version_info += f"- **SHA256:** `{binary['Checksum']}`\n"
        version_info += "\n"

    version_info += """
## How to Use

1. Extract all files in the directory.
2. Run in terminal/PowerShell:
   ./dryad.exe run example.dryad

---
*Release automatically generated on {release_info['BuildDate']}*
"""

    with open(version_info_path, "w", encoding="utf-8") as file:
        file.write(version_info)

    print(f"  ✅ VERSION.md generated")

    # Step 7: Generate checksum file if required
    if generate_checksums:
        checksum_file_path = os.path.join(output_dir, "CHECKSUMS.txt")
        with open(checksum_file_path, "w", encoding="utf-8") as file:
            file.write(f"# Checksums SHA256 - Dryad v{version}\n\n")
            for binary in release_info["Binaries"]:
                file.write(f"{binary['Checksum']}  {binary['Name']}\n")

        print(f"  ✅ CHECKSUMS.txt generated")

    # Step 8: Create zip file if required
    if create_zip:
        zip_name = f"dryad_v{version}_{datetime.now().strftime('%Y-%m-%d')}.zip"
        shutil.make_archive(zip_name.replace(".zip", ""), "zip", output_dir)
        zip_size = os.path.getsize(zip_name) / (1024 * 1024)
        print(f"  ✅ {zip_name} created ({zip_size:.2f} MB)")

    print(f"🎉 Deployment for version {version} completed successfully!")

def main():
    import argparse

    parser = argparse.ArgumentParser(description="Automated release deployment script for Dryad")
    parser.add_argument("--version", type=str, default="dev", help="Version tag for the release")
    parser.add_argument("--zip", action="store_true", help="Create a zip file for the release")
    parser.add_argument("--checksums", action="store_true", help="Generate a checksums file")
    parser.add_argument("--output", type=str, default="dryad_release", help="Output directory for the release")

    args = parser.parse_args()

    deploy_release(
        version=args.version,
        create_zip=args.zip,
        generate_checksums=args.checksums,
        output_dir=args.output
    )

if __name__ == "__main__":
    main()