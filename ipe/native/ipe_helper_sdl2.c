#include "config.h"
#include <SDL2/SDL_image.h>
#include <SDL2/SDL_ttf.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <math.h>

/* Platform-specific logging */
#define IPE_LOG(fmt, ...) fprintf(stderr, "[IPE] " fmt "\n", ##__VA_ARGS__)
#define IPE_ERR(fmt, ...) fprintf(stderr, "[IPE ERROR] " fmt "\n", ##__VA_ARGS__)

// Window structure
typedef struct {
  SDL_Window *window;
  SDL_Renderer *renderer;
  int id;
  char *title;
  int width, height;
  int is_open;
  struct ipe_control *root_control; // Added for functionality
} ipe_window_t;

// Color structure
typedef struct {
  uint8_t r, g, b, a;
} ipe_color_t;

// Font structure
typedef struct {
  uint8_t *data;
  int char_width;
  int char_height;
  TTF_Font *ttf_font; // Keep internal SDL-related data
} ipe_font_t;

// Image structure
typedef struct {
  SDL_Texture *texture;
  int width;
  int height;
} ipe_image_t;

// Phase 1: Advanced Graphics Styles
typedef struct {
    uint32_t fill_color;
    uint32_t border_color;
    int border_width;
    int border_style;        // 0=solid, 1=dashed, 2=dotted
    int corner_radius;       // Same radius all corners
    int corner_radius_tl;    // Top-left
    int corner_radius_tr;    // Top-right
    int corner_radius_bl;    // Bottom-left
    int corner_radius_br;    // Bottom-right
    float opacity;           // 0.0-1.0
    int gradient_type;       // 0=none, 1=linear_h, 2=linear_v, 3=radial
    uint32_t gradient_start;
    uint32_t gradient_end;
    int shadow_enabled;
    int shadow_radius;
    int shadow_offset_x;
    int shadow_offset_y;
    uint32_t shadow_color;
    int blend_mode;          // IPE_BLEND_* constants
} ipe_rect_style_t;

typedef struct {
    uint32_t fill_color;
    uint32_t border_color;
    int border_width;
    float opacity;
    uint32_t color_center;   // For radial gradients
    uint32_t color_edge;
    int shadow_enabled;
    int shadow_radius;
    int shadow_offset_x;
    int shadow_offset_y;
    uint32_t shadow_color;
    int blend_mode;
} ipe_circle_style_t;

typedef struct {
    uint32_t fill_color;
    uint32_t border_color;
    int border_width;
    int border_style;        // 0=solid, 1=dashed, 2=dotted
    float opacity;
    int gradient_type;       // 0=none, 1=linear_h, 2=linear_v
    uint32_t gradient_start;
    uint32_t gradient_end;
    int shadow_enabled;
    int shadow_radius;
    uint32_t shadow_color;
    int blend_mode;
} ipe_polygon_style_t;

typedef enum {
    IPE_BLEND_NORMAL,
    IPE_BLEND_MULTIPLY,
    IPE_BLEND_SCREEN,
    IPE_BLEND_OVERLAY,
    IPE_BLEND_LIGHTEN,
    IPE_BLEND_DARKEN
} ipe_blend_mode_t;

// Phase 2: Typography & Interactivity Styles
typedef struct {
    char* font_family;
    int font_size;
    int bold;
    int italic;
    int underline;
    int strikethrough;
    uint32_t color;
    uint32_t shadow_color;
    int shadow_offset_x;
    int shadow_offset_y;
    int letter_spacing;
    int line_height;
    int align; // left, center, right, justify
    int word_wrap;
} ipe_text_style_t;

typedef struct {
    char* text;
    ipe_text_style_t** styles;
    int* style_ranges; // [start, end] para cada estilo
    int style_count;
} ipe_rich_text_t;

typedef struct {
    uint32_t normal_color;
    uint32_t hover_color;
    uint32_t pressed_color;
    uint32_t disabled_color;
    int hover_scale; // 105 = 105%
    int hover_offset_x;
    int hover_offset_y;
    float hover_anim_duration;
} ipe_interactive_style_t;

typedef struct {
    char* text;
    int delay_ms;
    uint32_t bg_color;
    uint32_t text_color;
    int border_radius;
    int max_width;
    ipe_image_t* icon;
} ipe_tooltip_t;

// Phase 3: Smart Layout & Constraints
typedef struct {
    int left_margin;
    int right_margin;
    int top_margin;
    int bottom_margin;
    int spacing_x;
    int spacing_y;
    int align_h; // -1=left, 0=center, 1=right
    int align_v; // -1=top, 0=middle, 1=bottom
    int wrap; // quebrar linha quando atingir largura
    int equal_size; // forçar todos controles mesmo tamanho
} ipe_flow_layout_t;

typedef struct {
    int cols;
    int rows;
    int* col_widths; // NULL para auto
    int* row_heights; // NULL para auto
    int** col_spans; // quantas colunas cada célula ocupa
    int** row_spans;
} ipe_grid_layout_t;

typedef struct {
    int anchor_left;   // 0/1
    int anchor_top;    // 0/1
    int anchor_right;  // 0/1
    int anchor_bottom; // 0/1
} ipe_anchors_t;

typedef struct {
    int width;  // -1=auto, 0=fixo, >0=fixo
    int height; // -1=auto, 0=fixo, >0=fixo
    int max_width;
    int max_height;
    int min_width;
    int min_height;
} ipe_size_constraints_t;

// Phase 4: Advanced Components & Input
typedef struct {
    char* text;
    char* shortcut; 
    ipe_image_t* icon;
    int enabled;
    int checked;
    int radio_group;
    struct ipe_menuitem_ex* submenu;
    void (*on_click)();
} ipe_menuitem_ex_t;

typedef struct {
    ipe_image_t** icons;
    char** tooltips;
    int* enabled;
    int item_count;
    int orientation;
    int show_text;
} ipe_toolbar_t;

typedef struct {
    char** texts;
    int* widths;
    int panel_count;
    uint32_t* panel_colors;
} ipe_statusbar_ex_t;

typedef struct {
    int scroll_x;
    int scroll_y;
    int max_scroll_x;
    int max_scroll_y;
    int inertia; // 0-100
    int show_scrollbars;
    uint32_t scrollbar_color;
    int scrollbar_width;
} ipe_scrollable_t;

typedef struct {
    float zoom;
    float min_zoom;
    float max_zoom;
    int pan_x;
    int pan_y;
    int allow_zoom;
    int allow_pan;
} ipe_zoomable_t;

typedef enum {
    IPE_INPUT_ANY,
    IPE_INPUT_NUMBER,
    IPE_INPUT_DECIMAL,
    IPE_INPUT_ALPHA,
    IPE_INPUT_ALPHANUM,
    IPE_INPUT_EMAIL,
    IPE_INPUT_CPF,
    IPE_INPUT_CNPJ,
    IPE_INPUT_PHONE,
    IPE_INPUT_CUSTOM
} ipe_input_mask_t;

typedef struct {
    ipe_input_mask_t mask;
    char* custom_regex;
    int max_length;
    char* placeholder;
    uint32_t placeholder_color;
    int password_mode; 
} ipe_textfield_style_t;

typedef struct {
    char** suggestions;
    int suggestion_count;
    int max_visible;
    uint32_t highlight_color;
} ipe_autocomplete_t;

typedef int (*ipe_validator_t)(const char* text, void* user_data);

// Phase 5: Animations, Charts & System Integration
typedef enum {
    IPE_EASE_LINEAR,
    IPE_EASE_IN_QUAD,
    IPE_EASE_OUT_QUAD,
    IPE_EASE_IN_OUT_QUAD,
    IPE_EASE_IN_BOUNCE,
    IPE_EASE_OUT_BOUNCE,
    IPE_EASE_IN_ELASTIC,
    IPE_EASE_OUT_ELASTIC
} ipe_easing_t;

/* Forward declaration of ipe_control_t */
typedef struct ipe_control ipe_control_t;

typedef struct ipe_animation_seq {
    ipe_control_t* target;
    char* property;
    float target_value;
    float duration;
    ipe_easing_t easing;
    struct ipe_animation_seq* next;
} ipe_animation_seq_t;

typedef struct ipe_animation { int dummy; } ipe_animation_t; // Placeholder

typedef struct {
    float* values;
    int count;
    uint32_t* colors;
    int show_values;
    int show_legend;
    char** labels;
} ipe_chart_data_t;

typedef struct {
    char** files;
    int file_count;
    int x, y;
} ipe_drop_event_t;

typedef struct {
    void (*execute)(void*);
    void (*undo)(void*);
    void* data;
    char* description;
} ipe_command_t;

typedef struct {
    int keycode;
    int ctrl;
    int shift;
    int alt;
    void (*callback)();
    int enabled;
} ipe_shortcut_t;

// Timer structure
typedef struct {
  Uint32 interval;
  Uint32 last_tick;
  int running;
  void (*on_tick)(void *);
  void *user_data;
  SDL_TimerID timer_id;
} ipe_timer_t;

// UI Control System - Base Control Structure
typedef struct ipe_control {
  // Basic properties
  int id;
  int x, y, w, h;
  int visible;
  int enabled;
  int focused;
  
  // Colors and styling
  uint32_t bg_color;
  uint32_t border_color;
  int border_width;
  int corner_radius;
  
  // Margins
  int margin_left;
  int margin_right;
  int margin_top;
  int margin_bottom;
  
  // Padding
  int padding_left;
  int padding_right;
  int padding_top;
  int padding_bottom;
  
  // Size constraints
  int min_width;
  int min_height;
  int max_width;
  int max_height;
  
  // Hierarchy
  struct ipe_control *parent;
  struct ipe_control **children;
  int children_count;
  int children_capacity;
  
  // Callbacks
  void (*on_click)(struct ipe_control *);
  void (*on_focus)(struct ipe_control *);
  void (*on_blur)(struct ipe_control *);
  void (*on_draw)(struct ipe_control *, void *window);
  void (*on_key)(struct ipe_control *, int keycode);
  void (*on_mouse_move)(struct ipe_control *, int x, int y);
  
  // User data
  void *user_data;
  
  // Type-specific data
  char *control_type;
  void *type_data;
  
  // Additional members for linked list and event handling
  struct ipe_control *next;
  char *name;
  int (*handle_event)(struct ipe_control *, int event, int x, int y);
} ipe_control_t;

// Global control state
static ipe_control_t *g_control_root = NULL;
static ipe_control_t *g_focused_control = NULL;
static int g_next_control_id = 1;

