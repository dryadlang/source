import os
import subprocess

def test_binary(release_dir, name, args=None):
    if args is None:
        args = []

        print(f"[INFO] Testing {name}...")
    binary_path = os.path.join(release_dir, name)

    if not os.path.isfile(binary_path):
        print(f"  ❌ {name} not found!")
        return False

    try:
        result = subprocess.run([binary_path] + args, capture_output=True, text=True)
        if result.returncode == 0 or name == "dryad.exe":
            print(f"  ✅ {name} executed successfully")
            return True
        else:
            print(f"  ❌ {name} failed (exit code: {result.returncode})")
            print(f"     Output: {result.stdout.strip()}")
            return False
    except Exception as e:
        print(f"  ❌ Error executing {name}: {e}")
        return False

def test_dryad_file(release_dir, file_name):
    print(f"📝 Testing Dryad file: {file_name}...")
    file_path = os.path.join(release_dir, file_name)

    if not os.path.isfile(file_path):
        print(f"  ⚠️  File {file_name} not found (optional)")
        return None

    dryad_path = os.path.join(release_dir, "dryad.exe")

    try:
        result = subprocess.run([dryad_path, "run", file_path], capture_output=True, text=True)
        exit_code = result.returncode

        print(f"  ✅ {file_name} processed (exit code: {exit_code})")
        if result.stdout:
            print(f"     Output: {result.stdout.strip()}")
        return True
    except Exception as e:
        print(f"  ❌ Error processing {file_name}: {e}")
        return False

def main():
    release_dir = "dryad_release"

    print("[INFO] Testing Dryad Release")
    print("============================")

    if not os.path.isdir(release_dir):
        print(f"❌ Directory {release_dir} not found!")
        print("💡 Please run: python update_release.py")
        return

    tests_passed = 0
    tests_failed = 0

    # Test critical binaries
    print("\n[INFO] Testing Binaries")
    print("===================")
    binaries = [
        {"name": "dryad.exe", "args": ["--help"], "essential": True},
        {"name": "oak.exe", "args": [], "essential": False},
        {"name": "benchmark.exe", "args": [], "essential": False}
    ]

    for binary in binaries:
        if test_binary(release_dir, binary["name"], binary.get("args")):
            tests_passed += 1
        else:
            tests_failed += 1
            if binary["essential"]:
                print("🚨 CRITICAL: Essential binary failed!")

    # Check documentation
    print("\n[INFO] Verifying Documentation")
    print("==========================")
    docs = ["README.md", "VERSION.md", "BUILD_INFO.md"]

    for doc in docs:
        doc_path = os.path.join(release_dir, doc)
        if os.path.isfile(doc_path):
            size = os.path.getsize(doc_path) / 1024
            print(f"  ✅ {doc} found ({size:.1f}KB)")
            tests_passed += 1
        else:
            print(f"  ⚠️  {doc} not found (optional)")

    # Test example Dryad files
    print("\n[INFO] Testing Example Dryad Files")
    print("===============================")
    dryad_files = [f for f in os.listdir(release_dir) if f.endswith(".dryad")]

    if not dryad_files:
        print("  ⚠️  No .dryad files found for testing")
    else:
        for file in dryad_files:
            result = test_dryad_file(release_dir, file)
            if result is True:
                tests_passed += 1
            elif result is False:
                tests_failed += 1

    # Final Report
    print("\n[INFO] Test Results")
    print("====================")
    total_tests = tests_passed + tests_failed
    success_rate = round((tests_passed / total_tests) * 100, 1) if total_tests > 0 else 0

    print(f"Tests run: {total_tests}")
    print(f"Success: {tests_passed}")
    print(f"Failures: {tests_failed}")
    print(f"Success rate: {success_rate}%")

    if tests_failed == 0:
        print("\n[INFO] All tests passed! Release is ready for distribution.")
    elif success_rate >= 80:
        print("\n[WARNING] Release is functional with minor issues.")
    else:
        print("\n❌ Release has major issues. Investigation recommended.")

if __name__ == "__main__":
    main()