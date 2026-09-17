# Syntax Reference

Summary of Dryad surface syntax as implemented in `dryad-cpp/`. For the full formal spec see [dryad_theoretical_foundation_v2.pdf](../../dryad_theory/dryad_theoretical_foundation_v2.pdf).

## Literals

```dryad
let n = 42;
let pi = 3.14;
let name = "Dryad";
let ok = true;
let empty = null;
let items = [1, 2, 3];
```

## Variables

```dryad
let x = 1;          // mutable binding
const y = 2;        // constant (enforced at runtime)
```

## Functions

```dryad
function add(a, b) {
    return a + b;
}

async function fetch() {
    return await somePromise();
}

export function publicApi() { return 1; }
internal function helper() { return 2; }
```

## Control flow

```dryad
if (x > 0) {
    print("positive");
} else {
    print("non-positive");
}

while (n > 0) { n = n - 1; }

for (let i = 0; i < 10; i = i + 1) {
    print(i);
}

break;
continue;
return value;
```

## Classes & interfaces

```dryad
class Animal {
    function speak() {
        return "...";
    }
}

class Dog extends Animal {
    function speak() {
        return "woof";
    }
}

interface Drawable {
    function draw(): void;
}

abstract class Shape {
    abstract function area(): number;
}
```

### Properties

```dryad
class Point {
    get x(): number { return this._x; }
    set x(v: number) { this._x = v; }
    readonly label: string;
}
```

### Construction

```dryad
let p = new Point(3, 4);
p.x = 10;
```

## Operators

| Category | Operators |
|----------|-----------|
| Arithmetic | `+` `-` `*` `/` `%` |
| Comparison | `==` `!=` `<` `<=` `>` `>=` |
| Logical | `!` `&&` `\|\|` |
| Assignment | `=` `+=` `-=` `*=` `/=` |
| Member / call | `.` `()` `[]` |

## Decorators

### `@compile(mode)` — execution tier

```dryad
@compile("bytecode")
function fast() { return 1 + 2; }

@compile("AOT")
function native() { return 3 * 4; }
```

Modes: `"interpreter"`, `"bytecode"`, `"AOT"`.

### `@intrinsic(name)` — syscall binding

```dryad
@intrinsic("syscall.read")
extern function __read(fd: number, buf: byte, len: number): number;
```

Used internally by the standard library. Application code should use `@std/*` instead.

## Async / await

```dryad
async function main() {
    let x = await Promise_resolve(42);
    return x;
}
```

Requires the event loop pump at script end (CLI does this automatically).

## Not yet implemented

These keywords are recognized by the lexer but have no parser support:

- `try`, `catch`, `finally`, `throw`

## See also

- [Modules & imports](modules.md)
- [Execution modes](execution-modes.md)
- [AST diagram](../diagrams/dryad-language/parts/03-ast.puml)
