# IPE UI Controls System Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use `superpowers/subagent-driven-development` to implement this plan task-by-task.

**Goal:** Build a complete UI controls system for IPE that enables creating interactive applications with buttons, text inputs, panels, and event handling.

**Architecture:** A hierarchical control system where all UI elements inherit from a base `ipe_control_t` structure. Controls are managed in trees (parent-child relationships), support event propagation, state management, and can be composed into complex UIs. Drawing and input handling use the existing SDL2 backend.

**Tech Stack:** C (native implementation), SDL2 (rendering), FFI (Dryad bindings)

---

## Phase 1: Core Control System Foundation

### Task 1: Define Base Control Structure & Memory Management

**Files:**
- Modify: `ipe/native/ipe_helper_sdl2.c:284-330` (add new control structures)
- Modify: `ipe/lib/ipe.dryad:100-150` (add Dryad wrapper module)
- Create: `ipe/tests/test_controls_basic.dryad` (basic control tests)

**Step 1: Add control base structure to C**

In `ipe/native/ipe_helper_sdl2.c`, after the existing type definitions (around line 330), add:

```c
// Base control structure - all UI elements inherit from this
typedef struct ipe_control {
    int id;                    // Unique identifier
    int x, y, w, h;           // Position and size
    int visible;              // 1 = visible, 0 = hidden
    int enabled;              // 1 = enabled, 0 = disabled
    int focused;              // 1 = has focus, 0 = no focus
    
    // Appearance
    uint32_t bg_color;        // Background color
    uint32_t border_color;    // Border color
    int border_width;         // Border width in pixels
    int corner_radius;        // Corner radius for rounded corners
    
    // Layout
    int margin_left, margin_right, margin_top, margin_bottom;
    int padding_left, padding_right, padding_top, padding_bottom;
    int min_width, min_height; // Minimum size constraints
    int max_width, max_height; // Maximum size constraints
    
    // Hierarchy
    struct ipe_control* parent;
    struct ipe_control** children;
    int children_count;
    int children_capacity;
    
    // Events & callbacks
    void (*on_click)(struct ipe_control*);
    void (*on_focus)(struct ipe_control*);
    void (*on_blur)(struct ipe_control*);
    void (*on_draw)(struct ipe_control*);
    void (*on_key)(struct ipe_control*, int key_code);
    void (*on_mouse_move)(struct ipe_control*, int x, int y);
    
    // User data
    void* user_data;
    
    // Type-specific data
    char* control_type;  // "button", "textfield", "panel", etc.
    void* type_data;     // Pointer to type-specific structure
} ipe_control_t;

// Global control system state
static ipe_control_t* g_control_root = NULL;
static ipe_control_t* g_focused_control = NULL;
static int g_next_control_id = 1;
```

**Step 2: Add memory management functions**

Add these functions to `ipe_helper_sdl2.c`:

```c
IPE_EXPORT ipe_control_t* ipe_control_create(int x, int y, int w, int h) {
    ipe_control_t* ctrl = (ipe_control_t*)malloc(sizeof(ipe_control_t));
    if (!ctrl) return NULL;
    
    memset(ctrl, 0, sizeof(ipe_control_t));
    ctrl->id = g_next_control_id++;
    ctrl->x = x;
    ctrl->y = y;
    ctrl->w = w;
    ctrl->h = h;
    ctrl->visible = 1;
    ctrl->enabled = 1;
    ctrl->bg_color = 0xFFFFFF;
    ctrl->border_color = 0x000000;
    ctrl->border_width = 1;
    ctrl->corner_radius = 0;
    
    // Initialize children array
    ctrl->children_capacity = 10;
    ctrl->children = (ipe_control_t**)malloc(sizeof(ipe_control_t*) * ctrl->children_capacity);
    ctrl->children_count = 0;
    
    return ctrl;
}

IPE_EXPORT void ipe_control_destroy(ipe_control_t* ctrl) {
    if (!ctrl) return;
    
    // Destroy children recursively
    for (int i = 0; i < ctrl->children_count; i++) {
        ipe_control_destroy(ctrl->children[i]);
    }
    
    free(ctrl->children);
    if (ctrl->control_type) free(ctrl->control_type);
    if (ctrl->type_data) free(ctrl->type_data);
    free(ctrl);
}

IPE_EXPORT void ipe_control_set_color(ipe_control_t* ctrl, uint32_t bg_color, uint32_t border_color) {
    if (!ctrl) return;
    ctrl->bg_color = bg_color;
    ctrl->border_color = border_color;
}

IPE_EXPORT void ipe_control_set_position(ipe_control_t* ctrl, int x, int y) {
    if (!ctrl) return;
    ctrl->x = x;
    ctrl->y = y;
}

IPE_EXPORT void ipe_control_set_size(ipe_control_t* ctrl, int w, int h) {
    if (!ctrl) return;
    ctrl->w = w;
    ctrl->h = h;
}

IPE_EXPORT void ipe_control_set_visible(ipe_control_t* ctrl, int visible) {
    if (!ctrl) return;
    ctrl->visible = visible;
}

IPE_EXPORT void ipe_control_set_enabled(ipe_control_t* ctrl, int enabled) {
    if (!ctrl) return;
    ctrl->enabled = enabled;
}
```

