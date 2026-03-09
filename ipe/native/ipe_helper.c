#include <windows.h>
#include <stdint.h>

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

// Global state
static ipe_window_t *g_current_window = NULL;
static int g_running = 1;
static HWND g_hwnd = NULL;
static HDC g_hdc = NULL;

LRESULT CALLBACK WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam) {
    switch (uMsg) {
        case WM_DESTROY:
            PostQuitMessage(0);
            g_running = 0;
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

__declspec(dllexport) int ipe_init() {
    WNDCLASS wc = {0};
    wc.lpfnWndProc = WindowProc;
    wc.hInstance = GetModuleHandle(NULL);
    wc.lpszClassName = "IpeWindowClass";
    wc.hbrBackground = (HBRUSH)(COLOR_WINDOW + 1);
    wc.hCursor = LoadCursor(NULL, IDC_ARROW);

    if (!RegisterClass(&wc)) return 0;
    return 1;
}

__declspec(dllexport) void* ipe_window_create(int width, int height, const char* title) {
    g_hwnd = CreateWindowEx(
        0,
        "IpeWindowClass",
        title,
        WS_OVERLAPPEDWINDOW | WS_VISIBLE,
        CW_USEDEFAULT, CW_USEDEFAULT, width, height,
        NULL, NULL, GetModuleHandle(NULL), NULL
    );

    if (g_hwnd) {
        g_hdc = GetDC(g_hwnd);
    }
    return (void*)g_hwnd;
}

__declspec(dllexport) void ipe_clear_background(uint32_t color) {
    if (!g_hwnd) return;
    RECT rect;
    GetClientRect(g_hwnd, &rect);
    HBRUSH brush = CreateSolidBrush(RGB((color >> 16) & 0xFF, (color >> 8) & 0xFF, color & 0xFF));
    FillRect(g_hdc, &rect, brush);
    DeleteObject(brush);
}

__declspec(dllexport) void ipe_draw_rect(int x, int y, int w, int h, uint32_t color) {
    if (!g_hdc) return;
    HBRUSH brush = CreateSolidBrush(RGB((color >> 16) & 0xFF, (color >> 8) & 0xFF, color & 0xFF));
    RECT rect = {x, y, x + w, y + h};
    FillRect(g_hdc, &rect, brush);
    DeleteObject(brush);
}

__declspec(dllexport) int ipe_process_events() {
    MSG msg;
    while (PeekMessage(&msg, NULL, 0, 0, PM_REMOVE)) {
        if (msg.message == WM_QUIT) {
            g_running = 0;
        }
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }
    return g_running;
}

__declspec(dllexport) int ipe_is_window_open() {
    return g_running;
}

__declspec(dllexport) void ipe_window_close() {
    if (g_hwnd) {
        ReleaseDC(g_hwnd, g_hdc);
        DestroyWindow(g_hwnd);
        g_hwnd = NULL;
    }
}
