#!/usr/bin/env bash
# Run all Dryad examples one by one with timing for benchmark.
# Usage: ./examples/run_benchmark.sh [--verbose] [--include-net] [--include-optional] [--triple]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
BINARY="${DRYAD_BIN:-$ROOT_DIR/build/bin/dryad}"
STDLIB="${DRYAD_STDLIB_DIR:-$ROOT_DIR/stdlib}"
MANIFEST="$SCRIPT_DIR/manifest.json"
TMPDIR="${TMPDIR:-/tmp}"
RUN_TIMEOUT="${DRYAD_RUN_TIMEOUT:-30}"
COMPILE_TIMEOUT="${DRYAD_COMPILE_TIMEOUT:-120}"

VERBOSE=0
INCLUDE_NET=0
INCLUDE_OPTIONAL=0
TRIPLE=0

for arg in "$@"; do
    case "$arg" in
        --verbose|-v) VERBOSE=1 ;;
        --include-net) INCLUDE_NET=1 ;;
        --include-optional) INCLUDE_OPTIONAL=1 ;;
        --triple) TRIPLE=1 ;;
        --help|-h)
            echo "Usage: $0 [--verbose] [--include-net] [--include-optional] [--triple]"
            echo ""
            echo "  --triple   Run each example in interpreter, bytecode VM, and AOT compile modes"
            exit 0
            ;;
    esac
done

if [[ ! -x "$BINARY" ]]; then
    echo "ERROR: dryad binary not found at $BINARY"
    echo "Build first: cmake -B build && cmake --build build"
    exit 1
fi

export DRYAD_STDLIB_DIR="$STDLIB"

PASS=0
FAIL=0
SKIP=0
TOTAL_MS=0

# Triple-mode counters (cells = example × mode)
TRIPLE_PASS=0
TRIPLE_FAIL=0
TRIPLE_SKIP=0
TRIPLE_TOTAL=0

print_header() {
    if [[ $TRIPLE -eq 1 ]]; then
        printf "\n%-36s %6s %6s %6s\n" "EXAMPLE" "INTERP" "VM" "AOT"
        printf "%-36s %6s %6s %6s\n" "------------------------------------" "------" "------" "------"
    else
        printf "\n%-50s %8s %6s\n" "EXAMPLE" "TIME" "STATUS"
        printf "%-50s %8s %6s\n" "--------------------------------------------------" "--------" "------"
    fi
}

apply_example_env() {
    local env_json="$1"
    [[ -z "$env_json" || "$env_json" == "{}" ]] && return
    while IFS='=' read -r key value; do
        [[ -n "$key" ]] && export "$key=$value"
    done < <(python3 -c "import json,sys,os; d=json.loads(sys.argv[1]); root=sys.argv[2]; [print(f'{k}={os.path.abspath(os.path.join(root,v))}' if k=='DRYAD_LIBRARY_PATH' and not os.path.isabs(v) else f'{k}={v}') for k,v in d.items()]" "$env_json" "$ROOT_DIR")
}

run_mode() {
    local mode="$1"
    local flags="$2"
    local full="$3"
    local out_bin="${4:-}"
    local env_json="${5:-}"

    apply_example_env "$env_json"

    local cmd=("$BINARY")
    if [[ -n "$flags" ]]; then
        # shellcheck disable=SC2206
        cmd+=($flags)
    fi
    cmd+=("$full")

    if [[ "$mode" == "aot" ]]; then
        local ll_out="$out_bin.ll"
        if timeout "$COMPILE_TIMEOUT" "$BINARY" compile "$full" --aot -o "$ll_out" >/dev/null 2>&1; then
            rm -f "$ll_out"
            return 0
        fi
        return 1
    fi

    if timeout "$RUN_TIMEOUT" "${cmd[@]}" >/dev/null 2>&1; then
        return 0
    fi
    return 1
}

