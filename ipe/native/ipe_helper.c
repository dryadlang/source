#include <stdint.h>
#include <windows.h>
#include <stdio.h>
#include <math.h>

// Window structure
typedef struct {
  HWND hwnd;
  HDC hdc;
  int id;
  char *title;
  int width, height;
  int is_open;
  struct ipe_control *root_control;
} ipe_window_t;

// Color structure
typedef struct {
  uint8_t r, g, b, a;
} ipe_color_t;

// Font structure
typedef struct {
  void* data; // Internal GDI font
  int char_width;
  int char_height;
} ipe_font_t;

// Image structure
typedef struct {
  HBITMAP hBitmap;
  int width;
  int height;
} ipe_image_t;

// Phase 1: Advanced Graphics Styles
typedef struct {
    int x, y, w, h;
    uint32_t fill_color;
    uint32_t border_color;
    int border_width;
    int border_style; // 0=solid, 1=dashed, 2=dotted
    int corner_radius_tl;
    int corner_radius_tr;
    int corner_radius_bl;
    int corner_radius_br;
    int shadow_radius;
    int shadow_offset_x;
    int shadow_offset_y;
    uint32_t shadow_color;
    float opacity; // 0.0 a 1.0
    int gradient; // 0=none, 1=horizontal, 2=vertical, 3=radial
    uint32_t gradient_color_start;
    uint32_t gradient_color_end;
} ipe_rect_style_t;

typedef struct {
    int x, y, radius;
    uint32_t color_center;
    uint32_t color_edge;
    int border_width;
    uint32_t border_color;
    int start_angle;
    int end_angle;
    int shadow;
} ipe_circle_style_t;

typedef struct {
    int* points; // intercalado x,y
    int point_count;
    uint32_t fill_color;
    uint32_t border_color;
    int border_width;
    int* corner_radius; // por vértice
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

typedef struct ipe_animation_seq {
    struct ipe_control* target;
    char* property;
    float target_value;
    float duration;
    ipe_easing_t easing;
    struct ipe_animation_seq* next;
} ipe_animation_seq_t;

typedef struct ipe_animation { int dummy; } ipe_animation_t;

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
  UINT_PTR timer_id;
  int interval;
  void (*on_tick)(void *);
  void *user_data;
  int running;
} ipe_timer_t;

// Control structure
typedef struct ipe_control {
  int x, y, w, h;
  int visible;
  char *name;
  void (*draw)(struct ipe_control *);
  int (*handle_event)(struct ipe_control *, int event, int x, int y);
  void *user_data;
  struct ipe_control *next;
  struct ipe_control *children;
  ipe_anchors_t anchors;
  ipe_size_constraints_t size_constraints;
  void* layout;
  int layout_type;
} ipe_control_t;

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

static ipe_window_t *g_current_window = NULL;
static int g_running = 1;

// Callbacks
typedef void (*ipe_mouse_callback)(int x, int y, int button);
typedef void (*ipe_key_callback)(int keycode);
typedef void (*ipe_resize_callback)(int w, int h);

static ipe_mouse_callback g_mousedown_cb = NULL;
static ipe_mouse_callback g_mousemove_cb = NULL;
static ipe_key_callback g_keydown_cb = NULL;
static ipe_resize_callback g_resize_cb = NULL;

static int g_last_event_type = 0;
static int g_mouse_x = 0;
static int g_mouse_y = 0;
static int g_key_code = 0;

LRESULT CALLBACK WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam) {
  ipe_window_t *win = (ipe_window_t *)GetWindowLongPtr(hwnd, GWLP_USERDATA);
  
  switch (uMsg) {
  case WM_DESTROY:
    if (win) win->is_open = 0;
    g_running = 0;
    g_last_event_type = 1;
    return 0;
  case WM_LBUTTONDOWN:
    g_last_event_type = 2;
    g_mouse_x = LOWORD(lParam);
    g_mouse_y = HIWORD(lParam);
    if (g_mousedown_cb) g_mousedown_cb(g_mouse_x, g_mouse_y, 1);
    return 0;
  case WM_LBUTTONUP:
    g_last_event_type = 3;
    g_mouse_x = LOWORD(lParam);
    g_mouse_y = HIWORD(lParam);
    return 0;
  case WM_MOUSEMOVE:
    g_mouse_x = LOWORD(lParam);
    g_mouse_y = HIWORD(lParam);
    if (g_mousemove_cb) g_mousemove_cb(g_mouse_x, g_mouse_y, 0);
    return 0;
  case WM_KEYDOWN:
    g_last_event_type = 4;
    g_key_code = (int)wParam;
    if (g_keydown_cb) g_keydown_cb(g_key_code);
    return 0;
  case WM_SIZE:
    if (g_resize_cb) g_resize_cb(LOWORD(lParam), HIWORD(lParam));
    return 0;
  case WM_PAINT: {
    PAINTSTRUCT ps;
    HDC hdc = BeginPaint(hwnd, &ps);
    EndPaint(hwnd, &ps);
    return 0;
  }
  }
  return DefWindowProc(hwnd, uMsg, wParam, lParam);
}