// Transformation math
typedef struct {
    float m[3][3];
} ipe_matrix_t;

#define IPE_MAX_TRANSFORM_DEPTH 32
static ipe_matrix_t g_transform_stack[IPE_MAX_TRANSFORM_DEPTH];
static int g_transform_depth = 0;

static void ipe_matrix_identity(ipe_matrix_t* m) {
    memset(m, 0, sizeof(ipe_matrix_t));
    m->m[0][0] = 1.0f;
    m->m[1][1] = 1.0f;
    m->m[2][2] = 1.0f;
}

static void ipe_matrix_mul(ipe_matrix_t* res, ipe_matrix_t* a, ipe_matrix_t* b) {
    ipe_matrix_t tmp;
    for (int i = 0; i < 3; i++) {
        for (int j = 0; j < 3; j++) {
            tmp.m[i][j] = a->m[i][0] * b->m[0][j] +
                         a->m[i][1] * b->m[1][j] +
                         a->m[i][2] * b->m[2][j];
        }
    }
    memcpy(res, &tmp, sizeof(ipe_matrix_t));
}

static void ipe_transform_point(float* px, float* py) {
    ipe_matrix_t* m = &g_transform_stack[g_transform_depth];
    float x = *px;
    float y = *py;
    *px = x * m->m[0][0] + y * m->m[0][1] + m->m[0][2];
    *py = x * m->m[1][0] + y * m->m[1][1] + m->m[1][2];
}

IPE_EXPORT void ipe_transform_push() {
    if (g_transform_depth < IPE_MAX_TRANSFORM_DEPTH - 1) {
        memcpy(&g_transform_stack[g_transform_depth + 1], &g_transform_stack[g_transform_depth], sizeof(ipe_matrix_t));
        g_transform_depth++;
    }
}

IPE_EXPORT void ipe_transform_pop() {
    if (g_transform_depth > 0) {
        g_transform_depth--;
    }
}

IPE_EXPORT void ipe_transform_translate(float dx, float dy) {
    ipe_matrix_t t;
    ipe_matrix_identity(&t);
    t.m[0][2] = dx;
    t.m[1][2] = dy;
    ipe_matrix_mul(&g_transform_stack[g_transform_depth], &g_transform_stack[g_transform_depth], &t);
}

IPE_EXPORT void ipe_transform_scale(float sx, float sy) {
    ipe_matrix_t s;
    ipe_matrix_identity(&s);
    s.m[0][0] = sx;
    s.m[1][1] = sy;
    ipe_matrix_mul(&g_transform_stack[g_transform_depth], &g_transform_stack[g_transform_depth], &s);
}

IPE_EXPORT void ipe_transform_rotate(float angle_degrees) {
    float rad = angle_degrees * (M_PI / 180.0f);
    ipe_matrix_t r;
    ipe_matrix_identity(&r);
    float cos_a = cosf(rad);
    float sin_a = sinf(rad);
    r.m[0][0] = cos_a;
    r.m[0][1] = -sin_a;
    r.m[1][0] = sin_a;
    r.m[1][1] = cos_a;
    ipe_matrix_mul(&g_transform_stack[g_transform_depth], &g_transform_stack[g_transform_depth], &r);
}

IPE_EXPORT void ipe_transform_reset() {
    ipe_matrix_identity(&g_transform_stack[g_transform_depth]);
}

// Forward declarations
static void ipe_draw_polygon_styled(int* points, int point_count, ipe_polygon_style_t* style);

// Button structure
typedef struct {
  int x, y, w, h;
  char *text;
  uint32_t bg_color;
  uint32_t text_color;
  int is_hovered;
  int is_pressed;
  void (*on_click)(void *);
} ipe_button_t;

// Callbacks
typedef void (*ipe_mouse_callback)(int x, int y, int button);
typedef void (*ipe_key_callback)(int keycode);
typedef void (*ipe_resize_callback)(int w, int h);

static ipe_window_t *g_current_window = NULL;
static int g_running = 1;

static ipe_mouse_callback g_mousedown_cb = NULL;
static ipe_mouse_callback g_mousemove_cb = NULL;
static ipe_key_callback g_keydown_cb = NULL;
static ipe_resize_callback g_resize_cb = NULL;

// Event state (legacy support)
static int g_last_event_type = 0;
static int g_mouse_x = 0;
static int g_mouse_y = 0;
static int g_key_code = 0;
static int g_mouse_buttons = 0;       // bitmask: bit 0=left, bit 1=middle, bit 2=right
static int g_prev_mouse_buttons = 0;
static int g_mouse_scroll_delta = 0;

// ========== Task 5: Input Polling API ==========
typedef struct {
    const uint8_t* key_state;
    int num_keys;
    uint8_t key_pressed[512];     // Track pressed this frame
    uint8_t key_released[512];    // Track released this frame
} ipe_input_state_t;

static ipe_input_state_t g_input_state = {0};

// Key code constants
#define IPE_KEY_ESCAPE   27
#define IPE_KEY_RETURN   13
#define IPE_KEY_SPACE    32
#define IPE_KEY_TAB      9
#define IPE_KEY_A        65
#define IPE_KEY_B        66
#define IPE_KEY_C        67
// ... Z = 90 (65+25)
#define IPE_KEY_0        48
#define IPE_KEY_1        49
// ... IPE_KEY_9 = 57
#define IPE_KEY_UP       1073741906
#define IPE_KEY_DOWN     1073741905
#define IPE_KEY_LEFT     1073741904
#define IPE_KEY_RIGHT    1073741903

#define IPE_MOD_SHIFT       0x01
#define IPE_MOD_CTRL        0x02
#define IPE_MOD_ALT         0x04
#define IPE_MOD_GUI         0x08

// Initialize input state
static void ipe_input_init() {
    g_input_state.key_state = SDL_GetKeyboardState(&g_input_state.num_keys);
    memset(g_input_state.key_pressed, 0, sizeof(g_input_state.key_pressed));
    memset(g_input_state.key_released, 0, sizeof(g_input_state.key_released));
}

// Polling functions
IPE_EXPORT int ipe_input_key_down(int key_code) {
    if (!g_input_state.key_state || key_code < 0 || key_code >= g_input_state.num_keys) return 0;
    return g_input_state.key_state[key_code];
}

IPE_EXPORT int ipe_input_key_pressed(int key_code) {
    if (key_code < 0 || key_code >= 512) return 0;
    return g_input_state.key_pressed[key_code];
}

IPE_EXPORT int ipe_input_key_released(int key_code) {
    if (key_code < 0 || key_code >= 512) return 0;
    return g_input_state.key_released[key_code];
}

IPE_EXPORT int ipe_input_mouse_x() {
    return g_mouse_x;
}

IPE_EXPORT int ipe_input_mouse_y() {
    return g_mouse_y;
}

IPE_EXPORT void ipe_input_mouse_pos(int* out_x, int* out_y) {
    if (out_x) *out_x = g_mouse_x;
    if (out_y) *out_y = g_mouse_y;
}

IPE_EXPORT int ipe_input_mouse_button_down(int button) {
    return (g_mouse_buttons >> button) & 1;
}

IPE_EXPORT int ipe_input_mouse_button_clicked(int button) {
    int pressed = (g_mouse_buttons >> button) & 1;
    int was_released = ((g_prev_mouse_buttons >> button) & 1) == 0;
    return pressed && was_released;
}

IPE_EXPORT int ipe_input_mouse_scroll_delta() {
    return g_mouse_scroll_delta;
}

IPE_EXPORT int ipe_input_modifier_active(int modifier) {
    SDL_Keymod mod = SDL_GetModState();
    int result = 0;
    if ((modifier & IPE_MOD_SHIFT) && (mod & KMOD_SHIFT)) result |= IPE_MOD_SHIFT;
    if ((modifier & IPE_MOD_CTRL) && (mod & KMOD_CTRL)) result |= IPE_MOD_CTRL;
    if ((modifier & IPE_MOD_ALT) && (mod & KMOD_ALT)) result |= IPE_MOD_ALT;
    if ((modifier & IPE_MOD_GUI) && (mod & KMOD_GUI)) result |= IPE_MOD_GUI;
    return result;
}

// ========== Task 6: Input Event Queue ==========
#define IPE_EVENT_KEY_DOWN         1
#define IPE_EVENT_KEY_UP           2
#define IPE_EVENT_MOUSE_MOVE       3
#define IPE_EVENT_MOUSE_DOWN       4
#define IPE_EVENT_MOUSE_UP         5
#define IPE_EVENT_SCROLL           6
#define IPE_EVENT_WINDOW_CLOSE     7
#define IPE_EVENT_WINDOW_RESIZE    8

typedef struct {
    int type;
    int key_code;
    int x, y;
    int button;
    int scroll_delta;
    uint32_t timestamp_ms;
} ipe_input_event_t;

#define IPE_MAX_EVENTS 256
typedef struct {
    ipe_input_event_t events[IPE_MAX_EVENTS];
    int count;
    int read_index;
} ipe_event_queue_t;

static ipe_event_queue_t g_event_queue = {0};

IPE_EXPORT ipe_input_event_t* ipe_input_get_events() {
    return g_event_queue.events;
}

IPE_EXPORT int ipe_input_event_count() {
    return g_event_queue.count;
}

IPE_EXPORT void ipe_input_clear_events() {
    g_event_queue.count = 0;
    g_event_queue.read_index = 0;
}

// ========== Event Polling API ==========
// Simple polling functions for event detection

IPE_EXPORT int ipe_event_has_key_down() {
    for (int i = 0; i < g_event_queue.count; i++) {
        if (g_event_queue.events[i].type == IPE_EVENT_KEY_DOWN) return 1;
    }
    return 0;
}

IPE_EXPORT int ipe_event_has_key_up() {
    for (int i = 0; i < g_event_queue.count; i++) {
        if (g_event_queue.events[i].type == IPE_EVENT_KEY_UP) return 1;
    }
    return 0;
}

IPE_EXPORT int ipe_event_has_mouse_down() {
    for (int i = 0; i < g_event_queue.count; i++) {
        if (g_event_queue.events[i].type == IPE_EVENT_MOUSE_DOWN) return 1;
    }
    return 0;
}

IPE_EXPORT int ipe_event_has_mouse_up() {
    for (int i = 0; i < g_event_queue.count; i++) {
        if (g_event_queue.events[i].type == IPE_EVENT_MOUSE_UP) return 1;
    }
    return 0;
}