run_example_triple() {
    local id="$1"
    local path="$2"
    local flags="${3:-}"
    local optional="${4:-false}"
    local env_json="${5:-}"
    local full="$SCRIPT_DIR/$path"

    TRIPLE_TOTAL=$((TRIPLE_TOTAL + 3))

    if [[ ! -f "$full" ]]; then
        printf "%-36s %6s %6s %6s\n" "$id" "MISS" "MISS" "MISS"
        TRIPLE_FAIL=$((TRIPLE_FAIL + 3))
        FAIL=$((FAIL + 1))
        return
    fi

    if [[ "$optional" == "true" && "$INCLUDE_OPTIONAL" -eq 0 ]]; then
        printf "%-36s %6s %6s %6s\n" "$id" "SKIP" "SKIP" "SKIP"
        TRIPLE_SKIP=$((TRIPLE_SKIP + 3))
        SKIP=$((SKIP + 1))
        return
    fi

    if [[ "$flags" == *"--allow-net"* && "$INCLUDE_NET" -eq 0 ]]; then
        printf "%-36s %6s %6s %6s\n" "$id" "SKIP" "SKIP" "SKIP"
        TRIPLE_SKIP=$((TRIPLE_SKIP + 3))
        SKIP=$((SKIP + 1))
        return
    fi

    local start end elapsed_ms
    local s_interp="FAIL" s_vm="FAIL" s_aot="FAIL"
    start=$(date +%s%3N)

    if run_mode "interp" "$flags" "$full" "" "$env_json"; then
        s_interp="PASS"
        TRIPLE_PASS=$((TRIPLE_PASS + 1))
    else
        TRIPLE_FAIL=$((TRIPLE_FAIL + 1))
        FAIL=$((FAIL + 1))
        if [[ $VERBOSE -eq 1 ]]; then
            echo "--- $id [interp] failed ---"
            "$BINARY" $flags "$full" 2>&1 | head -5 || true
        fi
    fi

    local vm_flags="--vm"
    if [[ -n "$flags" ]]; then
        vm_flags="--vm $flags"
    fi
    if run_mode "vm" "$vm_flags" "$full" "" "$env_json"; then
        s_vm="PASS"
        TRIPLE_PASS=$((TRIPLE_PASS + 1))
    else
        TRIPLE_FAIL=$((TRIPLE_FAIL + 1))
        FAIL=$((FAIL + 1))
        if [[ $VERBOSE -eq 1 ]]; then
            echo "--- $id [vm] failed ---"
            "$BINARY" --vm $flags "$full" 2>&1 | head -5 || true
        fi
    fi

    local aot_tmp="$TMPDIR/dryad_triple_${id}"
    if run_mode "aot" "" "$full" "$aot_tmp" "$env_json"; then
        s_aot="PASS"
        TRIPLE_PASS=$((TRIPLE_PASS + 1))
    else
        TRIPLE_FAIL=$((TRIPLE_FAIL + 1))
        FAIL=$((FAIL + 1))
        if [[ $VERBOSE -eq 1 ]]; then
            echo "--- $id [aot] failed ---"
            "$BINARY" compile "$full" --aot -o "${aot_tmp}.ll" 2>&1 | tail -3 || true
        fi
    fi

    end=$(date +%s%3N)
    elapsed_ms=$((end - start))
    TOTAL_MS=$((TOTAL_MS + elapsed_ms))

    printf "%-36s %6s %6s %6s\n" "$id" "$s_interp" "$s_vm" "$s_aot"
}