int ipe_init() {
  WNDCLASS wc = {0};
  wc.lpfnWndProc = WindowProc;
  wc.hInstance = GetModuleHandle(NULL);
  wc.lpszClassName = "IpeWindowClass";
  wc.hbrBackground = (HBRUSH)(COLOR_WINDOW + 1);
  wc.hCursor = LoadCursor(NULL, IDC_ARROW);
  if (!RegisterClass(&wc)) return 0;
  return 1;
}

ipe_window_t *ipe_window_create(int width, int height, const char *title) {
  ipe_window_t *win = (ipe_window_t *)malloc(sizeof(ipe_window_t));
  if (!win) return NULL;

  win->hwnd = CreateWindowEx(0, "IpeWindowClass", title,
                           WS_OVERLAPPEDWINDOW | WS_VISIBLE, CW_USEDEFAULT,
                           CW_USEDEFAULT, width, height, NULL, NULL,
                           GetModuleHandle(NULL), NULL);

  if (win->hwnd) {
    win->hdc = GetDC(win->hwnd);
    win->title = _strdup(title);
    win->width = width;
    win->height = height;
    win->is_open = 1;
    win->root_control = NULL;
    SetWindowLongPtr(win->hwnd, GWLP_USERDATA, (LONG_PTR)win);
    if (!g_current_window) g_current_window = win;
  } else {
    free(win);
    return NULL;
  }
  return win;
}

void ipe_window_set_current(ipe_window_t *win) {
  if (win) g_current_window = win;
}

void ipe_window_destroy(ipe_window_t *win) {
  if (!win) return;
  if (g_current_window == win) g_current_window = NULL;
  if (win->hdc) ReleaseDC(win->hwnd, win->hdc);
  if (win->hwnd) DestroyWindow(win->hwnd);
  if (win->title) free(win->title);
  free(win);
}

void ipe_clear_background(uint32_t color) {
  if (!g_current_window) return;
  RECT rect;
  GetClientRect(g_current_window->hwnd, &rect);
  HBRUSH brush = CreateSolidBrush(RGB((color >> 16) & 0xFF, (color >> 8) & 0xFF, color & 0xFF));
  FillRect(g_current_window->hdc, &rect, brush);
  DeleteObject(brush);
}

void ipe_present() { /* Direct GDI */ }

void ipe_draw_rect(int x, int y, int w, int h, uint32_t color) {
  if (!g_current_window) return;
  HBRUSH brush = CreateSolidBrush(RGB((color >> 16) & 0xFF, (color >> 8) & 0xFF, color & 0xFF));
  RECT rect = {x, y, x + w, y + h};
  FillRect(g_current_window->hdc, &rect, brush);
  DeleteObject(brush);
}

void ipe_draw_line(int x1, int y1, int x2, int y2, uint32_t color, int thickness) {
  if (!g_current_window) return;
  HPEN pen = CreatePen(PS_SOLID, thickness, RGB((color >> 16) & 0xFF, (color >> 8) & 0xFF, color & 0xFF));
  HPEN oldPen = SelectObject(g_current_window->hdc, pen);
  MoveToEx(g_current_window->hdc, x1, y1, NULL);
  LineTo(g_current_window->hdc, x2, y2);
  SelectObject(g_current_window->hdc, oldPen);
  DeleteObject(pen);
}

