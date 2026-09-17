# @std/collections — List, Map, Set, Queue

Dynamic collections without generics (untyped values for now). All implement `IEnumerable` where iteration applies.

## Modules

| Module | Purpose |
|--------|---------|
| `list.dryad` | Dynamic array with bounds checking, `map`/`filter`/`reduce` |
| `map.dryad` | Key-value store with `put`/`get`/`has`/`delete` |
| `set.dryad` | Unique values (backed by `Map`) |
| `queue.dryad` | FIFO queue with circular buffer, `enqueue`/`dequeue`/`peek` |
| `../collections.dryad` | Barrel: `import { List, Map, Set, Queue } from "@std/collections"` |

## Quick start

```dryad
import { List, Map, Set, Queue } from "@std/collections";

let list = new List();
list.add(1);
list.add(2);

let map = new Map();
map.put("name", "Dryad");

let unique = new Set();
unique.add("a");
unique.add("a"); // ignored
```

## API

**List**
- `new List()` — default capacity 16
- `new List(64)` — custom initial capacity
- `get(index)`, `set(index, value)`, `add`, `insert`, `remove`, `removeAt`
- `map(function(x) { ... })`, `filter(...)`, `reduce(...)`

**Map**
- `put(key, value)`, `get(key)`, `has(key)`, `delete(key)`
- `keys()`, `values()` → `List`

**Set**
- `add(item)`, `has(item)`, `delete(item)`, `clear`, `forEach`

**Queue**
- `new Queue()` — default capacity 16
- `new Queue(64)` — custom buffer size
- `enqueue(item)`, `dequeue()`, `peek()`, `isEmpty()`, `clear`, `forEach` (FIFO order)

## Running examples

```bash
./bin/dryad examples/03-collections/collections_usage.dryad
```