run_example() {
    local id="$1"
    local path="$2"
    local flags="${3:-}"
    local optional="${4:-false}"
    local full="$SCRIPT_DIR/$path"

    if [[ ! -f "$full" ]]; then
        echo "MISSING  $id ($path)"
        FAIL=$((FAIL + 1))
        return
    fi

    if [[ "$optional" == "true" && "$INCLUDE_OPTIONAL" -eq 0 ]]; then
        printf "%-50s %8s %6s\n" "$id" "-" "SKIP"
        SKIP=$((SKIP + 1))
        return
    fi

    if [[ "$flags" == *"--allow-net"* && "$INCLUDE_NET" -eq 0 ]]; then
        printf "%-50s %8s %6s\n" "$id" "-" "SKIP"
        SKIP=$((SKIP + 1))
        return
    fi

    local start end elapsed_ms status output
    start=$(date +%s%3N)

    if [[ $VERBOSE -eq 1 ]]; then
        echo "--- $id: $BINARY $flags $full ---"
    fi

    local cmd=("$BINARY")
    if [[ -n "$flags" ]]; then
        # shellcheck disable=SC2206
        cmd+=($flags)
    fi
    cmd+=("$full")

    if output=$("${cmd[@]}" 2>&1); then
        status="PASS"
        PASS=$((PASS + 1))
    else
        status="FAIL"
        FAIL=$((FAIL + 1))
        if [[ $VERBOSE -eq 1 ]]; then
            echo "$output"
        fi
    fi

    end=$(date +%s%3N)
    elapsed_ms=$((end - start))
    TOTAL_MS=$((TOTAL_MS + elapsed_ms))

    printf "%-50s %6dms %6s\n" "$id" "$elapsed_ms" "$status"

    if [[ $VERBOSE -eq 1 && "$status" == "PASS" ]]; then
        echo "$output" | head -5
    fi
}

run_tooling() {
    local id="$1"
    local cmd="$2"
    local target="$3"
    local output="${4:-}"
    local flags="${5:-}"
    local full="$SCRIPT_DIR/$target"

    local start end elapsed_ms
    start=$(date +%s%3N)

    local cmd_args=("$BINARY" "$cmd" "$full")
    if [[ -n "$flags" ]]; then
        # shellcheck disable=SC2206
        cmd_args+=($flags)
    fi
    if [[ -n "$output" ]]; then
        cmd_args+=("-o" "$output")
    fi

    local elapsed status
    if "${cmd_args[@]}" >/dev/null 2>&1; then
        status="PASS"
        PASS=$((PASS + 1))
        [[ -n "$output" ]] && rm -f "$output"
    else
        status="FAIL"
        FAIL=$((FAIL + 1))
    fi
    end=$(date +%s%3N)
    elapsed=$((end - start))
    TOTAL_MS=$((TOTAL_MS + elapsed))
    printf "%-50s %6dms %6s\n" "$id" "$elapsed" "$status"
}

echo "Dryad Examples Benchmark"
echo "Binary: $BINARY"
echo "Stdlib: $STDLIB"
[[ $TRIPLE -eq 1 ]] && echo "Mode: triple (interpreter + VM + AOT compile)"

print_header

while IFS='|' read -r id path flags optional env_json; do
    if [[ $TRIPLE -eq 1 ]]; then
        run_example_triple "$id" "$path" "$flags" "$optional" "$env_json"
    else
        run_example "$id" "$path" "$flags" "$optional"
    fi
done < <(python3 - "$MANIFEST" <<'PY'
import json, sys
with open(sys.argv[1]) as f:
    data = json.load(f)
for ex in data["examples"]:
    flags = " ".join(ex.get("flags", []))
    optional = "true" if ex.get("optional") else "false"
    env = json.dumps(ex.get("env", {}))
    print(f"{ex['id']}|{ex['path']}|{flags}|{optional}|{env}")
PY
)

if [[ $TRIPLE -eq 0 ]]; then
    echo ""
    echo "Tooling commands:"
    printf "%-50s %8s %6s\n" "--------------------------------------------------" "--------" "------"

    while IFS='|' read -r id cmd target output flags; do
        run_tooling "$id" "$cmd" "$target" "$output" "$flags"
    done < <(python3 - "$MANIFEST" <<'PY'
import json, sys
with open(sys.argv[1]) as f:
    data = json.load(f)
for t in data.get("tooling", []):
    flags = " ".join(t.get("flags", []))
    print(f"{t['id']}|{t['command']}|{t['target']}|{t.get('output', '')}|{flags}")
PY
)
fi

echo ""
if [[ $TRIPLE -eq 1 ]]; then
    echo "Triple coverage: $TRIPLE_PASS/$TRIPLE_TOTAL cells passed ($TRIPLE_SKIP skipped)"
else
    echo "Summary: $PASS passed, $FAIL failed, $SKIP skipped, total ${TOTAL_MS}ms"
fi

if [[ $FAIL -gt 0 || $TRIPLE_FAIL -gt 0 ]]; then
    exit 1
fi
