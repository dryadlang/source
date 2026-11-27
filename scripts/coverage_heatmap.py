#!/usr/bin/env python3
"""Render a self-contained HTML coverage heatmap from a gcovr --json report.

Usage (from the repository root, after a coverage build + gcovr run with the
same flags as CI's coverage job — see the dryadlang-dev skill):

  python3 scripts/coverage_heatmap.py --json coverage/full.json \\
      --out coverage/heatmap.html

Produces a single HTML file (no network dependency, no CDN) with a two-level
squarified treemap: modules (src/<area>, matching scripts/coverage_map.py's
grouping) sized by total line count, and within each module its files, also
sized by line count and colored on a red -> yellow -> green scale by
whichever coverage kind (line/function/branch) is currently selected. Open
it directly in a browser; nothing needs to be served.
"""

from __future__ import annotations

import argparse
import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LINE_FLOOR = 70.0
LINE_TARGET = 80.0


def module_of(file: str) -> str:
    parts = Path(file).parts
    return parts[1] if len(parts) > 2 and parts[0] == "src" else parts[0]


def pct(covered: int, total: int) -> float:
    return round(100.0 * covered / total, 1) if total else 100.0


def git(*args: str) -> str:
    result = subprocess.run(["git", *args], cwd=ROOT, capture_output=True, text=True)
    return result.stdout.strip()


def build_dataset(report: dict) -> dict:
    files = []
    for f in report.get("files", []):
        path = f["file"]
        lines = f.get("lines", [])
        line_total = len(lines)
        line_covered = sum(1 for l in lines if l["count"] > 0)
        branches = [b for l in lines for b in l.get("branches", [])]
        branch_total = len(branches)
        branch_covered = sum(1 for b in branches if b.get("count", 0) > 0)
        funcs = f.get("functions", [])
        func_total = len(funcs)
        func_covered = sum(1 for fn in funcs if fn.get("execution_count", 0) > 0)
        uncovered = [l["line_number"] for l in lines if l["count"] == 0]
        if line_total == 0:
            continue
        files.append({
            "file": path,
            "module": module_of(path),
            "lines": line_total,
            "line": pct(line_covered, line_total),
            "function": pct(func_covered, func_total),
            "branch": pct(branch_covered, branch_total),
            "uncoveredCount": len(uncovered),
            "uncoveredSample": uncovered[:12],
        })

    modules: dict[str, dict] = {}
    for item in files:
        m = modules.setdefault(item["module"], {"name": item["module"], "files": [],
                                                  "lines": 0, "lineCovered": 0,
                                                  "funcTotal": 0, "funcCovered": 0,
                                                  "branchTotal": 0, "branchCovered": 0})
        m["files"].append(item)
        m["lines"] += item["lines"]
        m["lineCovered"] += round(item["lines"] * item["line"] / 100.0)

    total_lines = sum(i["lines"] for i in files)
    total_line_covered = sum(round(i["lines"] * i["line"] / 100.0) for i in files)

    module_list = []
    for m in sorted(modules.values(), key=lambda m: m["lines"], reverse=True):
        m["files"].sort(key=lambda f: f["line"])
        module_list.append({
            "name": m["name"],
            "lines": m["lines"],
            "line": pct(m["lineCovered"], m["lines"]),
            "files": m["files"],
        })

    return {
        "generatedAt": None,
        "commit": git("rev-parse", "--short", "HEAD"),
        "floor": LINE_FLOOR,
        "target": LINE_TARGET,
        "totalLines": total_lines,
        "totalLine": pct(total_line_covered, total_lines),
        "modules": module_list,
    }