void ipe_draw_circle(int x, int y, int radius, uint32_t color, int filled) {
    if (!g_current_window) return;
    COLORREF cr = RGB((color >> 16) & 0xFF, (color >> 8) & 0xFF, color & 0xFF);
    if (filled) {
        HBRUSH brush = CreateSolidBrush(cr);
        HBRUSH oldBrush = SelectObject(g_current_window->hdc, brush);
        HPEN pen = CreatePen(PS_NULL, 0, 0);
        HPEN oldPen = SelectObject(g_current_window->hdc, pen);
        Ellipse(g_current_window->hdc, x - radius, y - radius, x + radius, y + radius);
        SelectObject(g_current_window->hdc, oldBrush);
        SelectObject(g_current_window->hdc, oldPen);
        DeleteObject(brush);
        DeleteObject(pen);
    } else {
        HPEN pen = CreatePen(PS_SOLID, 1, cr);
        HPEN oldPen = SelectObject(g_current_window->hdc, pen);
        Arc(g_current_window->hdc, x-radius, y-radius, x+radius, y+radius, 0,0,0,0);
        SelectObject(g_current_window->hdc, oldPen);
        DeleteObject(pen);
    }
}

// Porting all advanced logic (simplified GDI versions)
void ipe_draw_rect_ex(ipe_rect_style_t* style) { ipe_draw_rect(style->x, style->y, style->w, style->h, style->fill_color); }
void ipe_draw_circle_ex(ipe_circle_style_t* style) { ipe_draw_circle(style->x, style->y, style->radius, style->color_center, 1); }
void ipe_draw_polygon_ex(ipe_polygon_style_t* style) {}
void ipe_set_blend_mode(ipe_blend_mode_t mode) {}
void ipe_set_global_alpha(float alpha) {}
void ipe_draw_text(const char* text, int x, int y, uint32_t color) { TextOutA(g_current_window->hdc, x, y, text, (int)strlen(text)); }
void ipe_draw_text_ex(const char *text, int x, int y, ipe_font_t *font, uint32_t color) { TextOutA(g_current_window->hdc, x, y, text, (int)strlen(text)); }

// Callbacks setters
void ipe_set_mousedown_callback(ipe_mouse_callback cb) { g_mousedown_cb = cb; }
void ipe_set_mousemove_callback(ipe_mouse_callback cb) { g_mousemove_cb = cb; }
void ipe_set_keydown_callback(ipe_key_callback cb) { g_keydown_cb = cb; }
void ipe_set_resize_callback(ipe_resize_callback cb) { g_resize_cb = cb; }

// Component logic
void ipe_control_add(ipe_control_t *parent, ipe_control_t *child) {
  if (!parent || !child) return;
  if ((void*)parent == (void*)g_current_window) {
    if (!g_current_window->root_control) g_current_window->root_control = child;
    else { ipe_control_t *c = g_current_window->root_control; while (c->next) c = c->next; c->next = child; }
    return;
  }
  if (!parent->children) parent->children = child;
  else { ipe_control_t *c = parent->children; while (c->next) c = c->next; c->next = child; }
}

int ipe_process_events() {
  MSG msg;
  g_last_event_type = 0;
  while (PeekMessage(&msg, NULL, 0, 0, PM_REMOVE)) {
    if (msg.message == WM_QUIT) g_running = 0;
    TranslateMessage(&msg);
    DispatchMessage(&msg);
  }
  // Process UI hierarchy
  if (g_current_window && g_current_window->root_control) {
      // Loop controls (simplified)
  }
  return g_running;
}

// Rest of missing signatures (placeholders for alignment)
void ipe_set_theme(const char* tn) {}
int ipe_message_box(const char* t, const char* m, int ty) { return MessageBoxA(NULL, m, t, MB_OK); }
char* ipe_open_file_dialog(const char* f) { return NULL; }
char* ipe_save_file_dialog(const char* f) { return NULL; }
void ipe_screenshot(const char* f) {}
ipe_image_t* ipe_image_load(const char* p) { return NULL; }
void ipe_image_draw(ipe_image_t* i, int x, int y) {}
void ipe_image_destroy(ipe_image_t* i) {}
ipe_timer_t* ipe_timer_start(int i, void (*cb)(void*)) { return NULL; }
void ipe_timer_stop(ipe_timer_t* t) {}
uint32_t ipe_get_ticks() { return GetTickCount(); }

void ipe_window_close() {
  if (g_current_window) ipe_window_destroy(g_current_window);
}

int ipe_get_last_event() { return g_last_event_type; }
int ipe_get_mouse_x() { return g_mouse_x; }
int ipe_get_mouse_y() { return g_mouse_y; }
int ipe_get_key_code() { return g_key_code; }
int ipe_is_window_open() { return g_running; }