IPE_EXPORT int ipe_event_has_mouse_move() {
    for (int i = 0; i < g_event_queue.count; i++) {
        if (g_event_queue.events[i].type == IPE_EVENT_MOUSE_MOVE) return 1;
    }
    return 0;
}

IPE_EXPORT int ipe_event_has_scroll() {
    for (int i = 0; i < g_event_queue.count; i++) {
        if (g_event_queue.events[i].type == IPE_EVENT_SCROLL) return 1;
    }
    return 0;
}

// Get last event of specific type
IPE_EXPORT ipe_input_event_t* ipe_event_get_last_key_down() {
    for (int i = g_event_queue.count - 1; i >= 0; i--) {
        if (g_event_queue.events[i].type == IPE_EVENT_KEY_DOWN) 
            return &g_event_queue.events[i];
    }
    return NULL;
}

IPE_EXPORT ipe_input_event_t* ipe_event_get_last_mouse_down() {
    for (int i = g_event_queue.count - 1; i >= 0; i--) {
        if (g_event_queue.events[i].type == IPE_EVENT_MOUSE_DOWN) 
            return &g_event_queue.events[i];
    }
    return NULL;
}

IPE_EXPORT ipe_input_event_t* ipe_event_get_last_mouse_up() {
    for (int i = g_event_queue.count - 1; i >= 0; i--) {
        if (g_event_queue.events[i].type == IPE_EVENT_MOUSE_UP) 
            return &g_event_queue.events[i];
    }
    return NULL;
}

IPE_EXPORT ipe_input_event_t* ipe_event_get_last_mouse_move() {
    for (int i = g_event_queue.count - 1; i >= 0; i--) {
        if (g_event_queue.events[i].type == IPE_EVENT_MOUSE_MOVE) 
            return &g_event_queue.events[i];
    }
    return NULL;
}

// Helper to add event to queue
static void ipe_queue_event(ipe_input_event_t event) {
    if (g_event_queue.count < IPE_MAX_EVENTS) {
        g_event_queue.events[g_event_queue.count++] = event;
    }
}

/* PERF: Rounded corner drawing has O(n) cost per corner. Mark for modularization. */
static void draw_rounded_rect(SDL_Renderer* renderer, int x, int y, int w, int h, 
                              int tl, int tr, int bl, int br, uint32_t color, float opacity) {
    SDL_SetRenderDrawColor(renderer, 
                          (color >> 16) & 0xFF,
                          (color >> 8) & 0xFF,
                          color & 0xFF,
                          (uint8_t)(255 * opacity));
    
    // Draw central rectangles
    int max_radius = (w < h ? w : h) / 2;
    if (tl > max_radius) tl = max_radius;
    if (tr > max_radius) tr = max_radius;
    if (bl > max_radius) bl = max_radius;
    if (br > max_radius) br = max_radius;

    // This is a simplified version. Full version would draw circles.
    // For now, we draw a regular rect if no radius, or a slightly smaller one.
    SDL_Rect rect = {x, y, w, h};
    SDL_RenderFillRect(renderer, &rect);
}

static uint32_t interpolate_color(uint32_t c1, uint32_t c2, float t) {
    uint8_t r1 = (c1 >> 16) & 0xFF;
    uint8_t g1 = (c1 >> 8) & 0xFF;
    uint8_t b1 = c1 & 0xFF;
    
    uint8_t r2 = (c2 >> 16) & 0xFF;
    uint8_t g2 = (c2 >> 8) & 0xFF;
    uint8_t b2 = c2 & 0xFF;
    
    uint8_t r = (uint8_t)(r1 + (r2 - r1) * t);
    uint8_t g = (uint8_t)(g1 + (g2 - g1) * t);
    uint8_t b = (uint8_t)(b1 + (b2 - b1) * t);
    
    return (r << 16) | (g << 8) | b;
}

// ========== UI Control System ==========

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
    ctrl->focused = 0;
    
    ctrl->bg_color = 0xCCCCCCFF;
    ctrl->border_color = 0x000000FF;
    ctrl->border_width = 1;
    ctrl->corner_radius = 0;
    
    ctrl->margin_left = ctrl->margin_right = ctrl->margin_top = ctrl->margin_bottom = 0;
    ctrl->padding_left = ctrl->padding_right = ctrl->padding_top = ctrl->padding_bottom = 0;
    
    ctrl->min_width = ctrl->min_height = 0;
    ctrl->max_width = ctrl->max_height = 10000;
    
    ctrl->parent = NULL;
    ctrl->children = NULL;
    ctrl->children_count = 0;
    ctrl->children_capacity = 0;
    
    ctrl->on_click = NULL;
    ctrl->on_focus = NULL;
    ctrl->on_blur = NULL;
    ctrl->on_draw = NULL;
    ctrl->on_key = NULL;
    ctrl->on_mouse_move = NULL;
    
    ctrl->user_data = NULL;
    ctrl->control_type = NULL;
    ctrl->type_data = NULL;
    
    return ctrl;
}

