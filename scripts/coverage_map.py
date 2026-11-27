#!/usr/bin/env python3
"""Generate the coverage map shown on dryadlang.org (/coverage).

Combines three sources into one JSON snapshot:

  * test line/function/branch coverage of the C++ runtime, from a gcovr
    JSON summary (same flags as the CI coverage job);
  * the @std API surface: every top-level function/class/value and public
    method in src/stdlib, with whether tests reference it and whether
    docs/stdlib documents it;
  * the execution-tier matrix from docs/implementation/plans/BYTECODE_COVERAGE.md.

Usage (from the repository root):

  python3 scripts/coverage_map.py --gcovr-summary coverage/summary.json \\
      --out docs/website/src/data/coverage.json

Without --gcovr-summary the line-coverage section is copied from the existing
--out file, so the stdlib and tier sections can be refreshed without a
coverage build. --check exits 1 when --out would change (for CI).
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
STDLIB = ROOT / "src" / "stdlib"
TESTS = ROOT / "src" / "tests"
DOCS = ROOT / "docs"
TIER_TABLE = DOCS / "implementation" / "plans" / "BYTECODE_COVERAGE.md"

LINE_TARGET = 80.0
LINE_FLOOR = 70.0

TOP_FUNCTION = re.compile(r"^(?:async\s+)?function\s+([A-Za-z_]\w*)\s*\(")
TOP_CLASS = re.compile(r"^class\s+([A-Za-z_]\w*)")
TOP_VALUE = re.compile(r"^(?:let|const)\s+([A-Za-z_]\w*)")
METHOD = re.compile(r"^\s+(?:static\s+)?(?:async\s+)?([A-Za-z_]\w*)\s*\(")
PROPERTY = re.compile(r"^\s+([A-Za-z_]\w*)\s*:\s*[\w\[\]|<> ]+\s*\{")
KEYWORDS = {"if", "while", "for", "return", "switch", "catch", "function", "match"}


def module_name(path: Path) -> str:
    rel = path.relative_to(STDLIB).with_suffix("")
    return rel.as_posix()


def doc_page_for(module: str) -> str | None:
    parts = module.split("/")
    if len(parts) < 2:
        return None
    if parts[1] == "async" and len(parts) > 2 and parts[2] == "threading":
        name = "async-threading"
    else:
        name = parts[1]
    page = DOCS / "stdlib" / f"{name}.md"
    return page.name if page.exists() else None


def strip_comment(line: str) -> str:
    in_string = None
    for i, ch in enumerate(line):
        if in_string:
            if ch == in_string and line[i - 1] != "\\":
                in_string = None
        elif ch in "\"'`":
            in_string = ch
        elif line.startswith("//", i):
            return line[:i]
    return line


def parse_module(path: Path) -> list[dict]:
    apis: list[dict] = []
    current_class: dict | None = None
    depth = 0
    for raw in path.read_text(encoding="utf-8").splitlines():
        line = strip_comment(raw)
        if depth == 0:
            current_class = None
            for kind, pattern in (("function", TOP_FUNCTION), ("class", TOP_CLASS),
                                  ("value", TOP_VALUE)):
                match = pattern.match(line)
                if match:
                    entry = {"name": match.group(1), "kind": kind}
                    if kind == "class":
                        entry["methods"] = []
                        current_class = entry
                    apis.append(entry)
                    break
        elif depth == 1 and current_class is not None:
            match = METHOD.match(line) or PROPERTY.match(line)
            if match:
                name = match.group(1)
                if name not in KEYWORDS and name != "constructor" and not name.startswith("_"):
                    if name not in {m["name"] for m in current_class["methods"]}:
                        current_class["methods"].append({"name": name})
        depth += line.count("{") - line.count("}")
        if depth == 0 and current_class is not None and "}" in line:
            current_class = None
    return apis


def read_corpus(directory: Path, suffixes: tuple[str, ...]) -> str:
    chunks = []
    for path in sorted(directory.rglob("*")):
        if path.is_file() and path.suffix in suffixes:
            chunks.append(path.read_text(encoding="utf-8", errors="replace"))
    return "\n".join(chunks)


def mentions(corpus: str, name: str) -> bool:
    return re.search(rf"(?<![\w$]){re.escape(name)}(?![\w$])", corpus) is not None


def method_mentioned(corpus: str, name: str) -> bool:
    return re.search(rf"\.{re.escape(name)}(?![\w$])", corpus) is not None


def stdlib_section() -> dict:
    test_corpus = read_corpus(TESTS, (".dryad", ".cpp"))
    doc_texts = {p.name: p.read_text(encoding="utf-8") for p in (DOCS / "stdlib").glob("*.md")}
    example_corpus = read_corpus(DOCS / "examples", (".dryad", ".dyd"))
    sources = {path: path.read_text(encoding="utf-8") for path in sorted(STDLIB.rglob("*.dryad"))}
    modules = []
    for path in sources:
        module = module_name(path)
        page = doc_page_for(module)
        doc_corpus = doc_texts.get(page, "") if page else ""
        # The stdlib has no export keyword and its naming is not consistent
        # (Path_join is public, List_ensureIndex is a helper), so visibility
        # is inferred from use: a name nothing outside its own file refers
        # to is reported as an internal helper.
        outside = "\n".join([test_corpus, example_corpus, *doc_texts.values(),
                             *(text for other, text in sources.items() if other != path)])
        apis = parse_module(path)
        for api in apis:
            api["tested"] = mentions(test_corpus, api["name"])
            api["documented"] = mentions(doc_corpus, api["name"])
            api["internal"] = api["name"].startswith("_") or not mentions(outside, api["name"])
            for method in api.get("methods", []):
                method["tested"] = method_mentioned(test_corpus, method["name"])
                method["documented"] = mentions(doc_corpus, method["name"])
        modules.append({
            "module": module,
            "source": path.relative_to(ROOT).as_posix(),
            "docPage": page,
            "apis": apis,
        })

    public = [a for m in modules for a in m["apis"] if not a["internal"]]
    methods = [mt for a in public for mt in a.get("methods", [])]
    return {
        "summary": {
            "modules": len(modules),
            "publicApis": len(public),
            "internalHelpers": sum(1 for m in modules for a in m["apis"] if a["internal"]),
            "tested": sum(1 for a in public if a["tested"]),
            "documented": sum(1 for a in public if a["documented"]),
            "methods": len(methods),
            "methodsTested": sum(1 for mt in methods if mt["tested"]),
            "methodsDocumented": sum(1 for mt in methods if mt["documented"]),
        },
        "modules": modules,
    }


def markdown_table(text: str, heading: str) -> list[list[str]]:
    lines = text.splitlines()
    try:
        start = next(i for i, l in enumerate(lines) if l.strip().startswith("## ") and heading in l)
    except StopIteration:
        return []
    rows: list[list[str]] = []
    for line in lines[start + 1:]:
        if line.startswith("## "):
            break
        if not line.startswith("|"):
            if rows:
                break
            continue
        cells = [c.strip().replace("\\|", "|") for c in re.split(r"(?<!\\)\|", line.strip())[1:-1]]
        if all(set(c) <= set(":- ") for c in cells):
            continue
        rows.append(cells)
    return rows[1:] if rows else []


def tier_section() -> dict:
    text = TIER_TABLE.read_text(encoding="utf-8")
    constructs = [
        {"construct": r[0], "interpreter": r[1], "bytecode": r[2], "notes": r[3] if len(r) > 3 else ""}
        for r in markdown_table(text, "Language constructs")
    ]
    interpreter_only = [
        {"construct": r[0], "reason": r[1] if len(r) > 1 else ""}
        for r in markdown_table(text, "Explicitly interpreter-only")
    ]
    return {"source": TIER_TABLE.relative_to(ROOT).as_posix(),
            "constructs": constructs, "interpreterOnly": interpreter_only}


def pct(covered: int, total: int) -> float:
    return round(100.0 * covered / total, 1) if total else 100.0


def line_section(summary_path: Path) -> dict:
    data = json.loads(summary_path.read_text(encoding="utf-8"))
    groups: dict[str, dict] = {}
    for f in data.get("files", []):
        name = f["filename"]
        parts = Path(name).parts
        group = parts[1] if len(parts) > 2 and parts[0] == "src" else parts[0]
        g = groups.setdefault(group, {"name": group, "files": [],
                                      "line": [0, 0], "function": [0, 0], "branch": [0, 0]})
        g["files"].append({
            "file": name,
            "line": pct(f["line_covered"], f["line_total"]),
            "lines": f["line_total"],
            "function": pct(f["function_covered"], f["function_total"]),
            "branch": pct(f["branch_covered"], f["branch_total"]),
        })
        for key in ("line", "function", "branch"):
            g[key][0] += f[f"{key}_covered"]
            g[key][1] += f[f"{key}_total"]
    modules = []
    for g in sorted(groups.values(), key=lambda g: g["name"]):
        g["files"].sort(key=lambda f: f["line"])
        modules.append({
            "name": g["name"],
            "line": pct(*g["line"]), "lines": g["line"][1],
            "function": pct(*g["function"]),
            "branch": pct(*g["branch"]),
            "files": g["files"],
        })
    return {
        "target": LINE_TARGET,
        "floor": LINE_FLOOR,
        "line": round(data["line_percent"], 1),
        "function": round(data["function_percent"], 1),
        "branch": round(data["branch_percent"], 1),
        "linesTotal": data["line_total"],
        "linesCovered": data["line_covered"],
        "modules": modules,
    }


def gaps(lines: dict | None, stdlib: dict) -> list[dict]:
    out: list[dict] = []
    if lines:
        for module in lines["modules"]:
            for f in module["files"]:
                if f["lines"] >= 50 and f["line"] < lines["floor"]:
                    out.append({"area": "runtime", "kind": "low-coverage",
                                "item": f["file"], "detail": f"{f['line']}% lines"})
    for module in stdlib["modules"]:
        for api in module["apis"]:
            if api["internal"]:
                continue
            if not api["documented"]:
                out.append({"area": "stdlib", "kind": "undocumented",
                            "item": f"{module['module']}.{api['name']}", "detail": api["kind"]})
            if not api["tested"]:
                out.append({"area": "stdlib", "kind": "untested",
                            "item": f"{module['module']}.{api['name']}", "detail": api["kind"]})
    return out


def git(*args: str) -> str:
    return subprocess.run(["git", *args], cwd=ROOT, capture_output=True, text=True).stdout.strip()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--gcovr-summary", type=Path, help="gcovr --json-summary output")
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--check", action="store_true",
                        help="exit 1 if --out is stale (ignores generatedAt)")
    args = parser.parse_args()

    previous = json.loads(args.out.read_text()) if args.out.exists() else {}
    if args.gcovr_summary:
        lines = line_section(args.gcovr_summary)
        lines["commit"] = git("rev-parse", "--short", "HEAD")
    else:
        lines = previous.get("lineCoverage")

    stdlib = stdlib_section()
    snapshot = {
        "generatedAt": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "lineCoverage": lines,
        "stdlib": stdlib,
        "tiers": tier_section(),
        "gaps": gaps(lines, stdlib),
    }

    if args.check:
        strip = lambda d: {k: v for k, v in d.items() if k != "generatedAt"}
        if strip(previous) != strip(snapshot):
            print(f"{args.out} is stale; rerun scripts/coverage_map.py", file=sys.stderr)
            return 1
        return 0

    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(snapshot, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    s = stdlib["summary"]
    print(f"wrote {args.out}: {s['publicApis']} public APIs "
          f"({s['tested']} tested, {s['documented']} documented), "
          f"{len(snapshot['gaps'])} gaps"
          + (f", line coverage {lines['line']}%" if lines else ", no line coverage"))
    return 0


if __name__ == "__main__":
    sys.exit(main())