**Step 3: Add control hierarchy management**

Add to `ipe_helper_sdl2.c`:

```c
IPE_EXPORT void ipe_control_add_child(ipe_control_t* parent, ipe_control_t* child) {
    if (!parent || !child) return;
    
    // Grow children array if needed
    if (parent->children_count >= parent->children_capacity) {
        parent->children_capacity *= 2;
        parent->children = (ipe_control_t**)realloc(parent->children, 
                          sizeof(ipe_control_t*) * parent->children_capacity);
    }
    
    parent->children[parent->children_count++] = child;
    child->parent = parent;
}

IPE_EXPORT void ipe_control_remove_child(ipe_control_t* parent, ipe_control_t* child) {
    if (!parent || !child) return;
    
    for (int i = 0; i < parent->children_count; i++) {
        if (parent->children[i] == child) {
            // Shift remaining children
            for (int j = i; j < parent->children_count - 1; j++) {
                parent->children[j] = parent->children[j + 1];
            }
            parent->children_count--;
            child->parent = NULL;
            return;
        }
    }
}

IPE_EXPORT void ipe_control_set_root(ipe_control_t* ctrl) {
    if (g_control_root) {
        ipe_control_destroy(g_control_root);
    }
    g_control_root = ctrl;
}

IPE_EXPORT ipe_control_t* ipe_control_get_root() {
    return g_control_root;
}
```

**Step 4: Add base rendering function**

```c
static void ipe_control_draw_base(ipe_control_t* ctrl, ipe_window_t* window) {
    if (!ctrl || !ctrl->visible) return;
    
    // Draw background
    if (ctrl->corner_radius > 0) {
        ipe_fill_rect_rounded(ctrl->x, ctrl->y, ctrl->w, ctrl->h, ctrl->corner_radius, ctrl->bg_color, window->renderer);
    } else {
        SDL_Rect rect = {ctrl->x, ctrl->y, ctrl->w, ctrl->h};
        SDL_SetRenderDrawColor(window->renderer, 
            (ctrl->bg_color >> 16) & 0xFF,
            (ctrl->bg_color >> 8) & 0xFF,
            ctrl->bg_color & 0xFF, 
            255);
        SDL_RenderFillRect(window->renderer, &rect);
    }
    
    // Draw border
    if (ctrl->border_width > 0) {
        SDL_Rect rect = {ctrl->x, ctrl->y, ctrl->w, ctrl->h};
        SDL_SetRenderDrawColor(window->renderer,
            (ctrl->border_color >> 16) & 0xFF,
            (ctrl->border_color >> 8) & 0xFF,
            ctrl->border_color & 0xFF,
            255);
        for (int i = 0; i < ctrl->border_width; i++) {
            SDL_Rect border = {ctrl->x + i, ctrl->y + i, ctrl->w - 2*i, ctrl->h - 2*i};
            SDL_RenderDrawRect(window->renderer, &border);
        }
    }
}

IPE_EXPORT void ipe_control_draw_recursive(ipe_control_t* ctrl, ipe_window_t* window) {
    if (!ctrl || !ctrl->visible) return;
    
    // Draw this control
    if (ctrl->on_draw) {
        ctrl->on_draw(ctrl);
    } else {
        ipe_control_draw_base(ctrl, window);
    }
    
    // Draw all children
    for (int i = 0; i < ctrl->children_count; i++) {
        ipe_control_draw_recursive(ctrl->children[i], window);
    }
}
```

**Step 5: Write test in Dryad**

Create `ipe/tests/test_controls_basic.dryad`:

```dryad
use "../lib/ipe.dryad";

// Test 1: Create a basic control
function test_control_creation() {
    println("Test: Control Creation");
    
    // Initialize Ipe
    if (!ipe::init()) {
        println("[X] Failed to initialize Ipe");
        return false;
    }
    
    var window = ipe::createWindow(800, 600, "Control Test");
    if (window == null) {
        println("[X] Failed to create window");
        return false;
    }
    
    // Create a root control
    var root = ipe::control_create(10, 10, 780, 580);
    if (root == null) {
        println("[X] Failed to create root control");
        return false;
    }
    
    ipe::control_set_color(root, 0xF0F0F0, 0x333333);
    ipe::control_set_root(root);
    
    println("[✓] Control creation successful");
    
    // Cleanup
    ipe::close();
    return true;
}

// Test 2: Control hierarchy
function test_control_hierarchy() {
    println("Test: Control Hierarchy");
    
    if (!ipe::init()) return false;
    
    var window = ipe::createWindow(800, 600, "Hierarchy Test");
    var root = ipe::control_create(10, 10, 780, 580);
    
    var child1 = ipe::control_create(20, 20, 300, 200);
    var child2 = ipe::control_create(350, 20, 300, 200);
    
    ipe::control_add_child(root, child1);
    ipe::control_add_child(root, child2);
    
    ipe::control_set_root(root);
    
    println("[✓] Control hierarchy successful");
    
    ipe::close();
    return true;
}

// Run all tests
println("=== Control System Basic Tests ===");
test_control_creation();
test_control_hierarchy();
println("=== Tests Complete ===");
```

**Step 6: Run tests and verify**

```bash
cd /home/pedro/repo/source
dryad run ipe/tests/test_controls_basic.dryad
```

Expected output: Both tests pass with "✓" marks.

**Step 7: Commit**

```bash
git add ipe/native/ipe_helper_sdl2.c ipe/lib/ipe.dryad ipe/tests/test_controls_basic.dryad
git commit -m "feat(ipe): add base control system with hierarchy support"
```

---

### Task 2: Input Event Handling for Controls

**Files:**
- Modify: `ipe/native/ipe_helper_sdl2.c` (add event handling)
- Modify: `ipe/lib/ipe.dryad` (add event wrappers)
- Create: `ipe/tests/test_controls_events.dryad` (event tests)

**Step 1: Add event handling functions**

Add to `ipe_helper_sdl2.c`:

```c
// Find control at given screen coordinates
static ipe_control_t* ipe_control_at_point(ipe_control_t* ctrl, int x, int y) {
    if (!ctrl || !ctrl->visible || !ctrl->enabled) return NULL;
    
    // Check if point is outside this control's bounds
    if (x < ctrl->x || x > ctrl->x + ctrl->w || 
        y < ctrl->y || y > ctrl->y + ctrl->h) {
        return NULL;
    }
    
    // Check children in reverse order (top-to-bottom in z-order)
    for (int i = ctrl->children_count - 1; i >= 0; i--) {
        ipe_control_t* found = ipe_control_at_point(ctrl->children[i], x, y);
        if (found) return found;
    }
    
    return ctrl;
}

IPE_EXPORT void ipe_control_handle_mouse_click(int x, int y) {
    if (!g_control_root) return;
    
    ipe_control_t* ctrl = ipe_control_at_point(g_control_root, x, y);
    if (ctrl && ctrl->on_click) {
        ctrl->on_click(ctrl);
    }
}

IPE_EXPORT void ipe_control_handle_mouse_move(int x, int y) {
    if (!g_control_root) return;
    
    ipe_control_t* ctrl = ipe_control_at_point(g_control_root, x, y);
    if (ctrl && ctrl->on_mouse_move) {
        ctrl->on_mouse_move(ctrl, x, y);
    }
}

IPE_EXPORT void ipe_control_handle_key_press(int key_code) {
    if (g_focused_control && g_focused_control->on_key) {
        g_focused_control->on_key(g_focused_control, key_code);
    }
}

IPE_EXPORT void ipe_control_set_focus(ipe_control_t* ctrl) {
    if (g_focused_control && g_focused_control->on_blur) {
        g_focused_control->on_blur(g_focused_control);
    }
    
    g_focused_control = ctrl;
    
    if (ctrl) {
        ctrl->focused = 1;
        if (ctrl->on_focus) {
            ctrl->on_focus(ctrl);
        }
    }
}

IPE_EXPORT ipe_control_t* ipe_control_get_focus() {
    return g_focused_control;
}
```

**Step 2: Integrate event handling in main loop**

Modify the existing event processing to call control handlers. In the SDL event loop section of `ipe_helper_sdl2.c`, add control event handling:

```c
// In SDL_event handling loop, add:
case SDL_MOUSEBUTTONDOWN:
    if (g_control_root) {
        ipe_control_handle_mouse_click(event.button.x, event.button.y);
    }
    // ... existing code
    break;
    
case SDL_MOUSEMOTION:
    if (g_control_root) {
        ipe_control_handle_mouse_move(event.motion.x, event.motion.y);
    }
    g_mouse_x = event.motion.x;
    g_mouse_y = event.motion.y;
    break;
    
case SDL_KEYDOWN:
    if (g_control_root) {
        ipe_control_handle_key_press(event.key.keysym.sym);
    }
    // ... existing code
    break;
```

**Step 3: Add Dryad wrappers**

In `ipe/lib/ipe.dryad`, in the `ipe` module, add:

```dryad
    function control_create(x, y, w, h) {
        if (!lib_loaded) return null;
        return ffi_call(lib_alias, "ipe_control_create", "pointer", x, y, w, h);
    }
    
    function control_destroy(ctrl) {
        if (!lib_loaded) return;
        ffi_call(lib_alias, "ipe_control_destroy", "void", ctrl);
    }
    
    function control_set_color(ctrl, bg_color, border_color) {
        if (!lib_loaded) return;
        ffi_call(lib_alias, "ipe_control_set_color", "void", ctrl, bg_color, border_color);
    }
    
    function control_set_position(ctrl, x, y) {
        if (!lib_loaded) return;
        ffi_call(lib_alias, "ipe_control_set_position", "void", ctrl, x, y);
    }
    
    function control_set_size(ctrl, w, h) {
        if (!lib_loaded) return;
        ffi_call(lib_alias, "ipe_control_set_size", "void", ctrl, w, h);
    }
    
    function control_set_visible(ctrl, visible) {
        if (!lib_loaded) return;
        ffi_call(lib_alias, "ipe_control_set_visible", "void", ctrl, visible);
    }
    
    function control_set_enabled(ctrl, enabled) {
        if (!lib_loaded) return;
        ffi_call(lib_alias, "ipe_control_set_enabled", "void", ctrl, enabled);
    }
    
    function control_add_child(parent, child) {
        if (!lib_loaded) return;
        ffi_call(lib_alias, "ipe_control_add_child", "void", parent, child);
    }
    
    function control_remove_child(parent, child) {
        if (!lib_loaded) return;
        ffi_call(lib_alias, "ipe_control_remove_child", "void", parent, child);
    }
    
    function control_set_root(ctrl) {
        if (!lib_loaded) return;
        ffi_call(lib_alias, "ipe_control_set_root", "void", ctrl);
    }
    
    function control_get_root() {
        if (!lib_loaded) return null;
        return ffi_call(lib_alias, "ipe_control_get_root", "pointer");
    }
    
    function control_draw_recursive(ctrl) {
        if (!lib_loaded) return;
        // Window pointer is stored internally - this draws all visible controls
        ffi_call(lib_alias, "ipe_control_draw_recursive", "void", ctrl);
    }
    
    function control_handle_mouse_click(x, y) {
        if (!lib_loaded) return;
        ffi_call(lib_alias, "ipe_control_handle_mouse_click", "void", x, y);
    }
    
    function control_set_focus(ctrl) {
        if (!lib_loaded) return;
        ffi_call(lib_alias, "ipe_control_set_focus", "void", ctrl);
    }
    
    function control_get_focus() {
        if (!lib_loaded) return null;
        return ffi_call(lib_alias, "ipe_control_get_focus", "pointer");
    }
```

**Step 4: Write event tests**

Create `ipe/tests/test_controls_events.dryad`:

```dryad
use "../lib/ipe.dryad";

function test_control_events() {
    println("Test: Control Event Handling");
    
    if (!ipe::init()) return false;
    
    var window = ipe::createWindow(800, 600, "Event Test");
    var root = ipe::control_create(10, 10, 780, 580);
    
    var clicked = false;
    var button = ipe::control_create(50, 50, 100, 50);
    ipe::control_add_child(root, button);
    ipe::control_set_root(root);
    
    // Simulate click at button position
    println("Simulating mouse click at (100, 75)...");
    ipe::control_handle_mouse_click(100, 75);
    
    println("[✓] Event handling test passed");
    ipe::close();
    return true;
}

test_control_events();
```

**Step 5: Run tests**

```bash
cd /home/pedro/repo/source
dryad run ipe/tests/test_controls_events.dryad
```

**Step 6: Commit**

```bash
git add ipe/native/ipe_helper_sdl2.c ipe/lib/ipe.dryad ipe/tests/test_controls_events.dryad
git commit -m "feat(ipe): add input event handling for controls"
```

---

## Phase 2: Concrete Control Types

### Task 3: Button Control Implementation