HTML_TEMPLATE = """<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>dryadlang coverage heatmap</title>
<style>
  :root { color-scheme: dark; }
  * { box-sizing: border-box; }
  body {
    margin: 0; padding: 24px; background: #11141a; color: #d8dce3;
    font: 14px/1.4 -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  }
  h1 { font-size: 18px; margin: 0 0 4px; font-weight: 600; }
  .sub { color: #8891a3; margin-bottom: 16px; font-size: 13px; }
  .controls { display: flex; gap: 8px; align-items: center; margin-bottom: 16px; flex-wrap: wrap; }
  .controls button {
    background: #1d212b; color: #d8dce3; border: 1px solid #2c3140; border-radius: 6px;
    padding: 6px 12px; cursor: pointer; font-size: 13px;
  }
  .controls button.active { background: #3b5bdb; border-color: #3b5bdb; color: #fff; }
  .legend { display: flex; align-items: center; gap: 6px; margin-left: auto; font-size: 12px; color: #8891a3; }
  .legend .swatch { width: 14px; height: 14px; border-radius: 3px; display: inline-block; }
  #treemap { position: relative; width: 100%; height: 760px; border: 1px solid #2c3140; border-radius: 8px; overflow: hidden; }
  .module-box {
    position: absolute; border: 2px solid #11141a; overflow: hidden;
  }
  .module-label {
    position: absolute; top: 0; left: 0; right: 0; padding: 2px 6px;
    background: rgba(0,0,0,0.55); font-size: 11px; font-weight: 600; color: #fff;
    white-space: nowrap; overflow: hidden; text-overflow: ellipsis; pointer-events: none; z-index: 2;
  }
  .file-box {
    position: absolute; border: 1px solid rgba(0,0,0,0.35); cursor: pointer;
    display: flex; align-items: flex-end; padding: 3px 4px;
  }
  .file-box span {
    font-size: 10px; color: rgba(0,0,0,0.75); white-space: nowrap; overflow: hidden;
    text-overflow: ellipsis; max-width: 100%; text-shadow: 0 1px 1px rgba(255,255,255,0.25);
  }
  #tooltip {
    position: fixed; pointer-events: none; background: #1d212b; border: 1px solid #3b5bdb;
    border-radius: 6px; padding: 8px 10px; font-size: 12px; max-width: 320px; z-index: 50;
    display: none; box-shadow: 0 4px 16px rgba(0,0,0,0.4);
  }
  #tooltip b { color: #fff; }
  #tooltip .line { margin-top: 2px; }
  #tooltip .bad { color: #ff6b6b; }
  #tooltip .ok { color: #69db7c; }
</style>
</head>
<body>
<h1>dryadlang/source coverage heatmap</h1>
<div class="sub" id="subtitle"></div>
<div class="controls">
  <button data-metric="line" class="active">line %</button>
  <button data-metric="function">function %</button>
  <button data-metric="branch">branch %</button>
  <div class="legend">
    <span>0%</span>
    <span class="swatch" style="background:#d9480f"></span>
    <span class="swatch" style="background:#f08c00"></span>
    <span class="swatch" style="background:#e8c547"></span>
    <span class="swatch" style="background:#74b816"></span>
    <span class="swatch" style="background:#2f9e44"></span>
    <span>100%</span>
    <span style="margin-left:10px;">box size = lines in file</span>
  </div>
</div>
<div id="treemap"></div>
<div id="tooltip"></div>
<script>
const DATA = __DATA_JSON__;
let metric = "line";

function color(pct) {
  // red (0) -> orange -> yellow -> green (100), via HSL interpolation
  const hue = Math.max(0, Math.min(100, pct)) * 1.1; // 0=red(0) .. 100=green(110)
  const light = 42;
  return `hsl(${hue}, 62%, ${light}%)`;
}

// Classic squarified treemap (Bruls, Huizing, van Wijk).
function squarify(items, x, y, w, h, key) {
  const boxes = [];
  const total = items.reduce((s, i) => s + i[key], 0);
  if (total <= 0 || items.length === 0) return boxes;
  const scale = (w * h) / total;
  const sized = items.map(i => ({ item: i, area: i[key] * scale }));

  function layoutRow(row, x, y, w, h, horizontal) {
    const rowTotal = row.reduce((s, r) => s + r.area, 0);
    let offset = 0;
    row.forEach(r => {
      if (horizontal) {
        const rw = w, rh = rowTotal > 0 ? (r.area / rowTotal) * h : 0;
        boxes.push({ item: r.item, x, y: y + offset, w: rw, h: rh });
        offset += rh;
      } else {
        const rh = h, rw = rowTotal > 0 ? (r.area / rowTotal) * w : 0;
        boxes.push({ item: r.item, x: x + offset, y, w: rw, h: rh });
        offset += rw;
      }
    });
  }

  function worst(row, length) {
    const sum = row.reduce((s, r) => s + r.area, 0);
    if (sum === 0) return Infinity;
    const max = Math.max(...row.map(r => r.area));
    const min = Math.min(...row.map(r => r.area));
    return Math.max((length * length * max) / (sum * sum), (sum * sum) / (length * length * min));
  }

  let remaining = sized.slice().sort((a, b) => b.area - a.area);
  let cx = x, cy = y, cw = w, ch = h;

  while (remaining.length) {
    const horizontal = cw >= ch;
    const length = horizontal ? ch : cw;
    let row = [remaining[0]];
    let rest = remaining.slice(1);
    let bestWorst = worst(row, length);
    while (rest.length) {
      const trial = row.concat([rest[0]]);
      const trialWorst = worst(trial, length);
      if (trialWorst <= bestWorst) {
        row = trial; rest = rest.slice(1); bestWorst = trialWorst;
      } else break;
    }
    const rowArea = row.reduce((s, r) => s + r.area, 0);
    if (horizontal) {
      const rowW = rowArea / ch;
      layoutRow(row, cx, cy, rowW, ch, false);
      cx += rowW; cw -= rowW;
    } else {
      const rowH = rowArea / cw;
      layoutRow(row, cx, cy, cw, rowH, true);
      cy += rowH; ch -= rowH;
    }
    remaining = rest;
  }
  return boxes;
}

const root = document.getElementById("treemap");
const tooltip = document.getElementById("tooltip");

function render() {
  root.innerHTML = "";
  const W = root.clientWidth, H = root.clientHeight;
  const moduleBoxes = squarify(DATA.modules, 0, 0, W, H, "lines");
  moduleBoxes.forEach(mb => {
    const mod = mb.item;
    const div = document.createElement("div");
    div.className = "module-box";
    div.style.left = mb.x + "px"; div.style.top = mb.y + "px";
    div.style.width = mb.w + "px"; div.style.height = mb.h + "px";
    const label = document.createElement("div");
    label.className = "module-label";
    label.textContent = `${mod.name} (${mod.line}% line, ${mod.lines} lines)`;
    div.appendChild(label);
    root.appendChild(div);

    const labelH = 16;
    const fileBoxes = squarify(mod.files, mb.x, mb.y + labelH, mb.w, Math.max(0, mb.h - labelH), "lines");
    fileBoxes.forEach(fb => {
      const f = fb.item;
      const fdiv = document.createElement("div");
      fdiv.className = "file-box";
      fdiv.style.left = fb.x + "px"; fdiv.style.top = fb.y + "px";
      fdiv.style.width = Math.max(0, fb.w) + "px"; fdiv.style.height = Math.max(0, fb.h) + "px";
      fdiv.style.background = color(f[metric]);
      if (fb.w > 36 && fb.h > 14) {
        const span = document.createElement("span");
        span.textContent = f.file.split("/").pop();
        fdiv.appendChild(span);
      }
      fdiv.addEventListener("mousemove", (e) => showTooltip(e, f));
      fdiv.addEventListener("mouseleave", () => tooltip.style.display = "none");
      root.appendChild(fdiv);
    });
  });
}

function showTooltip(e, f) {
  const cls = (v) => v < DATA.floor ? "bad" : "ok";
  tooltip.innerHTML = `<b>${f.file}</b>
    <div class="line">line: <span class="${cls(f.line)}">${f.line}%</span> (${f.lines} lines, ${f.uncoveredCount} uncovered)</div>
    <div class="line">function: <span class="${cls(f.function)}">${f.function}%</span></div>
    <div class="line">branch: <span class="${cls(f.branch)}">${f.branch}%</span></div>
    ${f.uncoveredSample.length ? `<div class="line">first uncovered lines: ${f.uncoveredSample.join(", ")}${f.uncoveredCount > f.uncoveredSample.length ? "…" : ""}</div>` : ""}`;
  tooltip.style.display = "block";
  const pad = 14;
  let left = e.clientX + pad, top = e.clientY + pad;
  if (left + 330 > window.innerWidth) left = e.clientX - 330 - pad;
  if (top + 140 > window.innerHeight) top = e.clientY - 140 - pad;
  tooltip.style.left = left + "px";
  tooltip.style.top = top + "px";
}

document.querySelectorAll(".controls button[data-metric]").forEach(btn => {
  btn.addEventListener("click", () => {
    document.querySelectorAll(".controls button[data-metric]").forEach(b => b.classList.remove("active"));
    btn.classList.add("active");
    metric = btn.dataset.metric;
    render();
  });
});

document.getElementById("subtitle").textContent =
  `${DATA.totalLine}% overall line coverage (floor ${DATA.floor}%, target ${DATA.target}%) · ` +
  `${DATA.totalLines} lines · commit ${DATA.commit}` +
  (DATA.generatedAt ? ` · generated ${DATA.generatedAt}` : "");

window.addEventListener("resize", render);
render();
</script>
</body>
</html>
"""


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--json", type=Path, required=True, help="gcovr --json report")
    parser.add_argument("--out", type=Path, required=True, help="output HTML path")
    args = parser.parse_args()

    report = json.loads(args.json.read_text(encoding="utf-8"))
    dataset = build_dataset(report)
    from datetime import datetime, timezone
    dataset["generatedAt"] = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")

    html = HTML_TEMPLATE.replace("__DATA_JSON__", json.dumps(dataset))
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(html, encoding="utf-8")
    print(f"wrote {args.out}: {len(dataset['modules'])} modules, "
          f"{sum(len(m['files']) for m in dataset['modules'])} files, "
          f"{dataset['totalLine']}% overall line coverage")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