// Final alignment signatures
void ipe_draw_triangle(int x1, int y1, int x2, int y2, int x3, int y3, uint32_t color) {}
void ipe_draw_rounded_rect(int x, int y, int w, int h, int radius, uint32_t color) { ipe_draw_rect(x, y, w, h, color); }
void ipe_begin_mask(int x, int y, int w, int h) {}
void ipe_end_mask() {}
void ipe_set_mask_from_texture(ipe_image_t* mask) {}
void ipe_set_clipboard_text(const char *text) { OpenClipboard(NULL); EmptyClipboard(); HGLOBAL hg = GlobalAlloc(GMEM_MOVEABLE, (SIZE_T)strlen(text) + 1); memcpy(GlobalLock(hg), text, strlen(text) + 1); GlobalUnlock(hg); SetClipboardData(CF_TEXT, hg); CloseClipboard(); }
char *ipe_get_clipboard_text() { return NULL; }
void ipe_show_mouse(int show) { ShowCursor(show); }
ipe_font_t *ipe_font_load(const char *path, int size) { return NULL; }
int ipe_text_width(const char *text, ipe_font_t *font) { return (int)strlen(text) * 8; }
void ipe_draw_rich_text(ipe_rich_text_t* text, int x, int y, int max_width) {}
void ipe_draw_text_arc(const char* text, int cx, int cy, int radius, float start_angle, ipe_text_style_t* style) {}
void ipe_control_pulse(ipe_control_t* ctrl) {}
void ipe_control_shake(ipe_control_t* ctrl) {}
void ipe_control_highlight(ipe_control_t* ctrl) {}
void ipe_control_set_tooltip(ipe_control_t* ctrl, ipe_tooltip_t* tooltip) {}
void ipe_toolbar_add_button(ipe_toolbar_t* tb, ipe_image_t* icon, const char* tooltip, void (*cb)()) {}
void ipe_control_set_contextmenu(ipe_control_t* ctrl, void* menu) {}
void ipe_control_make_scrollable(ipe_control_t* ctrl, ipe_scrollable_t* scroll) {}
void ipe_scroll_to(ipe_control_t* ctrl, int x, int y, int animated) {}
void ipe_control_set_anchors(ipe_control_t* ctrl, ipe_anchors_t anchors) {}
ipe_animation_t* ipe_animate_sequence(ipe_animation_seq_t* seq) { return NULL; }
void ipe_transition_slide(ipe_control_t* from, ipe_control_t* to, int direction) {}
void ipe_transition_fade(ipe_control_t* from, ipe_control_t* to) {}
void ipe_transition_flip(ipe_control_t* from, ipe_control_t* to) {}
void ipe_draw_barchart(int x, int y, int w, int h, ipe_chart_data_t* data) {}
void ipe_draw_piechart(int cx, int cy, int radius, ipe_chart_data_t* data) {}
void ipe_draw_linechart(int x, int y, int w, int h, ipe_chart_data_t* data) {}
void ipe_draw_sparkline(int* values, int count, int x, int y, int w, int h, uint32_t color) {}
void ipe_clipboard_set_image(ipe_image_t* img) {}
void ipe_clipboard_set_files(const char** paths, int count) {}
void ipe_control_set_drop_files(ipe_control_t* ctrl, void (*cb)(ipe_drop_event_t*)) {}
void ipe_command_execute(ipe_command_t* cmd) { if (cmd && cmd->execute) cmd->execute(cmd->data); }
void ipe_undo() {}
void ipe_redo() {}
int ipe_can_undo() { return 0; }
int ipe_can_redo() { return 0; }
void ipe_register_shortcut(ipe_shortcut_t* shortcut) {}
void ipe_unregister_shortcut(int keycode, int modifiers) {}
void ipe_image_draw_scaled(ipe_image_t *img, int x, int y, int w, int h) {}
void ipe_button_draw(ipe_button_t *btn) { ipe_draw_rect(btn->x, btn->y, btn->w, btn->h, btn->bg_color); }
int ipe_button_handle_event(ipe_button_t *btn, int event, int mx, int my) { return 0; }
ipe_button_t *ipe_button_create(int x, int y, int w, int h, const char *text) { return NULL; }
ipe_control_t *ipe_control_find(const char *name) { return NULL; }
void ipe_control_remove(ipe_control_t *ctrl) {}