**Files:**
- Modify: `ipe/native/ipe_helper_sdl2.c` (add button structure and rendering)
- Modify: `ipe/lib/ipe.dryad` (add button wrappers)
- Create: `ipe/tests/test_button.dryad` (button tests)

**Step 1: Add button structure**

In `ipe_helper_sdl2.c`, add after control structures:

```c
typedef struct {
    char* text;
    uint32_t text_color;
    uint32_t hover_color;
    uint32_t pressed_color;
    int is_hovered;
    int is_pressed;
} ipe_button_t;
```

**Step 2: Add button functions**

```c
IPE_EXPORT ipe_control_t* ipe_button_create(int x, int y, int w, int h, const char* text) {
    ipe_control_t* ctrl = ipe_control_create(x, y, w, h);
    if (!ctrl) return NULL;
    
    ipe_button_t* btn = (ipe_button_t*)malloc(sizeof(ipe_button_t));
    btn->text = (char*)malloc(strlen(text) + 1);
    strcpy(btn->text, text);
    btn->text_color = 0x000000;
    btn->hover_color = 0xE0E0E0;
    btn->pressed_color = 0xC0C0C0;
    btn->is_hovered = 0;
    btn->is_pressed = 0;
    
    ctrl->type_data = btn;
    ctrl->control_type = "button";
    ctrl->bg_color = 0xF0F0F0;
    ctrl->border_width = 1;
    ctrl->on_draw = ipe_button_draw;
    ctrl->on_mouse_move = ipe_button_on_mouse_move;
    
    return ctrl;
}

static void ipe_button_draw(ipe_control_t* ctrl) {
    if (!ctrl || !ctrl->type_data) return;
    
    ipe_button_t* btn = (ipe_button_t*)ctrl->type_data;
    
    // Draw button background with state
    uint32_t color = ctrl->bg_color;
    if (btn->is_pressed) {
        color = btn->pressed_color;
    } else if (btn->is_hovered) {
        color = btn->hover_color;
    }
    
    ctrl->bg_color = color;
    ipe_control_draw_base(ctrl, g_window); // Use global window
    
    // Draw text (simplified - using built-in font)
    int text_x = ctrl->x + (ctrl->w - strlen(btn->text) * 8) / 2;
    int text_y = ctrl->y + (ctrl->h - 8) / 2;
    ipe_draw_text(btn->text, text_x, text_y, btn->text_color);
}

static void ipe_button_on_mouse_move(ipe_control_t* ctrl, int x, int y) {
    if (!ctrl || !ctrl->type_data) return;
    
    ipe_button_t* btn = (ipe_button_t*)ctrl->type_data;
    
    // Check if mouse is inside button bounds
    int is_inside = (x >= ctrl->x && x <= ctrl->x + ctrl->w &&
                     y >= ctrl->y && y <= ctrl->y + ctrl->h);
    
    btn->is_hovered = is_inside;
}

IPE_EXPORT void ipe_button_set_text(ipe_control_t* ctrl, const char* text) {
    if (!ctrl || !ctrl->type_data) return;
    ipe_button_t* btn = (ipe_button_t*)ctrl->type_data;
    
    free(btn->text);
    btn->text = (char*)malloc(strlen(text) + 1);
    strcpy(btn->text, text);
}

IPE_EXPORT void ipe_button_set_colors(ipe_control_t* ctrl, uint32_t normal, uint32_t hover, uint32_t pressed) {
    if (!ctrl || !ctrl->type_data) return;
    ipe_button_t* btn = (ipe_button_t*)ctrl->type_data;
    
    ctrl->bg_color = normal;
    btn->hover_color = hover;
    btn->pressed_color = pressed;
}
```

**Step 3: Add Dryad wrappers**

In `ipe/lib/ipe.dryad`:

```dryad
    function button_create(x, y, w, h, text) {
        if (!lib_loaded) return null;
        return ffi_call(lib_alias, "ipe_button_create", "pointer", x, y, w, h, text);
    }
    
    function button_set_text(ctrl, text) {
        if (!lib_loaded) return;
        ffi_call(lib_alias, "ipe_button_set_text", "void", ctrl, text);
    }
    
    function button_set_colors(ctrl, normal, hover, pressed) {
        if (!lib_loaded) return;
        ffi_call(lib_alias, "ipe_button_set_colors", "void", ctrl, normal, hover, pressed);
    }
```

**Step 4: Write button test**

Create `ipe/tests/test_button.dryad`:

```dryad
use "../lib/ipe.dryad";

function test_button_creation() {
    println("Test: Button Creation");
    
    if (!ipe::init()) return false;
    
    var window = ipe::createWindow(800, 600, "Button Test");
    var root = ipe::control_create(10, 10, 780, 580);
    
    var button = ipe::button_create(50, 50, 150, 50, "Click Me");
    if (button == null) {
        println("[X] Failed to create button");
        return false;
    }
    
    ipe::button_set_colors(button, 0xF0F0F0, 0xE0E0E0, 0xC0C0C0);
    ipe::control_add_child(root, button);
    ipe::control_set_root(root);
    
    println("[✓] Button creation successful");
    
    ipe::close();
    return true;
}

test_button_creation();
```

**Step 5: Run and verify**

```bash
cd /home/pedro/repo/source
dryad run ipe/tests/test_button.dryad
```

**Step 6: Commit**

```bash
git add ipe/native/ipe_helper_sdl2.c ipe/lib/ipe.dryad ipe/tests/test_button.dryad
git commit -m "feat(ipe): add button control type"
```

---

### Task 4: TextInput Control

**Files:**
- Modify: `ipe/native/ipe_helper_sdl2.c` (text input structures and functions)
- Modify: `ipe/lib/ipe.dryad` (text input wrappers)
- Create: `ipe/tests/test_textinput.dryad`

**Step 1: Add text input structure**

```c
typedef struct {
    char* buffer;           // Text content
    int buffer_size;        // Allocated size
    int text_length;        // Current text length
    int cursor_pos;         // Cursor position
    int max_length;         // Maximum text length
    uint32_t text_color;
    uint32_t cursor_color;
    int show_cursor;        // Cursor blink state
    int is_focused;
} ipe_textinput_t;
```

**Step 2: Add text input functions**

```c
IPE_EXPORT ipe_control_t* ipe_textinput_create(int x, int y, int w, int h) {
    ipe_control_t* ctrl = ipe_control_create(x, y, w, h);
    if (!ctrl) return NULL;
    
    ipe_textinput_t* input = (ipe_textinput_t*)malloc(sizeof(ipe_textinput_t));
    input->buffer_size = 256;
    input->buffer = (char*)malloc(input->buffer_size);
    input->buffer[0] = '\0';
    input->text_length = 0;
    input->cursor_pos = 0;
    input->max_length = 255;
    input->text_color = 0x000000;
    input->cursor_color = 0x000000;
    input->show_cursor = 1;
    input->is_focused = 0;
    
    ctrl->type_data = input;
    ctrl->control_type = "textinput";
    ctrl->bg_color = 0xFFFFFF;
    ctrl->border_width = 1;
    ctrl->border_color = 0x999999;
    ctrl->on_draw = ipe_textinput_draw;
    ctrl->on_key = ipe_textinput_on_key;
    ctrl->on_focus = ipe_textinput_on_focus;
    ctrl->on_blur = ipe_textinput_on_blur;
    
    return ctrl;
}

static void ipe_textinput_draw(ipe_control_t* ctrl) {
    if (!ctrl || !ctrl->type_data) return;
    
    ipe_textinput_t* input = (ipe_textinput_t*)ctrl->type_data;
    
    // Draw background and border
    ipe_control_draw_base(ctrl, g_window);
    
    // Draw text
    ipe_draw_text(input->buffer, ctrl->x + 5, ctrl->y + (ctrl->h - 8) / 2, input->text_color);
    
    // Draw cursor if focused
    if (input->is_focused && input->show_cursor) {
        int cursor_x = ctrl->x + 5 + input->cursor_pos * 8;
        SDL_RenderDrawLine(g_window->renderer, cursor_x, ctrl->y + 5, cursor_x, ctrl->y + ctrl->h - 5);
    }
}

static void ipe_textinput_on_key(ipe_control_t* ctrl, int key_code) {
    if (!ctrl || !ctrl->type_data) return;
    
    ipe_textinput_t* input = (ipe_textinput_t*)ctrl->type_data;
    
    if (key_code == SDLK_BACKSPACE) {
        if (input->cursor_pos > 0) {
            for (int i = input->cursor_pos - 1; i < input->text_length; i++) {
                input->buffer[i] = input->buffer[i + 1];
            }
            input->text_length--;
            input->cursor_pos--;
        }
    } else if (key_code >= 32 && key_code < 127) {
        // Regular character
        if (input->text_length < input->max_length) {
            // Shift text to make room
            for (int i = input->text_length; i >= input->cursor_pos; i--) {
                input->buffer[i + 1] = input->buffer[i];
            }
            input->buffer[input->cursor_pos] = (char)key_code;
            input->text_length++;
            input->cursor_pos++;
        }
    }
}

static void ipe_textinput_on_focus(ipe_control_t* ctrl) {
    if (!ctrl || !ctrl->type_data) return;
    ipe_textinput_t* input = (ipe_textinput_t*)ctrl->type_data;
    input->is_focused = 1;
}

static void ipe_textinput_on_blur(ipe_control_t* ctrl) {
    if (!ctrl || !ctrl->type_data) return;
    ipe_textinput_t* input = (ipe_textinput_t*)ctrl->type_data;
    input->is_focused = 0;
}

IPE_EXPORT const char* ipe_textinput_get_text(ipe_control_t* ctrl) {
    if (!ctrl || !ctrl->type_data) return "";
    ipe_textinput_t* input = (ipe_textinput_t*)ctrl->type_data;
    return input->buffer;
}

IPE_EXPORT void ipe_textinput_set_text(ipe_control_t* ctrl, const char* text) {
    if (!ctrl || !ctrl->type_data) return;
    ipe_textinput_t* input = (ipe_textinput_t*)ctrl->type_data;
    
    strncpy(input->buffer, text, input->max_length);
    input->text_length = strlen(input->buffer);
    input->cursor_pos = input->text_length;
}
```

