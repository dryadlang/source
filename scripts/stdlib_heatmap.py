#!/usr/bin/env python3
"""Render a self-contained HTML heatmap of the @std stdlib API surface.

Companion to coverage_heatmap.py: that one visualizes C++ runtime line/
function/branch coverage from gcovr; this one visualizes the @std stdlib's
public API surface (functions, classes, values) by whether each is tested
and/or documented, reusing the exact same scan coverage_map.py does (so the
numbers always match whatever `scripts/coverage_map.py` would report).

Usage (from the repository root, no coverage build required -- this only
scans src/stdlib, src/tests and docs/stdlib):

  python3 scripts/stdlib_heatmap.py --out coverage/stdlib_heatmap.html

Internal helpers (name starts with `_`, or nothing outside the file
references it on trees that predate the `_` convention) and deprecated
aliases are excluded, same as coverage_map.py's "public API" count.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(Path(__file__).resolve().parent))
from coverage_map import stdlib_section  # noqa: E402


def status_of(api: dict) -> str:
    if api["tested"] and api["documented"]:
        return "both"
    if api["tested"]:
        return "tested-only"
    if api["documented"]:
        return "documented-only"
    return "neither"


def build_dataset() -> dict:
    stdlib = stdlib_section()
    items = []
    for module in stdlib["modules"]:
        pkg = module["module"].split("/")[1] if "/" in module["module"] else module["module"]
        for api in module["apis"]:
            if api["internal"] or api.get("deprecated"):
                continue
            methods = api.get("methods", [])
            weight = 1 + len(methods)
            items.append({
                "package": pkg,
                "module": module["module"],
                "name": api["name"],
                "kind": api["kind"],
                "tested": api["tested"],
                "documented": api["documented"],
                "status": status_of(api),
                "weight": weight,
                "methods": [
                    {"name": m["name"], "tested": m["tested"], "documented": m["documented"]}
                    for m in methods
                ],
            })

    packages: dict[str, dict] = {}
    for item in items:
        p = packages.setdefault(item["package"], {"name": item["package"], "items": [], "weight": 0})
        p["items"].append(item)
        p["weight"] += item["weight"]

    package_list = []
    for p in sorted(packages.values(), key=lambda p: p["weight"], reverse=True):
        p["items"].sort(key=lambda i: (i["status"] != "neither", i["status"] != "tested-only",
                                        i["status"] != "documented-only", i["name"]))
        both = sum(1 for i in p["items"] if i["status"] == "both")
        package_list.append({
            "name": p["name"],
            "weight": p["weight"],
            "count": len(p["items"]),
            "bothPct": round(100.0 * both / len(p["items"]), 1) if p["items"] else 100.0,
            "items": p["items"],
        })

    s = stdlib["summary"]
    return {
        "generatedAt": None,
        "summary": s,
        "packages": package_list,
    }


HTML_TEMPLATE = """<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>dryadlang @std API heatmap</title>
<style>
  :root { color-scheme: dark; }
  * { box-sizing: border-box; }
  body {
    margin: 0; padding: 24px; background: #11141a; color: #d8dce3;
    font: 14px/1.4 -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  }
  h1 { font-size: 18px; margin: 0 0 4px; font-weight: 600; }
  .sub { color: #8891a3; margin-bottom: 16px; font-size: 13px; }
  .legend { display: flex; align-items: center; gap: 14px; margin-bottom: 16px; font-size: 12px; color: #c3c9d4; flex-wrap: wrap; }
  .legend .chip { display: flex; align-items: center; gap: 6px; }
  .legend .swatch { width: 14px; height: 14px; border-radius: 3px; display: inline-block; }
  #treemap { position: relative; width: 100%; height: 760px; border: 1px solid #2c3140; border-radius: 8px; overflow: hidden; }
  .pkg-box { position: absolute; border: 2px solid #11141a; overflow: hidden; }
  .pkg-label {
    position: absolute; top: 0; left: 0; right: 0; padding: 2px 6px;
    background: rgba(0,0,0,0.55); font-size: 11px; font-weight: 600; color: #fff;
    white-space: nowrap; overflow: hidden; text-overflow: ellipsis; pointer-events: none; z-index: 2;
  }
  .item-box {
    position: absolute; border: 1px solid rgba(0,0,0,0.35); cursor: pointer;
    display: flex; align-items: flex-end; padding: 3px 4px;
  }
  .item-box span {
    font-size: 10px; color: rgba(0,0,0,0.8); white-space: nowrap; overflow: hidden;
    text-overflow: ellipsis; max-width: 100%; text-shadow: 0 1px 1px rgba(255,255,255,0.25);
  }
  #tooltip {
    position: fixed; pointer-events: none; background: #1d212b; border: 1px solid #3b5bdb;
    border-radius: 6px; padding: 8px 10px; font-size: 12px; max-width: 340px; z-index: 50;
    display: none; box-shadow: 0 4px 16px rgba(0,0,0,0.4);
  }
  #tooltip b { color: #fff; }
  #tooltip .line { margin-top: 2px; }
  #tooltip .bad { color: #ff6b6b; }
  #tooltip .ok { color: #69db7c; }
  #tooltip .warn { color: #ffd43b; }
  #tooltip ul { margin: 4px 0 0; padding-left: 16px; }
</style>
</head>
<body>
<h1>dryadlang @std stdlib API heatmap</h1>
<div class="sub" id="subtitle"></div>
<div class="legend">
  <div class="chip"><span class="swatch" style="background:#2f9e44"></span> tested + documented</div>
  <div class="chip"><span class="swatch" style="background:#1098ad"></span> tested only</div>
  <div class="chip"><span class="swatch" style="background:#f08c00"></span> documented only</div>
  <div class="chip"><span class="swatch" style="background:#d9480f"></span> neither</div>
  <div class="chip" style="margin-left:auto;">box size = 1 + method count (classes)</div>
</div>
<div id="treemap"></div>
<div id="tooltip"></div>
<script>
const DATA = __DATA_JSON__;
const COLORS = { both: "#2f9e44", "tested-only": "#1098ad", "documented-only": "#f08c00", neither: "#d9480f" };

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
  const pkgBoxes = squarify(DATA.packages, 0, 0, W, H, "weight");
  pkgBoxes.forEach(pb => {
    const pkg = pb.item;
    const div = document.createElement("div");
    div.className = "pkg-box";
    div.style.left = pb.x + "px"; div.style.top = pb.y + "px";
    div.style.width = pb.w + "px"; div.style.height = pb.h + "px";
    const label = document.createElement("div");
    label.className = "pkg-label";
    label.textContent = `@std/${pkg.name} (${pkg.count} apis, ${pkg.bothPct}% done)`;
    div.appendChild(label);
    root.appendChild(div);

    const labelH = 16;
    const itemBoxes = squarify(pkg.items, pb.x, pb.y + labelH, pb.w, Math.max(0, pb.h - labelH), "weight");
    itemBoxes.forEach(ib => {
      const it = ib.item;
      const idiv = document.createElement("div");
      idiv.className = "item-box";
      idiv.style.left = ib.x + "px"; idiv.style.top = ib.y + "px";
      idiv.style.width = Math.max(0, ib.w) + "px"; idiv.style.height = Math.max(0, ib.h) + "px";
      idiv.style.background = COLORS[it.status];
      if (ib.w > 36 && ib.h > 14) {
        const span = document.createElement("span");
        span.textContent = it.name;
        idiv.appendChild(span);
      }
      idiv.addEventListener("mousemove", (e) => showTooltip(e, it));
      idiv.addEventListener("mouseleave", () => tooltip.style.display = "none");
      root.appendChild(idiv);
    });
  });
}

function showTooltip(e, it) {
  const flag = (v) => v ? '<span class="ok">yes</span>' : '<span class="bad">no</span>';
  let methodsHtml = "";
  if (it.methods.length) {
    const rows = it.methods.map(m =>
      `<li>${m.name}: tested ${flag(m.tested)}, documented ${flag(m.documented)}</li>`).join("");
    methodsHtml = `<ul>${rows}</ul>`;
  }
  tooltip.innerHTML = `<b>${it.module}.${it.name}</b> (${it.kind})
    <div class="line">tested: ${flag(it.tested)} &middot; documented: ${flag(it.documented)}</div>
    ${methodsHtml}`;
  tooltip.style.display = "block";
  const pad = 14;
  let left = e.clientX + pad, top = e.clientY + pad;
  if (left + 350 > window.innerWidth) left = e.clientX - 350 - pad;
  if (top + 160 > window.innerHeight) top = e.clientY - 160 - pad;
  tooltip.style.left = left + "px";
  tooltip.style.top = top + "px";
}

const s = DATA.summary;
document.getElementById("subtitle").textContent =
  `${s.publicApis} public APIs (${s.tested} tested, ${s.documented} documented) across ${s.modules} modules · ` +
  `${s.methods} methods (${s.methodsTested} tested, ${s.methodsDocumented} documented) · ` +
  `${s.internalHelpers} internal helpers and ${s.deprecatedAliases} deprecated aliases excluded` +
  (DATA.generatedAt ? ` · generated ${DATA.generatedAt}` : "");

window.addEventListener("resize", render);
render();
</script>
</body>
</html>
"""


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0] if __doc__ else "")
    parser.add_argument("--out", type=Path, required=True, help="output HTML path")
    args = parser.parse_args()

    dataset = build_dataset()
    from datetime import datetime, timezone
    dataset["generatedAt"] = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")

    html = HTML_TEMPLATE.replace("__DATA_JSON__", json.dumps(dataset))
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(html, encoding="utf-8")
    s = dataset["summary"]
    print(f"wrote {args.out}: {s['publicApis']} public APIs "
          f"({s['tested']} tested, {s['documented']} documented) "
          f"across {len(dataset['packages'])} packages")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