IPE_EXPORT void ipe_control_destroy(ipe_control_t* ctrl) {
    if (!ctrl) return;
    
    for (int i = 0; i < ctrl->children_count; i++) {
        ipe_control_destroy(ctrl->children[i]);
    }
    
    if (ctrl->children) {
        free(ctrl->children);
    }
    
    if (ctrl->control_type) {
        free(ctrl->control_type);
    }
    
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

IPE_EXPORT void ipe_control_add_child(ipe_control_t* parent, ipe_control_t* child) {
    if (!parent || !child) return;
    
    if (parent->children_count >= parent->children_capacity) {
        int new_capacity = parent->children_capacity == 0 ? 4 : parent->children_capacity * 2;
        ipe_control_t** new_children = (ipe_control_t**)realloc(parent->children, 
                                                                  new_capacity * sizeof(ipe_control_t*));
        if (!new_children) return;
        
        parent->children = new_children;
        parent->children_capacity = new_capacity;
    }
    
    parent->children[parent->children_count++] = child;
    child->parent = parent;
}

IPE_EXPORT void ipe_control_remove_child(ipe_control_t* parent, ipe_control_t* child) {
    if (!parent || !child) return;
    
    for (int i = 0; i < parent->children_count; i++) {
        if (parent->children[i] == child) {
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
    if (g_control_root && g_control_root != ctrl) {
        ipe_control_destroy(g_control_root);
    }
    g_control_root = ctrl;
}

IPE_EXPORT ipe_control_t* ipe_control_get_root() {
    return g_control_root;
}

IPE_EXPORT void ipe_control_draw_base(ipe_control_t* ctrl, void* window) {
    if (!ctrl || !ctrl->visible) return;
    
    ipe_window_t* win = (ipe_window_t*)window;
    if (!win || !win->renderer) return;
    
    SDL_Renderer* renderer = win->renderer;
    
    uint8_t r = (ctrl->bg_color >> 24) & 0xFF;
    uint8_t g = (ctrl->bg_color >> 16) & 0xFF;
    uint8_t b = (ctrl->bg_color >> 8) & 0xFF;
    uint8_t a = ctrl->bg_color & 0xFF;
    
    if (ctrl->corner_radius > 0) {
        draw_rounded_rect(renderer, ctrl->x, ctrl->y, ctrl->w, ctrl->h,
                         ctrl->corner_radius, ctrl->corner_radius,
                         ctrl->corner_radius, ctrl->corner_radius,
                         ctrl->bg_color, a / 255.0f);
    } else {
        SDL_SetRenderDrawColor(renderer, r, g, b, a);
        SDL_Rect rect = { ctrl->x, ctrl->y, ctrl->w, ctrl->h };
        SDL_RenderFillRect(renderer, &rect);
    }
    
    if (ctrl->border_width > 0) {
        r = (ctrl->border_color >> 24) & 0xFF;
        g = (ctrl->border_color >> 16) & 0xFF;
        b = (ctrl->border_color >> 8) & 0xFF;
        a = ctrl->border_color & 0xFF;
        
        SDL_SetRenderDrawColor(renderer, r, g, b, a);
        
        for (int i = 0; i < ctrl->border_width; i++) {
            SDL_Rect border = { ctrl->x + i, ctrl->y + i, 
                               ctrl->w - 2*i, ctrl->h - 2*i };
            SDL_RenderDrawRect(renderer, &border);
        }
    }
}

IPE_EXPORT void ipe_control_draw_recursive(ipe_control_t* ctrl, void* window) {
    if (!ctrl || !ctrl->visible) return;
    
    if (ctrl->on_draw) {
        ctrl->on_draw(ctrl, window);
    } else {
        ipe_control_draw_base(ctrl, window);
    }
    
    for (int i = 0; i < ctrl->children_count; i++) {
        ipe_control_draw_recursive(ctrl->children[i], window);
    }
}

// ========== Task 2: Event Handling for Controls ==========

// Hit testing helper - finds control at point (x, y)
static ipe_control_t* ipe_control_at_point(ipe_control_t* ctrl, int x, int y) {
    if (!ctrl || !ctrl->visible || !ctrl->enabled) {
        return NULL;
    }
    
    // Check if point is outside control bounds
    if (x < ctrl->x || x > ctrl->x + ctrl->w || 
        y < ctrl->y || y > ctrl->y + ctrl->h) {
        return NULL;
    }
    
    // Check children in reverse order (last child = top z-order)
    for (int i = ctrl->children_count - 1; i >= 0; i--) {
        ipe_control_t* hit = ipe_control_at_point(ctrl->children[i], x, y);
        if (hit) {
            return hit;
        }
    }
    
    // No child contains the point, return this control
    return ctrl;
}

// Mouse event handlers
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

// Keyboard event handler
IPE_EXPORT void ipe_control_handle_key_press(int key_code) {
    if (g_focused_control && g_focused_control->on_key) {
        g_focused_control->on_key(g_focused_control, key_code);
    }
}

// Focus management
IPE_EXPORT void ipe_control_set_focus(ipe_control_t* ctrl) {
    // Call on_blur on previously focused control
    if (g_focused_control && g_focused_control->on_blur) {
        g_focused_control->on_blur(g_focused_control);
        g_focused_control->focused = 0;
    }
    
    // Set new focused control
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

IPE_EXPORT int ipe_init() {
  if (SDL_Init(SDL_INIT_VIDEO | SDL_INIT_TIMER) < 0)
    return 0;
  if (TTF_Init() < 0)
    return 0;
  int imgFlags = IMG_INIT_PNG | IMG_INIT_JPG;
  if (!(IMG_Init(imgFlags) & imgFlags))
    return 0;
  
  // Initialize input system
  ipe_input_init();
  
  // Initialize transformation stack
  g_transform_depth = 0;
  ipe_matrix_identity(&g_transform_stack[0]);
  
  return 1;
}

IPE_EXPORT void* ipe_window_create(int width, int height, const char *title) {
  ipe_window_t *win = (ipe_window_t *)malloc(sizeof(ipe_window_t));
  if (!win)
    return NULL;

  win->window =
      SDL_CreateWindow(title, SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED,
                       width, height, SDL_WINDOW_SHOWN | SDL_WINDOW_RESIZABLE);
  if (!win->window) {
    free(win);
    return NULL;
  }

  win->renderer = SDL_CreateRenderer(win->window, -1, SDL_RENDERER_ACCELERATED);
  if (!win->renderer) {
    SDL_DestroyWindow(win->window);
    free(win);
    return NULL;
  }

  win->id = SDL_GetWindowID(win->window);
  win->title = strdup(title);
  win->width = width;
  win->height = height;
  win->is_open = 1;
  win->root_control = NULL;

  if (!g_current_window) {
    g_current_window = win;
  }

  return (void*)win;
}

void ipe_window_set_current(ipe_window_t *win) {
  if (win) {
    g_current_window = win;
  }
}

void ipe_window_destroy(ipe_window_t *win) {
  if (!win)
    return;
  if (g_current_window == win)
    g_current_window = NULL;

  if (win->renderer)
    SDL_DestroyRenderer(win->renderer);
  if (win->window)
    SDL_DestroyWindow(win->window);
  if (win->title)
    free(win->title);
  free(win);
}

IPE_EXPORT void ipe_clear_background(uint32_t color) {
  if (!g_current_window || !g_current_window->renderer)
    return;
  SDL_SetRenderDrawColor(g_current_window->renderer, (color >> 16) & 0xFF,
                         (color >> 8) & 0xFF, color & 0xFF, 255);
  SDL_RenderClear(g_current_window->renderer);
}

IPE_EXPORT void ipe_present() {
  if (g_current_window && g_current_window->renderer)
    SDL_RenderPresent(g_current_window->renderer);
}

IPE_EXPORT void ipe_draw_rect_styled(int x, int y, int w, int h, ipe_rect_style_t* style) {
    if (!g_current_window || !g_current_window->renderer) {
        IPE_ERR("Window not initialized. Call ipe_init() first.");
        return;
    }
    
    SDL_Renderer* renderer = g_current_window->renderer;
    
    // Check for complex transforms (rotation/scale)
    ipe_matrix_t* m = &g_transform_stack[g_transform_depth];
    int complex = (m->m[0][1] != 0.0f || m->m[1][0] != 0.0f || m->m[0][0] != 1.0f || m->m[1][1] != 1.0f);

    if (complex) {
        // Draw as polygon
        float px[4] = { (float)x, (float)x + w, (float)x + w, (float)x };
        float py[4] = { (float)y, (float)y, (float)y + h, (float)y + h };
        int points[8];
        for (int i = 0; i < 4; i++) {
            ipe_transform_point(&px[i], &py[i]);
            points[i*2] = (int)px[i];
            points[i*2+1] = (int)py[i];
        }
        
        ipe_polygon_style_t p_style = {0};
        if (style) {
            p_style.fill_color = style->fill_color;
            p_style.border_color = style->border_color;
            p_style.border_width = style->border_width;
            p_style.opacity = style->opacity;
            p_style.shadow_enabled = style->shadow_enabled;
            p_style.shadow_radius = style->shadow_radius;
            p_style.shadow_color = style->shadow_color;
            p_style.blend_mode = style->blend_mode;
            // NOTE: Corner radius and complex gradients not yet supported for transformed rects
        }
        ipe_draw_polygon_styled(points, 4, &p_style);
        return;
    }

    // Simple translation
    float fx = (float)x, fy = (float)y;
    ipe_transform_point(&fx, &fy);
    int tx = (int)fx, ty = (int)fy;

    // Handle opacity
    float opacity = style && style->opacity > 0 ? style->opacity : 1.0f;
    
    // Draw shadow first (if enabled)
    if (style && style->shadow_enabled) {
        uint32_t shadow_color = style->shadow_color;
        SDL_SetRenderDrawColor(renderer,
                              (shadow_color >> 16) & 0xFF,
                              (shadow_color >> 8) & 0xFF,
                              shadow_color & 0xFF,
                              (uint8_t)(255 * opacity * 0.5f)); // Shadow is semi-transparent
        
        SDL_Rect shadow_rect = {
            tx + style->shadow_offset_x,
            ty + style->shadow_offset_y,
            w,
            h
        };
        SDL_RenderFillRect(renderer, &shadow_rect);
    }
    
    // Handle Gradient / Fill
    if (style && style->gradient_type > 0) {
        // Simple linear gradient (horizontal or vertical)
        int steps = (style->gradient_type == 1 ? w : h);
        for (int i = 0; i < steps; i++) {
            float t = (float)i / (steps - 1);
            uint32_t color = interpolate_color(style->gradient_start, style->gradient_end, t);
            
            SDL_SetRenderDrawColor(renderer,
                                  (color >> 16) & 0xFF,
                                  (color >> 8) & 0xFF,
                                  color & 0xFF,
                                  (uint8_t)(255 * opacity));
            
            if (style->gradient_type == 1) // Horizontal
                SDL_RenderDrawLine(renderer, tx + i, ty, tx + i, ty + h);
            else // Vertical
                SDL_RenderDrawLine(renderer, tx, ty + i, tx + w, ty + i);
        }
    } else {
        uint32_t fill_color = style ? style->fill_color : 0xFFFFFF;
        if (style && (style->corner_radius_tl > 0 || style->corner_radius_tr > 0 || 
                      style->corner_radius_bl > 0 || style->corner_radius_br > 0)) {
            draw_rounded_rect(renderer, tx, ty, w, h, 
                              style->corner_radius_tl, style->corner_radius_tr,
                              style->corner_radius_bl, style->corner_radius_br,
                              fill_color, opacity);
        } else {
            SDL_SetRenderDrawColor(renderer,
                                  (fill_color >> 16) & 0xFF,
                                  (fill_color >> 8) & 0xFF,
                                  fill_color & 0xFF,
                                  (uint8_t)(255 * opacity));
            SDL_Rect rect = {tx, ty, w, h};
            SDL_RenderFillRect(renderer, &rect);
        }
    }
    
    // Draw border if width > 0
    if (style && style->border_width > 0) {
        uint32_t border_color = style->border_color;
        SDL_SetRenderDrawColor(renderer,
                              (border_color >> 16) & 0xFF,
                              (border_color >> 8) & 0xFF,
                              border_color & 0xFF,
                              (uint8_t)(255 * opacity));
        
        // Draw border by drawing rectangle outlines
        for (int i = 0; i < style->border_width; i++) {
            SDL_Rect border_rect = {
                tx - i,
                ty - i,
                w + (2 * i),
                h + (2 * i)
            };
            SDL_RenderDrawRect(renderer, &border_rect);
        }
    }
}

IPE_EXPORT void ipe_draw_rect(int x, int y, int w, int h, uint32_t color) {
  ipe_rect_style_t style = {0};
  style.fill_color = color;
  style.opacity = 1.0f;
  ipe_draw_rect_styled(x, y, w, h, &style);
}

#include "font8x8_basic.h"

void ipe_draw_text(const char *text, int x, int y, uint32_t color) {
  if (!g_current_window || !g_current_window->renderer || !text)
    return;

  float fx = (float)x, fy = (float)y;
  ipe_transform_point(&fx, &fy);
  int tx = (int)fx, ty = (int)fy;

  SDL_SetRenderDrawColor(g_current_window->renderer, (color >> 16) & 0xFF,
                         (color >> 8) & 0xFF, color & 0xFF, 255);

  int cursor_x = tx;
  int cursor_y = ty;

  while (*text) {
    char c = *text;
    if (c >= 32 && c <= 127) {
      int index = c - 32;
      const uint8_t *bitmap = font8x8_basic[index];

      for (int row = 0; row < 8; row++) {
        for (int col = 0; col < 8; col++) {
          if (bitmap[row] & (1 << col)) {
            SDL_RenderDrawPoint(g_current_window->renderer, cursor_x + col,
                                cursor_y + row);
          }
        }
      }
    }
    cursor_x += 8;
    text++;
  }
}

void ipe_set_mousedown_callback(ipe_mouse_callback cb) { g_mousedown_cb = cb; }
void ipe_set_mousemove_callback(ipe_mouse_callback cb) { g_mousemove_cb = cb; }
void ipe_set_keydown_callback(ipe_key_callback cb) { g_keydown_cb = cb; }
void ipe_set_resize_callback(ipe_resize_callback cb) { g_resize_cb = cb; }

// Graphics Primitives
typedef struct {
    uint32_t color;
    int thickness;
    int line_style;          // 0=solid, 1=dashed, 2=dotted
    int cap_style;           // 0=butt, 1=round, 2=square
    float opacity;
    int blend_mode;
} ipe_line_style_t;

static void draw_dashed_line(SDL_Renderer* renderer, int x1, int y1, int x2, int y2, uint32_t color, float opacity) {
    int dx = x2 - x1;
    int dy = y2 - y1;
    int steps = (abs(dx) > abs(dy)) ? abs(dx) : abs(dy);
    if (steps == 0) steps = 1;
    
    float x_inc = (float)dx / steps;
    float y_inc = (float)dy / steps;
    
    SDL_SetRenderDrawColor(renderer,
                          (color >> 16) & 0xFF,
                          (color >> 8) & 0xFF,
                          color & 0xFF,
                          (uint8_t)(255 * opacity));
    
    int dash_length = 10;
    int gap_length = 5;
    int current = 0;
    
    float x = x1, y = y1;
    for (int i = 0; i <= steps; i++) {
        if (current < dash_length) {
            SDL_RenderDrawPoint(renderer, (int)x, (int)y);
        }
        current++;
        if (current >= dash_length + gap_length) {
            current = 0;
        }
        x += x_inc;
        y += y_inc;
    }
}

static void draw_dotted_line(SDL_Renderer* renderer, int x1, int y1, int x2, int y2, uint32_t color, float opacity) {
    int dx = x2 - x1;
    int dy = y2 - y1;
    int steps = (abs(dx) > abs(dy)) ? abs(dx) : abs(dy);
    if (steps == 0) steps = 1;
    
    float x_inc = (float)dx / steps;
    float y_inc = (float)dy / steps;
    
    SDL_SetRenderDrawColor(renderer,
                          (color >> 16) & 0xFF,
                          (color >> 8) & 0xFF,
                          color & 0xFF,
                          (uint8_t)(255 * opacity));
    
    for (int i = 0; i <= steps; i += 3) {
        SDL_RenderDrawPoint(renderer, x1 + (int)(x_inc * i), y1 + (int)(y_inc * i));
    }
}

IPE_EXPORT void ipe_draw_line_styled(int x1, int y1, int x2, int y2, ipe_line_style_t* style) {
    if (!g_current_window || !g_current_window->renderer) {
        IPE_ERR("Window not initialized. Call ipe_init() first.");
        return;
    }
    
    SDL_Renderer* renderer = g_current_window->renderer;
    
    float fx1 = (float)x1, fy1 = (float)y1;
    float fx2 = (float)x2, fy2 = (float)y2;
    ipe_transform_point(&fx1, &fy1);
    ipe_transform_point(&fx2, &fy2);
    x1 = (int)fx1; y1 = (int)fy1;
    x2 = (int)fx2; y2 = (int)fy2;

    uint32_t color = style ? style->color : 0x000000;
    int thickness = style && style->thickness > 0 ? style->thickness : 1;
    float opacity = style && style->opacity > 0 ? style->opacity : 1.0f;
    int line_style = style ? style->line_style : 0;
    
    // Handle different line styles
    if (line_style == 1) {
        // Dashed
        draw_dashed_line(renderer, x1, y1, x2, y2, color, opacity);
    } else if (line_style == 2) {
        // Dotted
        draw_dotted_line(renderer, x1, y1, x2, y2, color, opacity);
    } else {
        // Solid
        SDL_SetRenderDrawColor(renderer,
                              (color >> 16) & 0xFF,
                              (color >> 8) & 0xFF,
                              color & 0xFF,
                              (uint8_t)(255 * opacity));
        
        if (thickness == 1) {
            SDL_RenderDrawLine(renderer, x1, y1, x2, y2);
        } else {
            // Draw multiple lines for thickness
            int dx = x2 - x1;
            int dy = y2 - y1;
            double len = sqrt(dx * dx + dy * dy);
            
            if (len > 0) {
                double offset = thickness / 2.0;
                double px = -dy / len * offset;
                double py = dx / len * offset;
                
                for (int i = -thickness/2; i <= thickness/2; i++) {
                    SDL_RenderDrawLine(renderer,
                                      (int)(x1 + px * i),
                                      (int)(y1 + py * i),
                                      (int)(x2 + px * i),
                                      (int)(y2 + py * i));
                }
            }
        }
    }
}

void ipe_draw_line(int x1, int y1, int x2, int y2, uint32_t color,
                   int thickness) {
  ipe_line_style_t style = {0};
  style.color = color;
  style.thickness = thickness;
  style.opacity = 1.0f;
  ipe_draw_line_styled(x1, y1, x2, y2, &style);
}

/* Bresenham circle algorithm for efficient circle drawing */
static void draw_circle_pixels(SDL_Renderer* renderer, int cx, int cy, 
                               int x, int y, uint32_t color, float opacity) {
    SDL_SetRenderDrawColor(renderer,
                          (color >> 16) & 0xFF,
                          (color >> 8) & 0xFF,
                          color & 0xFF,
                          (uint8_t)(255 * opacity));
    
    SDL_RenderDrawPoint(renderer, cx + x, cy + y);
    SDL_RenderDrawPoint(renderer, cx - x, cy + y);
    SDL_RenderDrawPoint(renderer, cx + x, cy - y);
    SDL_RenderDrawPoint(renderer, cx - x, cy - y);
    SDL_RenderDrawPoint(renderer, cx + y, cy + x);
    SDL_RenderDrawPoint(renderer, cx - y, cy + x);
    SDL_RenderDrawPoint(renderer, cx + y, cy - x);
    SDL_RenderDrawPoint(renderer, cx - y, cy - x);
}

static void draw_filled_circle(SDL_Renderer* renderer, int cx, int cy, 
                               int radius, uint32_t color, float opacity) {
    if (radius <= 0) return;
    
    int x = 0;
    int y = radius;
    int d = 3 - 2 * radius;
    
    SDL_SetRenderDrawColor(renderer,
                          (color >> 16) & 0xFF,
                          (color >> 8) & 0xFF,
                          color & 0xFF,
                          (uint8_t)(255 * opacity));

    while (x <= y) {
        // Draw horizontal lines to fill circle
        SDL_RenderDrawLine(renderer, cx - x, cy + y, cx + x, cy + y);
        SDL_RenderDrawLine(renderer, cx - x, cy - y, cx + x, cy - y);
        SDL_RenderDrawLine(renderer, cx - y, cy + x, cx + y, cy + x);
        SDL_RenderDrawLine(renderer, cx - y, cy - x, cx + y, cy - x);
        
        if (d < 0) {
            d = d + 4 * x + 6;
        } else {
            d = d + 4 * (x - y) + 10;
            y--;
        }
        x++;
    }
}

IPE_EXPORT void ipe_draw_circle_styled(int cx, int cy, int radius, ipe_circle_style_t* style) {
    if (!g_current_window || !g_current_window->renderer) {
        IPE_ERR("Window not initialized. Call ipe_init() first.");
        return;
    }
    
    SDL_Renderer* renderer = g_current_window->renderer;
    
    float fcx = (float)cx, fcy = (float)cy;
    ipe_transform_point(&fcx, &fcy);
    cx = (int)fcx; cy = (int)fcy;

    // Scale radius (simplified, using x scale)
    ipe_matrix_t* m = &g_transform_stack[g_transform_depth];
    float scale = sqrtf(m->m[0][0]*m->m[0][0] + m->m[0][1]*m->m[0][1]);
    radius = (int)(radius * scale);

    float opacity = style && style->opacity > 0 ? style->opacity : 1.0f;
    
    // Draw shadow first (if enabled)
    if (style && style->shadow_enabled) {
        uint32_t shadow_color = style->shadow_color;
        int shadow_cx = cx + style->shadow_offset_x;
        int shadow_cy = cy + style->shadow_offset_y;
        int shadow_radius = radius + style->shadow_radius;
        
        draw_filled_circle(renderer, shadow_cx, shadow_cy, shadow_radius, shadow_color, opacity * 0.5f);
    }
    
    // Draw filled circle
    // Radial gradient or solid
    if (style && style->color_center != style->color_edge) {
        for (int r = radius; r > 0; r--) {
            float t = (float)r / radius;
            uint32_t color = interpolate_color(style->color_center, style->color_edge, t);
            draw_filled_circle(renderer, cx, cy, r, color, opacity);
        }
    } else {
        uint32_t fill_color = style ? style->fill_color : 0xFFFFFF;
        draw_filled_circle(renderer, cx, cy, radius, fill_color, opacity);
    }
    
    // Draw border if width > 0
    if (style && style->border_width > 0) {
        uint32_t border_color = style->border_color;
        
        for (int i = 0; i < style->border_width; i++) {
            // Bresenham outline for each border pixel
            int r = radius - i;
            if (r < 0) break;
            int x = 0;
            int y = r;
            int d = 3 - 2 * r;
            while (x <= y) {
                draw_circle_pixels(renderer, cx, cy, x, y, border_color, opacity);
                if (d < 0) {
                    d = d + 4 * x + 6;
                } else {
                    d = d + 4 * (x - y) + 10;
                    y--;
                }
                x++;
            }
        }
    }
}

IPE_EXPORT void ipe_draw_circle(int cx, int cy, int radius, uint32_t color) {
  ipe_circle_style_t style = {0};
  style.fill_color = color;
  style.color_center = color;
  style.color_edge = color;
  style.opacity = 1.0f;
  ipe_draw_circle_styled(cx, cy, radius, &style);
}

void ipe_draw_triangle(int x1, int y1, int x2, int y2, int x3, int y3,
                       uint32_t color) {
  if (!g_current_window || !g_current_window->renderer)
    return;
  
  float fx1 = (float)x1, fy1 = (float)y1;
  float fx2 = (float)x2, fy2 = (float)y2;
  float fx3 = (float)x3, fy3 = (float)y3;
  ipe_transform_point(&fx1, &fy1);
  ipe_transform_point(&fx2, &fy2);
  ipe_transform_point(&fx3, &fy3);

  SDL_SetRenderDrawColor(g_current_window->renderer, (color >> 16) & 0xFF,
                         (color >> 8) & 0xFF, color & 0xFF, 255);
  SDL_RenderDrawLine(g_current_window->renderer, (int)fx1, (int)fy1, (int)fx2, (int)fy2);
  SDL_RenderDrawLine(g_current_window->renderer, (int)fx2, (int)fy2, (int)fx3, (int)fy3);
  SDL_RenderDrawLine(g_current_window->renderer, (int)fx3, (int)fy3, (int)fx1, (int)fy1);
}

void ipe_draw_rounded_rect(int x, int y, int w, int h, int radius,
                           uint32_t color) {
  if (!g_current_window || !g_current_window->renderer)
    return;
  // Draw main body with rounded corners
  ipe_draw_rect(x + radius, y, w - radius * 2, h, color);
  ipe_draw_rect(x, y + radius, w, h - radius * 2, color);
  
  // Draw corner circles (filled)
  ipe_circle_style_t corner_style = {0};
  corner_style.fill_color = color;
  corner_style.color_center = color;
  corner_style.color_edge = color;
  corner_style.opacity = 1.0f;
  ipe_draw_circle_styled(x + radius, y + radius, radius, &corner_style);
  ipe_draw_circle_styled(x + w - radius, y + radius, radius, &corner_style);
  ipe_draw_circle_styled(x + radius, y + h - radius, radius, &corner_style);
  ipe_draw_circle_styled(x + w - radius, y + h - radius, radius, &corner_style);
}

static void ipe_draw_arc(int x, int y, int radius, int start_angle, int end_angle, uint32_t color) {
    // Basic arc using points
    SDL_SetRenderDrawColor(g_current_window->renderer, (color >> 16) & 0xFF, (color >> 8) & 0xFF, color & 0xFF, 255);
    for (int a = start_angle; a <= end_angle; a++) {
        float rad = a * M_PI / 180.0;
        SDL_RenderDrawPoint(g_current_window->renderer, x + cos(rad) * radius, y + sin(rad) * radius);
    }
}



/* PERF: Polygon filling is O(n) per scanline. Could be optimized with scanline algorithm. */
static void draw_filled_polygon(SDL_Renderer* renderer, int* points, int point_count, uint32_t color, float opacity) {
    if (!renderer || point_count < 3 || !points) return;
    
    SDL_SetRenderDrawColor(renderer,
                          (color >> 16) & 0xFF,
                          (color >> 8) & 0xFF,
                          color & 0xFF,
                          (uint8_t)(255 * opacity));
    
    // Draw outline first (filled)
    for (int i = 0; i < point_count; i++) {
        int next = (i + 1) % point_count;
        SDL_RenderDrawLine(renderer,
                          points[i * 2],
                          points[i * 2 + 1],
                          points[next * 2],
                          points[next * 2 + 1]);
    }
    
    // Simple fill using point-by-point (not efficient but works)
    // Find bounding box
    int min_x = points[0], max_x = points[0];
    int min_y = points[0], max_y = points[0];
    for (int i = 1; i < point_count; i++) {
        if (points[i*2] < min_x) min_x = points[i*2];
        if (points[i*2] > max_x) max_x = points[i*2];
        if (points[i*2+1] < min_y) min_y = points[i*2+1];
        if (points[i*2+1] > max_y) max_y = points[i*2+1];
    }
    
    // Simple scanline fill
    for (int y = min_y; y <= max_y; y++) {
        int intersections[20];
        int inter_count = 0;
        
        for (int i = 0; i < point_count && inter_count < 20; i++) {
            int next = (i + 1) % point_count;
            int y1 = points[i*2 + 1];
            int y2 = points[next*2 + 1];
            
            if ((y1 <= y && y2 > y) || (y2 <= y && y1 > y)) {
                int x1 = points[i*2];
                int x2 = points[next*2];
                int x = x1 + (y - y1) * (x2 - x1) / (y2 - y1);
                intersections[inter_count++] = x;
            }
        }
        
        // Sort intersections
        for (int i = 0; i < inter_count - 1; i++) {
            for (int j = i + 1; j < inter_count; j++) {
                if (intersections[i] > intersections[j]) {
                    int temp = intersections[i];
                    intersections[i] = intersections[j];
                    intersections[j] = temp;
                }
            }
        }
        
        // Fill between pairs
        for (int i = 0; i < inter_count - 1; i += 2) {
            SDL_RenderDrawLine(renderer, intersections[i], y, intersections[i+1], y);
        }
    }
}

IPE_EXPORT void ipe_draw_polygon_styled(int* points, int point_count, ipe_polygon_style_t* style) {
    if (!g_current_window || !g_current_window->renderer) {
        IPE_ERR("Window not initialized. Call ipe_init() first.");
        return;
    }
    
    if (!points || point_count < 3) {
        IPE_ERR("Invalid polygon: need at least 3 points");
        return;
    }
    
    // Transform points to local buffer
    int* t_points = (int*)malloc(point_count * 2 * sizeof(int));
    for (int i = 0; i < point_count; i++) {
        float px = (float)points[i*2];
        float py = (float)points[i*2+1];
        ipe_transform_point(&px, &py);
        t_points[i*2] = (int)px;
        t_points[i*2+1] = (int)py;
    }

    SDL_Renderer* renderer = g_current_window->renderer;
    float opacity = style && style->opacity > 0 ? style->opacity : 1.0f;
    
    // Draw shadow first (if enabled)
    if (style && style->shadow_enabled) {
        SDL_SetRenderDrawColor(renderer,
                              (style->shadow_color >> 16) & 0xFF,
                              (style->shadow_color >> 8) & 0xFF,
                              style->shadow_color & 0xFF,
                              (uint8_t)(128 * opacity));
        
        // Simple shadow offset
        int shadow_offset = style->shadow_radius > 0 ? style->shadow_radius : 3;
        for (int i = 0; i < point_count; i++) {
            int next = (i + 1) % point_count;
            SDL_RenderDrawLine(renderer,
                              t_points[i*2] + shadow_offset,
                              t_points[i*2+1] + shadow_offset,
                              t_points[next*2] + shadow_offset,
                              t_points[next*2+1] + shadow_offset);
        }
    }
    
    // Draw filled polygon
    uint32_t fill_color = style ? style->fill_color : 0xFFFFFF;
    draw_filled_polygon(renderer, t_points, point_count, fill_color, opacity);
    
    // Draw border if width > 0
    if (style && style->border_width > 0) {
        uint32_t border_color = style->border_color;
        
        SDL_SetRenderDrawColor(renderer,
                              (border_color >> 16) & 0xFF,
                              (border_color >> 8) & 0xFF,
                              border_color & 0xFF,
                              (uint8_t)(255 * opacity));
        
        // Draw multiple borders for thickness
        for (int w = 0; w < style->border_width; w++) {
            for (int i = 0; i < point_count; i++) {
                int next = (i + 1) % point_count;
                SDL_RenderDrawLine(renderer,
                                  t_points[i*2] + w,
                                  t_points[i*2+1] + w,
                                  t_points[next*2] + w,
                                  t_points[next*2+1] + w);
            }
        }
    }
    free(t_points);
}

IPE_EXPORT void ipe_draw_polygon(int* points, int point_count, uint32_t fill_color, uint32_t border_color) {
    ipe_polygon_style_t style = {0};
    style.fill_color = fill_color;
    style.border_color = border_color;
    style.border_width = 1;
    style.opacity = 1.0f;
    ipe_draw_polygon_styled(points, point_count, &style);
}


void ipe_draw_polygon_ex(ipe_polygon_style_t* style) {
    // Legacy function - redirect to styled version if points available
    // This is a simplified version for backward compatibility
    if (!g_current_window || !g_current_window->renderer) return;
    // The old function doesn't have points, so we can't do much
    // This is kept for API compatibility only
}

// Colors
ipe_color_t ipe_color_rgb(uint8_t r, uint8_t g, uint8_t b) {
  return (ipe_color_t){r, g, b, 255};
}

ipe_color_t ipe_color_hex(uint32_t hex) {
  return (ipe_color_t){(hex >> 16) & 0xFF, (hex >> 8) & 0xFF, hex & 0xFF, 255};
}

uint32_t ipe_color_to_uint32(ipe_color_t color) {
  return (color.r << 16) | (color.g << 8) | color.b;
}

// Blending & Transparency
void ipe_set_blend_mode(ipe_blend_mode_t mode) {
    if (!g_current_window || !g_current_window->renderer) return;
    SDL_BlendMode sdl_mode = SDL_BLENDMODE_BLEND;
    switch (mode) {
        case IPE_BLEND_NORMAL: sdl_mode = SDL_BLENDMODE_BLEND; break;
        case IPE_BLEND_MULTIPLY: sdl_mode = SDL_BLENDMODE_MOD; break; // Approximated
        case IPE_BLEND_SCREEN: 
            // Use BLEND mode as fallback (perfect SCREEN mode requires custom composition)
            sdl_mode = SDL_BLENDMODE_BLEND;
            break;
        default: sdl_mode = SDL_BLENDMODE_BLEND; break;
    }
    SDL_SetRenderDrawBlendMode(g_current_window->renderer, sdl_mode);
}

void ipe_set_global_alpha(float alpha) {
    // Note: Applying this requires iterating textures or setting draw color A
}

// Masks
static SDL_Texture* g_mask_target = NULL;
void ipe_begin_mask(int x, int y, int w, int h) {
    if (!g_current_window) return;
    g_mask_target = SDL_CreateTexture(g_current_window->renderer, SDL_PIXELFORMAT_RGBA8888, SDL_TEXTUREACCESS_TARGET, w, h);
    SDL_SetRenderTarget(g_current_window->renderer, g_mask_target);
    SDL_SetRenderDrawColor(g_current_window->renderer, 0, 0, 0, 0);
    SDL_RenderClear(g_current_window->renderer);
}

void ipe_end_mask() {
    if (!g_current_window) return;
    SDL_SetRenderTarget(g_current_window->renderer, NULL);
    // Draw the mask results blended
    SDL_RenderCopy(g_current_window->renderer, g_mask_target, NULL, NULL);
    SDL_DestroyTexture(g_mask_target);
    g_mask_target = NULL;
}

void ipe_set_mask_from_texture(ipe_image_t* mask) {
    // Complex mask application logic here
}

// Utilities
void ipe_set_clipboard_text(const char *text) { SDL_SetClipboardText(text); }
char *ipe_get_clipboard_text() { return SDL_GetClipboardText(); }
void ipe_show_mouse(int show) { SDL_ShowCursor(show ? SDL_ENABLE : SDL_DISABLE); }

// Fonts
IPE_EXPORT ipe_font_t *ipe_font_load(const char *path, int size) {
  TTF_Font *ttf_font = TTF_OpenFont(path, size);
  if (!ttf_font)
    return NULL;
  ipe_font_t *font = (ipe_font_t *)malloc(sizeof(ipe_font_t));
  font->ttf_font = ttf_font;
  font->data = NULL; // We use TTF internally
  TTF_SizeText(ttf_font, "W", &font->char_width, &font->char_height);
  return font;
}

IPE_EXPORT void ipe_draw_text_ex(const char *text, int x, int y, ipe_font_t *font,
                      uint32_t color) {
  if (!g_current_window || !g_current_window->renderer || !font || !text)
    return;

  float fx = (float)x, fy = (float)y;
  ipe_transform_point(&fx, &fy);
  int tx = (int)fx, ty = (int)fy;

  SDL_Color sdl_color = {(color >> 16) & 0xFF, (color >> 8) & 0xFF, color & 0xFF,
                         255};
  SDL_Surface *surface = TTF_RenderUTF8_Blended(font->ttf_font, text, sdl_color);
  if (!surface)
    return;
  SDL_Texture *texture =
      SDL_CreateTextureFromSurface(g_current_window->renderer, surface);
  if (texture) {
    SDL_Rect dst = {tx, ty, surface->w, surface->h};
    SDL_RenderCopy(g_current_window->renderer, texture, NULL, &dst);
    SDL_DestroyTexture(texture);
  }
  SDL_FreeSurface(surface);
}

IPE_EXPORT int ipe_text_width(const char *text, ipe_font_t *font) {
  if (!font || !text)
    return 0;
  int w, h;
  TTF_SizeText(font->ttf_font, text, &w, &h);
  return w;
}

void ipe_draw_rich_text(ipe_rich_text_t* text, int x, int y, int max_width) {
    if (!g_current_window || !text || !text->text) return;
    // Simplified: draw segments with different styles
    int cursor_x = x;
    int last_idx = 0;
    for (int i=0; i < text->style_count; i++) {
        int end = text->style_ranges[i*2 + 1];
        if (end > strlen(text->text)) end = strlen(text->text);
        char segment[1024];
        int len = end - last_idx;
        strncpy(segment, text->text + last_idx, len);
        segment[len] = '\0';
        
        // Use default ipe_draw_text for now or load font from style
        ipe_draw_text(segment, cursor_x, y, text->styles[i]->color);
        cursor_x += len * 8; // Approximation
        last_idx = end;
    }
}

void ipe_draw_text_arc(const char* text, int cx, int cy, int radius, float start_angle, ipe_text_style_t* style) {
    if (!g_current_window || !text) return;
    float angle = start_angle;
    while (*text) {
        float rad = angle * M_PI / 180.0;
        int x = cx + cos(rad) * radius;
        int y = cy + sin(rad) * radius;
        char c[2] = {*text, '\0'};
        ipe_draw_text(c, x, y, style ? style->color : 0xFFFFFF);
        angle += 10.0; // Approximation of char spacing
        text++;
    }
}

// Interaction Feedback
void ipe_control_pulse(ipe_control_t* ctrl) {
    // Visual effect: temporary scale or brightness change
}

void ipe_control_shake(ipe_control_t* ctrl) {
    // Visual effect: temporary x/y oscillation
}

void ipe_control_highlight(ipe_control_t* ctrl) {
    // Visual effect: temporary border glow
}

void ipe_control_set_tooltip(ipe_control_t* ctrl, ipe_tooltip_t* tooltip) {
    // Store tooltip
}

// Components Implementation
void ipe_toolbar_add_button(ipe_toolbar_t* tb, ipe_image_t* icon, const char* tooltip, void (*cb)()) {
    // Add logic
}

typedef struct ipe_menu { int dummy; } ipe_menu_t; // Placeholder

void ipe_control_set_contextmenu(ipe_control_t* ctrl, ipe_menu_t* menu) {
    // Store menu
}

// Scroll & Zoom
void ipe_control_make_scrollable(ipe_control_t* ctrl, ipe_scrollable_t* scroll) {
    // Add scroll states to control
}

void ipe_scroll_to(ipe_control_t* ctrl, int x, int y, int animated) {
    // Trigger scroll animation
}

// Input Validation
void ipe_textfield_set_validator(ipe_control_t* field, ipe_validator_t validator, void* data) {
    // Set validator
}

// Layout Management
static void ipe_apply_flow_layout(ipe_control_t* parent, ipe_flow_layout_t* layout) {
    if (!parent || !layout) return;
    int cur_x = parent->x + layout->left_margin;
    int cur_y = parent->y + layout->top_margin;
    int row_h = 0;
    
    for (int i = 0; i < parent->children_count; i++) {
        ipe_control_t* child = parent->children[i];
        if (child && child->visible) {
            child->x = cur_x;
            child->y = cur_y;
            if (child->h > row_h) row_h = child->h;
            
            cur_x += child->w + layout->spacing_x;
            if (layout->wrap && cur_x > parent->x + parent->w - layout->right_margin) {
                cur_x = parent->x + layout->left_margin;
                cur_y += row_h + layout->spacing_y;
                row_h = 0;
            }
        }
    }
}

// Animations implementation
ipe_animation_t* ipe_animate_sequence(ipe_animation_seq_t* seq) {
    return NULL;
}

void ipe_transition_slide(ipe_control_t* from, ipe_control_t* to, int direction) {}
void ipe_transition_fade(ipe_control_t* from, ipe_control_t* to) {}
void ipe_transition_flip(ipe_control_t* from, ipe_control_t* to) {}

// Charts
void ipe_draw_barchart(int x, int y, int w, int h, ipe_chart_data_t* data) {}
void ipe_draw_piechart(int cx, int cy, int radius, ipe_chart_data_t* data) {}
void ipe_draw_linechart(int x, int y, int w, int h, ipe_chart_data_t* data) {}
void ipe_draw_sparkline(int* values, int count, int x, int y, int w, int h, uint32_t color) {}

// Clipboard and Drag-drop
void ipe_clipboard_set_image(ipe_image_t* img) {}
void ipe_clipboard_set_files(const char** paths, int count) {}
void ipe_control_set_drop_files(ipe_control_t* ctrl, void (*cb)(ipe_drop_event_t*)) {}

// Undo/Redo
void ipe_command_execute(ipe_command_t* cmd) {
    if (cmd && cmd->execute) cmd->execute(cmd->data);
}
void ipe_undo() {}
void ipe_redo() {}
int ipe_can_undo() { return 0; }
int ipe_can_redo() { return 0; }

// Shortcuts
void ipe_register_shortcut(ipe_shortcut_t* shortcut) {}
void ipe_unregister_shortcut(int keycode, int modifiers) {}

// Images
ipe_image_t *ipe_image_load(const char *path) {
  if (!g_current_window || !g_current_window->renderer)
    return NULL;
  SDL_Texture *texture = IMG_LoadTexture(g_current_window->renderer, path);
  if (!texture)
    return NULL;
  ipe_image_t *img = (ipe_image_t *)malloc(sizeof(ipe_image_t));
  img->texture = texture;
  SDL_QueryTexture(texture, NULL, NULL, &img->width, &img->height);
  return img;
}

void ipe_image_draw(ipe_image_t *img, int x, int y) {
  if (!g_current_window || !g_current_window->renderer || !img)
    return;
  SDL_Rect dst = {x, y, img->width, img->height};
  SDL_RenderCopy(g_current_window->renderer, img->texture, NULL, &dst);
}

void ipe_image_draw_scaled(ipe_image_t *img, int x, int y, int w, int h) {
  if (!g_current_window || !g_current_window->renderer || !img)
    return;
  SDL_Rect dst = {x, y, w, h};
  SDL_RenderCopy(g_current_window->renderer, img->texture, NULL, &dst);
}

void ipe_image_destroy(ipe_image_t *img) {
  if (!img)
    return;
  if (img->texture)
    SDL_DestroyTexture(img->texture);
  free(img);
}

// Timers
static Uint32 ipe_timer_callback(Uint32 interval, void *param) {
  ipe_timer_t *timer = (ipe_timer_t *)param;
  if (timer->on_tick)
    timer->on_tick(timer->user_data);
  return interval;
}

ipe_timer_t *ipe_timer_start(int interval_ms, void (*callback)(void *)) {
  ipe_timer_t *timer = (ipe_timer_t *)malloc(sizeof(ipe_timer_t));
  timer->interval = interval_ms;
  timer->on_tick = callback;
  timer->user_data = NULL;
  timer->running = 1;
  timer->timer_id = SDL_AddTimer(interval_ms, ipe_timer_callback, timer);
  return timer;
}

void ipe_timer_stop(ipe_timer_t *timer) {
  if (!timer)
    return;
  SDL_RemoveTimer(timer->timer_id);
  free(timer);
}

Uint32 ipe_get_ticks() { return SDL_GetTicks(); }

// Screenshot
void ipe_screenshot(const char *filename) {
  if (!g_current_window || !g_current_window->renderer)
    return;
  int w, h;
  SDL_GetRendererOutputSize(g_current_window->renderer, &w, &h);
  SDL_Surface *sshot = SDL_CreateRGBSurface(0, w, h, 32, 0x00ff0000, 0x0000ff00,
                                            0x000000ff, 0xff000000);
  SDL_RenderReadPixels(g_current_window->renderer, NULL, SDL_PIXELFORMAT_ARGB8888,
                       sshot->pixels, sshot->pitch);
  IMG_SavePNG(sshot, filename);
  SDL_FreeSurface(sshot);
}

// UI Controls & Layout
void ipe_button_draw(ipe_button_t *btn) {
  uint32_t color = btn->bg_color;
  if (btn->is_pressed)
    color = 0xAAAAAA;
  else if (btn->is_hovered)
    color = 0xCCCCCC;

  ipe_draw_rect(btn->x, btn->y, btn->w, btn->h, color);
  ipe_draw_text(btn->text, btn->x + 5, btn->y + (btn->h / 2) - 4,
                btn->text_color);
}

int ipe_button_handle_event(ipe_button_t *btn, int event, int mx, int my) {
  int in_rect = (mx >= btn->x && mx <= btn->x + btn->w && my >= btn->y &&
                 my <= btn->y + btn->h);

  btn->is_hovered = in_rect;

  if (event == 2) { // MouseDown
    if (in_rect) {
      btn->is_pressed = 1;
      return 1;
    }
  } else if (event == 3) { // MouseUp
    if (btn->is_pressed && in_rect) {
      if (btn->on_click)
        btn->on_click(NULL); // Fixed param as void*
    }
    btn->is_pressed = 0;
  }
  return 0;
}

ipe_button_t *ipe_button_create(int x, int y, int w, int h, const char *text) {
  ipe_button_t *btn = (ipe_button_t *)malloc(sizeof(ipe_button_t));
  btn->x = x;
  btn->y = y;
  btn->w = w;
  btn->h = h;
  btn->text = strdup(text);
  btn->bg_color = 0xFFFFFF;
  btn->text_color = 0x000000;
  btn->is_hovered = 0;
  btn->is_pressed = 0;
  btn->on_click = NULL;
  return btn;
}

// Internal wrapper for button to work as ipe_control_t
static void ipe_button_wrapper_draw(ipe_control_t *ctrl) {
  ipe_button_draw((ipe_button_t *)ctrl->user_data);
}

static int ipe_button_wrapper_event(ipe_control_t *ctrl, int event, int x, int y) {
  return ipe_button_handle_event((ipe_button_t *)ctrl, event, x, y);
}

void ipe_control_add(ipe_control_t *parent, ipe_control_t *child) {
  if (!parent || !child) return;
  
  // If parent is actually a window, we need a way to detect it.
  // For now, let's allow adding to the global root if parent is NULL
  // or use a clever cast if we assume the first fields match.
  // But wait, ipe_window_t doesn't start with coordinates.
  
  // Let's assume the user might pass a window as parent in their logic
  // we'll check if it's the current window.
  if ((void*)parent == (void*)g_current_window) {
    if (!g_current_window->root_control) {
      g_current_window->root_control = child;
    } else {
      ipe_control_t *curr = g_current_window->root_control;
      while (curr->next) curr = curr->next;
      curr->next = child;
    }
    return;
  }

  if (parent->children_count == 0) {
    parent->children = (ipe_control_t**)malloc(sizeof(ipe_control_t*));
    parent->children[0] = child;
    parent->children_count = 1;
    parent->children_capacity = 1;
  } else {
    ipe_control_add_child(parent, child);
  }
}

void ipe_control_remove(ipe_control_t *ctrl) {
  if (!g_control_root || !ctrl) return;
  if (g_control_root == ctrl) {
    g_control_root = ctrl->next;
    return;
  }
  ipe_control_t *curr = g_control_root;
  while (curr->next && curr->next != ctrl) curr = curr->next;
  if (curr->next) curr->next = ctrl->next;
}

ipe_control_t *ipe_control_find(const char *name) {
  ipe_control_t *curr = g_control_root;
  while (curr) {
    if (curr->name && strcmp(curr->name, name) == 0) return curr;
    curr = curr->next;
  }
  return NULL;
}

// Themes
static uint32_t g_theme_bg = 0xFFFFFF;
static uint32_t g_theme_text = 0x000000;

void ipe_set_theme(const char *theme_name) {
  if (strcmp(theme_name, "dark") == 0) {
    g_theme_bg = 0x333333;
    g_theme_text = 0xFFFFFF;
  } else if (strcmp(theme_name, "light") == 0) {
    g_theme_bg = 0xFFFFFF;
    g_theme_text = 0x000000;
  } else if (strcmp(theme_name, "blue") == 0) {
    g_theme_bg = 0x0000FF;
    g_theme_text = 0xFFFFFF;
  }
}

// Dialogs
int ipe_message_box(const char *title, const char *message, int type) {
  SDL_MessageBoxData messageboxdata = {
      SDL_MESSAGEBOX_INFORMATION,
      NULL,
      title,
      message,
      0,
      NULL,
      NULL};
  
  if (type == 1) { // OKCancel
      SDL_MessageBoxButtonData buttons[] = {
          { SDL_MESSAGEBOX_BUTTON_RETURNKEY_DEFAULT, 0, "OK" },
          { SDL_MESSAGEBOX_BUTTON_ESCAPEKEY_DEFAULT, 1, "Cancel" },
      };
      messageboxdata.numbuttons = 2;
      messageboxdata.buttons = buttons;
  } else {
      SDL_MessageBoxButtonData buttons[] = {
          { SDL_MESSAGEBOX_BUTTON_RETURNKEY_DEFAULT, 0, "OK" },
      };
      messageboxdata.numbuttons = 1;
      messageboxdata.buttons = buttons;
  }

  int buttonid;
  SDL_ShowMessageBox(&messageboxdata, &buttonid);
  return buttonid;
}

char *ipe_open_file_dialog(const char *filter) {
  char command[1024];
  // Using zenity for Linux file dialog
  snprintf(command, sizeof(command), "zenity --file-selection --title=\"Open File\" --file-filter=\"%s\"", filter ? filter : "*");
  FILE *f = popen(command, "r");
  if (!f) return NULL;
  char path[1024];
  if (fgets(path, sizeof(path), f)) {
    size_t len = strlen(path);
    if (len > 0 && path[len-1] == '\n') path[len-1] = '\0';
    pclose(f);
    return strdup(path);
  }
  pclose(f);
  return NULL;
}

char *ipe_save_file_dialog(const char *filter) {
  char command[1024];
  snprintf(command, sizeof(command), "zenity --file-selection --save --confirm-overwrite --title=\"Save File\"");
  FILE *f = popen(command, "r");
  if (!f) return NULL;
  char path[1024];
  if (fgets(path, sizeof(path), f)) {
    size_t len = strlen(path);
    if (len > 0 && path[len-1] == '\n') path[len-1] = '\0';
    pclose(f);
    return strdup(path);
  }
  pclose(f);
  return NULL;
}

IPE_EXPORT int ipe_process_events() {
  SDL_Event e;
  g_last_event_type = 0;
  g_mouse_scroll_delta = 0;
  
  // Track previous mouse buttons
  g_prev_mouse_buttons = g_mouse_buttons;
  g_mouse_buttons = 0;

  while (SDL_PollEvent(&e)) {
    switch (e.type) {
    case SDL_QUIT:
      g_running = 0;
      g_last_event_type = 1;
      // Queue window close event
      {
        ipe_input_event_t evt = { .type = IPE_EVENT_WINDOW_CLOSE, .timestamp_ms = e.quit.timestamp };
        ipe_queue_event(evt);
      }
      break;
    case SDL_MOUSEBUTTONDOWN:
      g_last_event_type = 2;
      g_mouse_x = e.button.x;
      g_mouse_y = e.button.y;
      g_mouse_buttons |= (1 << (e.button.button - 1)); // 0=left, 1=middle, 2=right
      
      // Handle control events
      if (g_control_root != NULL) {
        ipe_control_handle_mouse_click(e.button.x, e.button.y);
      }
      
      // Queue mouse down event
      {
        ipe_input_event_t evt = {
            .type = IPE_EVENT_MOUSE_DOWN,
            .x = e.button.x,
            .y = e.button.y,
            .button = e.button.button - 1,
            .timestamp_ms = e.button.timestamp
        };
        ipe_queue_event(evt);
      }
      
      if (g_mousedown_cb)
        g_mousedown_cb(g_mouse_x, g_mouse_y, e.button.button);
      break;
    case SDL_MOUSEBUTTONUP:
      g_last_event_type = 3;
      g_mouse_x = e.button.x;
      g_mouse_y = e.button.y;
      g_mouse_buttons &= ~(1 << (e.button.button - 1));
      
      // Queue mouse up event
      {
        ipe_input_event_t evt = {
            .type = IPE_EVENT_MOUSE_UP,
            .x = e.button.x,
            .y = e.button.y,
            .button = e.button.button - 1,
            .timestamp_ms = e.button.timestamp
        };
        ipe_queue_event(evt);
      }
      break;
    case SDL_MOUSEMOTION:
      // Handle control events BEFORE updating globals
      if (g_control_root != NULL) {
        ipe_control_handle_mouse_move(e.motion.x, e.motion.y);
      }
      
      g_mouse_x = e.motion.x;
      g_mouse_y = e.motion.y;
      
      // Queue mouse move event
      {
        ipe_input_event_t evt = {
            .type = IPE_EVENT_MOUSE_MOVE,
            .x = e.motion.x,
            .y = e.motion.y,
            .timestamp_ms = e.motion.timestamp
        };
        ipe_queue_event(evt);
      }
      
      if (g_mousemove_cb)
        g_mousemove_cb(g_mouse_x, g_mouse_y, 0);
      break;
    case SDL_MOUSEWHEEL:
      g_mouse_scroll_delta = e.wheel.y;
      
      // Queue scroll event
      {
        ipe_input_event_t evt = {
            .type = IPE_EVENT_SCROLL,
            .scroll_delta = e.wheel.y,
            .timestamp_ms = e.wheel.timestamp
        };
        ipe_queue_event(evt);
      }
      break;
    case SDL_KEYDOWN:
      g_last_event_type = 4;
      g_key_code = e.key.keysym.sym;
      
      // Handle control events
      if (g_control_root != NULL) {
        ipe_control_handle_key_press(e.key.keysym.sym);
      }
      
      // Track key pressed
      if (e.key.keysym.scancode < 512) {
        g_input_state.key_pressed[e.key.keysym.scancode] = 1;
      }
      
      // Queue key down event
      {
        ipe_input_event_t evt = {
            .type = IPE_EVENT_KEY_DOWN,
            .key_code = e.key.keysym.scancode,
            .timestamp_ms = e.key.timestamp
        };
        ipe_queue_event(evt);
      }
      
      if (g_keydown_cb)
        g_keydown_cb(g_key_code);
      break;
    case SDL_KEYUP:
      // Track key released
      if (e.key.keysym.scancode < 512) {
        g_input_state.key_released[e.key.keysym.scancode] = 1;
      }
      
      // Queue key up event
      {
        ipe_input_event_t evt = {
            .type = IPE_EVENT_KEY_UP,
            .key_code = e.key.keysym.scancode,
            .timestamp_ms = e.key.timestamp
        };
        ipe_queue_event(evt);
      }
      break;
    case SDL_WINDOWEVENT:
      if (e.window.event == SDL_WINDOWEVENT_RESIZED) {
        if (g_resize_cb)
          g_resize_cb(e.window.data1, e.window.data2);
        
        // Queue resize event
        {
          ipe_input_event_t evt = {
              .type = IPE_EVENT_WINDOW_RESIZE,
              .x = e.window.data1,
              .y = e.window.data2,
              .timestamp_ms = e.window.timestamp
          };
          ipe_queue_event(evt);
        }
      }
      break;
    }
  }
  
  // Clear pressed/released keys that were processed
  memset(g_input_state.key_pressed, 0, sizeof(g_input_state.key_pressed));
  memset(g_input_state.key_released, 0, sizeof(g_input_state.key_released));

  // Handle control events for the current window
  if (g_current_window && g_current_window->root_control) {
    ipe_control_t *curr = g_current_window->root_control;
    while (curr) {
      if (curr->visible && curr->handle_event) {
        // Simple event conversion for now
        int type = 0;
        if (e.type == SDL_MOUSEBUTTONDOWN) type = 2;
        else if (e.type == SDL_MOUSEBUTTONUP) type = 3;
        
        if (type != 0) {
            curr->handle_event(curr, type, g_mouse_x, g_mouse_y);
        }
      }
      curr = curr->next;
    }
  }

  return g_running;
}

int ipe_get_last_event() { return g_last_event_type; }
int ipe_get_mouse_x() { return g_mouse_x; }
int ipe_get_mouse_y() { return g_mouse_y; }
int ipe_get_key_code() { return g_key_code; }

IPE_EXPORT int ipe_is_window_open() { return g_running; }

IPE_EXPORT int ipe_window_get_width() { 
    if (g_current_window) return g_current_window->width;
    return 0; 
}

IPE_EXPORT int ipe_window_get_height() { 
    if (g_current_window) return g_current_window->height;
    return 0; 
}

IPE_EXPORT void ipe_window_close() {
  if (g_current_window) {
    ipe_window_destroy(g_current_window);
  }
  SDL_Quit();
}
