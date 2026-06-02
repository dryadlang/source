# Ipe Framework

**Ipe** is a lightweight, cross-platform GUI framework for the [Dryad](https://github.com/dryadlang/source) programming language. It is inspired by Windows Forms and provides a simple, object-oriented API for building desktop applications.

Ipe uses a dual-layer architecture:
1.  **Native Layer (`ipe/native`)**: A C shared library (`ipe.so` on Linux, `ipe.dll` on Windows) that handles window creation, rendering, and event polling using system APIs (SDL2 on Linux, GDI/Win32 on Windows).
2.  **Dryad Layer (`ipe/lib`)**: A Dryad wrapper library that uses FFI to communicate with the native layer and provides high-level classes like `Form`, `Button`, and `Label`.

## Features

*   **Cross-Platform**: Runs on Linux (via SDL2) and Windows (via Win32 API).
*   **Object-Oriented**: Class-based component system (`Form`, `Control`, etc.).
*   **Event-Driven**: Built-in `EventEmitter` pattern for handling UI events (clicks, key presses).
*   **Dependency-Free Text**: Includes a built-in bitmap font renderer, requiring no external font libraries.
*   **Lightweight**: Minimal overhead and dependencies.

## Installation & Build

To use Ipe, you must first compile the native shared library.

### Linux (Requires SDL2)
1.  Install SDL2 development headers (e.g., `sudo apt install libsdl2-dev`).
2.  Navigate to the native directory and compile:
    ```bash
    cd ipe/native
    # You can use the provided Makefile:
    make
    # OR compile manually with gcc:
    gcc -shared -O2 -fPIC $(sdl2-config --cflags) ipe_helper_sdl2.c -o ipe.so -lSDL2
    ```

### Windows (Requires MinGW or similar)
1.  Navigate to the native directory:
    ```bash
    cd ipe/native
    ```
2.  Compile using `gcc`:
    ```bash
    gcc -shared -O2 ipe_helper.c -o ipe.dll -lgdi32 -luser32 -lkernel32
    ```

## Usage

Import the library in your Dryad script:

```javascript
import { Form, Button, Label, Application } from "ipe/lib/ipe.dryad";

function main() {
    // 1. Create the main form
    let form = new Form();
    form.title = "My Ipe App";
    form.width = 800;
    form.height = 600;

    // 2. Add a button
    let btn = new Button();
    btn.text = "Click Me";
    btn.x = 100;
    btn.y = 100;
    btn.width = 120;
    btn.height = 40;
    
    // 3. Add an event listener
    btn.on("click", function(data) {
        println("Button clicked!");
    });
    
    form.add(btn);

    // 4. Run the application loop
    Application.run(form);
}

main();
```

## API Reference

### `Control` (Base Class)
Base class for all UI components. Inherits from `EventEmitter`.
*   **Properties**:
    *   `x`, `y`: Position coordinates.
    *   `width`, `height`: Dimensions.
    *   `text`: Content text.
    *   `backColor`: Background color (hex format, e.g., `0xFF0000` for red).
    *   `visible`: Boolean visibility state.
    *   `foreColor`: Foreground/Text color.

### `Form`
The main window container.
*   **Properties**:
    *   `title`: Window title bar text.
    *   `controls`: List of child controls.
*   **Methods**:
    *   `add(control)`: Adds a child control to the form.

### `Button`
A clickable button component. Renders with a simple shadow effect and centered text.

### `Label`
A text display component. Uses the internal 8x8 bitmap font renderer.

### `Application`
Static class for managing the application lifecycle.
*   **Methods**:
    *   `run(mainForm)`: Starts the main event loop. This method blocks until the window is closed.

## File Structure

*   `ipe/lib/ipe.dryad`: Core Dryad library implementation.
*   `ipe/native/`: Native C source code.
    *   `ipe_helper_sdl2.c`: Linux/SDL2 backend.
    *   `ipe_helper.c`: Windows/GDI backend.
    *   `font8x8_basic.h`: Bitmap font data.
*   `ipe/tests/`: Example applications.
    *   `demo.dryad`: Interactive WinForms-style demo.

## License

MIT License. See `LICENSE` for details.
