# IPE Framework: Complete Documentation

This document serves as the single source of truth for the IPE (Interactive Programming Environment) Graphics Library.

---

## 📖 Table of Contents

1. [Core Concepts](#-core-concepts)
2. [API Reference](#-api-reference)
3. [Advanced Styling](#-advanced-styling)
4. [Transformations](#-transformations)
5. [Text Rendering](#-text-rendering)
6. [Input Handling](#-input-handling)
7. [Developer & Build Guide](#-developer--build-guide)

---

## 🏛️ Core Concepts

IPE is a C-based library designed specifically for the Dryad language. It uses SDL2 for hardware acceleration and cross-platform compatibility (Windows, Linux, macOS).

### Paradigms

- **Imperative Drawing**: You issue commands (draw rect, draw line) which are buffered and displayed on `present()`.
- **Stateful Transformations**: Transformation operations (rotate, scale) affect all subsequent drawing until popped or reset.
- **Buffer-based Styling**: Complex styles (gradients, shadows) are passed via FFI buffers for maximum efficiency.

---

## 📋 API Reference

### Lifecycle

- **`ipe::init() -> bool`**: Initializes SDL2 and the graphics subsystem. Must be called first.
- **`ipe::createWindow(w, h, title) -> pointer`**: Opens a new window.
- **`ipe::clear(color)`**: Clears the back buffer with the given hex color (e.g., `0x333333`).
- **`ipe::present()`**: Swaps the buffers to display what has been drawn.
- **`ipe::isOpen() -> bool`**: Returns true if the window is still open.
- **`ipe::processEvents()`**: Processes system events. Call this once per frame.
- **`ipe::close()`**: Clears resources and closes the window.

### Basic Drawing (Primitive)

- **`ipe::drawRect(x, y, w, h, color)`**: Draws a solid rectangle.
- **`ipe::drawCircle(cx, cy, r, color)`**: Draws a solid circle.
- **`ipe::drawLine(x1, y1, x2, y2, color, thickness)`**: Draws a line.
- **`ipe::drawPolygon(points, count, fill_color, border_color)`**: Draws a polygon.
- **`ipe::drawText(text, x, y, color)`**: Draws simple text using a built-in bitmap font.

---

## 🎨 Advanced Styling

For premium visuals, IPE supports complex style structures.

### Styled Functions

- `ipe::drawRectStyled(x, y, w, h, style_buffer)`
- `ipe::drawCircleStyled(cx, cy, radius, style_buffer)`
- `ipe::drawPolygonStyled(points_buffer, count, style_buffer)`

### Style Creation

Use `ipe::createRectStyle()`, `ipe::createCircleStyle()`, or `ipe::createPolygonStyle()` to allocate a buffer. Use `ffi_write_buffer` to set properties:

- **Opacity**: `ffi_write_buffer(style, OFFSET, value, "f32")`
- **Gradients**: Set `gradient_type` (1=LinearH, 2=LinearV, 3=Radial) and use `gradient_start`/`gradient_end`.
- **Shadows**: Set `shadow_enabled` to 1 and configure `shadow_radius`, `shadow_offset`, and `shadow_color`.

---

## 🔄 Transformations

IPE uses a transformation stack similar to modern graphics APIs.

- **`ipe::transformPush()`**: Saves the current transformation state.
- **`ipe::transformPop()`**: Restores the previous transformation state.
- **`ipe::translate(dx, dy)`**: Moves the origin.
- **`ipe::scale(sx, sy)`**: Scales all subsequent drawing.
- **`ipe::rotate(angle)`**: Rotates the coordinate system by the given angle in degrees.
- **`ipe::transformReset()`**: Resets the current transformation matrix to identity.

---

## 🔤 Text Rendering

Beyond basic bitmap text, IPE supports high-quality TrueType (TTF) fonts.

- **`ipe::fontLoad(path, size) -> pointer`**: Loads a .ttf file from the given path.
- **`ipe::drawTextEx(text, x, y, font, color)`**: Draws text using the loaded font.
- **`ipe::textWidth(text, font) -> int`**: Returns the width of the text in pixels.

---

## 🖱️ Input Handling

### Polling API

Query the state of the keyboard/mouse instantly:

- `ipe::input_key_down(key_code)`
- `ipe::input_mouse_pos()` (returns pointer/buffer with x, y)
- `ipe::input_mouse_button_down(button_id)`

---

## 🛠️ Developer & Build Guide

### Prerequisites

- GCC (Windows/Linux) or Clang (macOS).
- SDL2, SDL2_ttf, and SDL2_image development libraries.

### Build Commands

- **Windows**: Run `ipe/native/build.cmd`.
- **Unix**: Run `ipe/native/build.sh`.
- **Makefile**: Use `make sdl2` in the `ipe/native` directory.

### Directory Structure

- `/lib`: Dryad module (`ipe.dryad`).
- `/native`: C source code (`ipe_helper_sdl2.c`).
- `/tests`: Example and verification scripts.
- `/docs`: Additional guides and (legacy) plans.