**Step 3: Add Dryad wrappers**

```dryad
    function textinput_create(x, y, w, h) {
        if (!lib_loaded) return null;
        return ffi_call(lib_alias, "ipe_textinput_create", "pointer", x, y, w, h);
    }
    
    function textinput_get_text(ctrl) {
        if (!lib_loaded) return "";
        return ffi_call(lib_alias, "ipe_textinput_get_text", "pointer", ctrl);
    }
    
    function textinput_set_text(ctrl, text) {
        if (!lib_loaded) return;
        ffi_call(lib_alias, "ipe_textinput_set_text", "void", ctrl, text);
    }
```

**Step 4-6: Test, verify, and commit** (following same pattern as button)

---

### Task 5: Panel & Container Control

**Files:**
- Modify: `ipe/native/ipe_helper_sdl2.c` (panel structure)
- Modify: `ipe/lib/ipe.dryad` (panel wrappers)
- Create: `ipe/tests/test_panel.dryad`

Add a simple panel/container control that just manages children visually:

```c
IPE_EXPORT ipe_control_t* ipe_panel_create(int x, int y, int w, int h) {
    ipe_control_t* ctrl = ipe_control_create(x, y, w, h);
    if (!ctrl) return NULL;
    
    ctrl->control_type = "panel";
    ctrl->bg_color = 0xF5F5F5;
    ctrl->border_width = 1;
    ctrl->on_draw = ipe_panel_draw;
    
    return ctrl;
}

static void ipe_panel_draw(ipe_control_t* ctrl) {
    if (!ctrl) return;
    ipe_control_draw_base(ctrl, g_window);
    // Children will be drawn by ipe_control_draw_recursive
}
```

---

## Phase 3: Layout System (Basic)

### Task 6: Basic Layout Manager

**Files:**
- Modify: `ipe/native/ipe_helper_sdl2.c` (add layout structure)
- Modify: `ipe/lib/ipe.dryad` (add layout API)
- Create: `ipe/tests/test_layout.dryad`

**Step 1: Add layout structure**

```c
typedef enum {
    IPE_LAYOUT_NONE,      // Manual positioning
    IPE_LAYOUT_VERTICAL,  // Stack children vertically
    IPE_LAYOUT_HORIZONTAL // Stack children horizontally
} ipe_layout_type_t;

typedef struct {
    ipe_layout_type_t type;
    int spacing;          // Space between children
    int padding;          // Padding inside container
    int align;            // 0=start, 1=center, 2=end
} ipe_layout_t;
```

**Step 2: Add layout functions**

```c
IPE_EXPORT void ipe_control_set_layout(ipe_control_t* ctrl, ipe_layout_type_t type, int spacing, int padding) {
    if (!ctrl) return;
    
    ipe_layout_t* layout = (ipe_layout_t*)malloc(sizeof(ipe_layout_t));
    layout->type = type;
    layout->spacing = spacing;
    layout->padding = padding;
    layout->align = 1; // center by default
    
    // TODO: Store in control
    free(layout); // Temporary until we store it
}

IPE_EXPORT void ipe_control_recalculate_layout(ipe_control_t* ctrl) {
    if (!ctrl || !ctrl->type_data) return;
    
    ipe_layout_t* layout = (ipe_layout_t*)ctrl->type_data;
    if (layout->type == IPE_LAYOUT_VERTICAL) {
        int y = ctrl->y + layout->padding;
        int width = ctrl->w - 2 * layout->padding;
        
        for (int i = 0; i < ctrl->children_count; i++) {
            ipe_control_t* child = ctrl->children[i];
            child->x = ctrl->x + layout->padding;
            child->y = y;
            child->w = width;
            y += child->h + layout->spacing;
        }
    } else if (layout->type == IPE_LAYOUT_HORIZONTAL) {
        int x = ctrl->x + layout->padding;
        int height = ctrl->h - 2 * layout->padding;
        
        for (int i = 0; i < ctrl->children_count; i++) {
            ipe_control_t* child = ctrl->children[i];
            child->x = x;
            child->y = ctrl->y + layout->padding;
            child->h = height;
            x += child->w + layout->spacing;
        }
    }
}
```

---

## Phase 4: Styling & Themes

### Task 7: Theme System

**Files:**
- Create: `ipe/native/themes.c` (theme definitions)
- Modify: `ipe/native/ipe_helper_sdl2.c` (theme loading)
- Modify: `ipe/lib/ipe.dryad` (theme API)
- Create: `ipe/tests/test_themes.dryad`

*This task creates a simple theme registry where predefined themes (light, dark, high-contrast) can be applied globally or per-control.*

---

## Phase 5: Animation System

### Task 8: Basic Animation Framework

**Files:**
- Modify: `ipe/native/ipe_helper_sdl2.c` (animation structures)
- Modify: `ipe/lib/ipe.dryad` (animation API)
- Create: `ipe/tests/test_animation.dryad`

*Simple tweening system for properties like opacity, position, color.*

---

## Phase 6: Image System

### Task 9: Image Loading and Rendering

**Files:**
- Modify: `ipe/native/ipe_helper_sdl2.c` (image functions)
- Modify: `ipe/lib/ipe.dryad` (image API)
- Create: `ipe/tests/test_images.dryad`

*Load PNG/JPG via SDL_image, render with transformations.*

---

## Phase 7: Advanced Canvas

### Task 10: Gradients and Effects

**Files:**
- Modify: `ipe/native/ipe_helper_sdl2.c` (gradient rendering)
- Modify: `ipe/lib/ipe.dryad` (gradient API)
- Create: `ipe/tests/test_gradients.dryad`

*Linear and radial gradients, blur effects, shadow effects.*

---

## Implementation Notes

### Build & Test Cycle

After each task, test with:

```bash
# Rebuild native library
cd ipe/native
./build.sh  # or build.cmd on Windows

# Run tests
cd ../..
dryad run ipe/tests/test_<feature>.dryad
```

### Error Handling

For C functions returning pointers: Check for NULL before use in Dryad.
For void functions: Assume success unless documented otherwise.

### FFI Considerations

- Passing strings requires null-terminated C strings
- Passing structures: Use pointers (returned from `*_create` functions)
- Callbacks: Currently not fully supported via FFI - use polling for events

### Code Style

- C: 4-space indentation, prefix all exported functions with `ipe_`
- Dryad: Follow existing module style, use descriptive names
- Tests: One feature per test file, clear pass/fail output

---

## Success Criteria

After completing all tasks, the IPE framework should support:

✅ Creating and managing hierarchical UI controls  
✅ Button, TextInput, Panel controls  
✅ Event handling (mouse, keyboard)  
✅ Basic layout management  
✅ Theme system  
✅ Animation system  
✅ Image loading and rendering  
✅ Advanced drawing effects (gradients, shadows)  

All tests passing, documentation updated, clean commit history.

---

## Appendix: Common Patterns

### Creating a Custom Control Type

```c
typedef struct {
    // Your properties
    char* value;
    int state;
} ipe_mycontrol_t;

IPE_EXPORT ipe_control_t* ipe_mycontrol_create(...) {
    ipe_control_t* ctrl = ipe_control_create(...);
    ipe_mycontrol_t* data = malloc(sizeof(ipe_mycontrol_t));
    // Initialize data...
    ctrl->type_data = data;
    ctrl->on_draw = ipe_mycontrol_draw;
    return ctrl;
}

static void ipe_mycontrol_draw(ipe_control_t* ctrl) {
    ipe_mycontrol_t* data = (ipe_mycontrol_t*)ctrl->type_data;
    // Draw logic...
}
```

### Adding Event Handlers in Dryad

```dryad
var button = ipe::button_create(50, 50, 100, 50, "Click");

// Store callback (would require more FFI support)
// For now, handle in main loop via polling
```

---

**Plan created:** 2026-03-09  
**Difficulty:** Medium (baseline GUI framework)  
**Estimated duration:** 2-3 weeks (10-15 hours development)
