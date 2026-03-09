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

// Control structure
typedef struct ipe_control {
  int x, y, w, h;
  int visible;
  char *name;
  void (*draw)(struct ipe_control *);
  int (*handle_event)(struct ipe_control *, int event, int x, int y);
  void *user_data;
  struct ipe_control *next;
  struct ipe_control *children; // For hierarchy
  ipe_anchors_t anchors;       // Phase 3
  ipe_size_constraints_t size_constraints; // Phase 3
  void* layout;                // Flow or Grid
  int layout_type;            // 0=none, 1=flow, 2=grid
} ipe_control_t;

// Button structure
typedef struct {
  int x, y, w, h; // Matches start of ipe_control_t for casting
  char *text;
  uint32_t bg_color;
  uint32_t text_color;
  int is_hovered;
  int is_pressed;
  void (*on_click)(void *);
} ipe_button_t;

static ipe_control_t *g_root_control = NULL;

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

IPE_EXPORT int ipe_init() {
  if (SDL_Init(SDL_INIT_VIDEO | SDL_INIT_TIMER) < 0)
    return 0;
  if (TTF_Init() < 0)
    return 0;
  int imgFlags = IMG_INIT_PNG | IMG_INIT_JPG;
  if (!(IMG_Init(imgFlags) & imgFlags))
    return 0;
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

void ipe_present() {
  if (g_current_window && g_current_window->renderer)
    SDL_RenderPresent(g_current_window->renderer);
}

IPE_EXPORT void ipe_draw_rect(int x, int y, int w, int h, uint32_t color) {
  if (!g_current_window || !g_current_window->renderer)
    return;
  SDL_SetRenderDrawColor(g_current_window->renderer, (color >> 16) & 0xFF,
                         (color >> 8) & 0xFF, color & 0xFF, 255);
  SDL_Rect rect = {x, y, w, h};
  SDL_RenderFillRect(g_current_window->renderer, &rect);
}

#include "font8x8_basic.h"

void ipe_draw_text(const char *text, int x, int y, uint32_t color) {
  if (!g_current_window || !g_current_window->renderer || !text)
    return;

  SDL_SetRenderDrawColor(g_current_window->renderer, (color >> 16) & 0xFF,
                         (color >> 8) & 0xFF, color & 0xFF, 255);

  int cursor_x = x;
  int cursor_y = y;

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
void ipe_draw_line(int x1, int y1, int x2, int y2, uint32_t color,
                   int thickness) {
  if (!g_current_window || !g_current_window->renderer)
    return;
  SDL_SetRenderDrawColor(g_current_window->renderer, (color >> 16) & 0xFF,
                         (color >> 8) & 0xFF, color & 0xFF, 255);
  if (thickness <= 1) {
    SDL_RenderDrawLine(g_current_window->renderer, x1, y1, x2, y2);
  } else {
    // Draw multiple lines for thickness (simple offset based)
    for (int i = 0; i < thickness; i++) {
        SDL_RenderDrawLine(g_current_window->renderer, x1, y1 + i, x2, y2 + i);
    }
  }
}

void ipe_draw_circle(int x, int y, int radius, uint32_t color, int filled) {
  if (!g_current_window || !g_current_window->renderer)
    return;
  SDL_SetRenderDrawColor(g_current_window->renderer, (color >> 16) & 0xFF,
                         (color >> 8) & 0xFF, color & 0xFF, 255);

  int offsetx = 0;
  int offsety = radius;
  int d = radius - 1;

  while (offsety >= offsetx) {
    if (filled) {
      SDL_RenderDrawLine(g_current_window->renderer, x - offsety, y + offsetx,
                         x + offsety, y + offsetx);
      SDL_RenderDrawLine(g_current_window->renderer, x - offsety, y - offsetx,
                         x + offsety, y - offsetx);
      SDL_RenderDrawLine(g_current_window->renderer, x - offsetx, y + offsety,
                         x + offsetx, y + offsety);
      SDL_RenderDrawLine(g_current_window->renderer, x - offsetx, y - offsety,
                         x + offsetx, y - offsety);
    } else {
      SDL_RenderDrawPoint(g_current_window->renderer, x + offsetx, y + offsety);
      SDL_RenderDrawPoint(g_current_window->renderer, x + offsety, y + offsetx);
      SDL_RenderDrawPoint(g_current_window->renderer, x - offsetx, y + offsety);
      SDL_RenderDrawPoint(g_current_window->renderer, x - offsety, y + offsetx);
      SDL_RenderDrawPoint(g_current_window->renderer, x + offsetx, y - offsety);
      SDL_RenderDrawPoint(g_current_window->renderer, x + offsety, y - offsetx);
      SDL_RenderDrawPoint(g_current_window->renderer, x - offsetx, y - offsety);
      SDL_RenderDrawPoint(g_current_window->renderer, x - offsety, y - offsetx);
    }

    if (d >= 2 * offsetx) {
      d -= 2 * offsetx + 1;
      offsetx++;
    } else if (d < 2 * (radius - offsety)) {
      d += 2 * offsety - 1;
      offsety--;
    } else {
      d += 2 * (offsety - offsetx - 1);
      offsety--;
      offsetx++;
    }
  }
}

void ipe_draw_triangle(int x1, int y1, int x2, int y2, int x3, int y3,
                       uint32_t color) {
  if (!g_current_window || !g_current_window->renderer)
    return;
  SDL_SetRenderDrawColor(g_current_window->renderer, (color >> 16) & 0xFF,
                         (color >> 8) & 0xFF, color & 0xFF, 255);
  SDL_RenderDrawLine(g_current_window->renderer, x1, y1, x2, y2);
  SDL_RenderDrawLine(g_current_window->renderer, x2, y2, x3, y3);
  SDL_RenderDrawLine(g_current_window->renderer, x3, y3, x1, y1);
}

void ipe_draw_rounded_rect(int x, int y, int w, int h, int radius,
                           uint32_t color) {
  if (!g_current_window || !g_current_window->renderer)
    return;
  ipe_draw_rect(x + radius, y, w - radius * 2, h, color);
  ipe_draw_rect(x, y + radius, w, h - radius * 2, color);
  ipe_draw_circle(x + radius, y + radius, radius, color, 1);
  ipe_draw_circle(x + w - radius, y + radius, radius, color, 1);
  ipe_draw_circle(x + radius, y + h - radius, radius, color, 1);
  ipe_draw_circle(x + w - radius, y + h - radius, radius, color, 1);
}

static void ipe_draw_arc(int x, int y, int radius, int start_angle, int end_angle, uint32_t color) {
    // Basic arc using points
    SDL_SetRenderDrawColor(g_current_window->renderer, (color >> 16) & 0xFF, (color >> 8) & 0xFF, color & 0xFF, 255);
    for (int a = start_angle; a <= end_angle; a++) {
        float rad = a * M_PI / 180.0;
        SDL_RenderDrawPoint(g_current_window->renderer, x + cos(rad) * radius, y + sin(rad) * radius);
    }
}

void ipe_draw_rect_ex(ipe_rect_style_t* style) {
    if (!g_current_window || !g_current_window->renderer) return;

    // 1. Shadow
    if (style->shadow_radius > 0) {
        uint32_t sc = style->shadow_color;
        SDL_SetRenderDrawColor(g_current_window->renderer, (sc >> 16) & 0xFF, (sc >> 8) & 0xFF, sc & 0xFF, 128);
        SDL_Rect shadow_rect = { style->x + style->shadow_offset_x, style->y + style->shadow_offset_y, style->w, style->h };
        SDL_RenderFillRect(g_current_window->renderer, &shadow_rect);
    }

    // 2. Gradient / Fill
    if (style->gradient > 0) {
        // Simple H/V gradient implementation
        for (int i = 0; i < (style->gradient == 1 ? style->w : style->h); i++) {
            float t = (float)i / (style->gradient == 1 ? style->w : style->h);
            uint8_t r = (style->gradient_color_start >> 16 & 0xFF) * (1-t) + (style->gradient_color_end >> 16 & 0xFF) * t;
            uint8_t g = (style->gradient_color_start >> 8 & 0xFF) * (1-t) + (style->gradient_color_end >> 8 & 0xFF) * t;
            uint8_t b = (style->gradient_color_start & 0xFF) * (1-t) + (style->gradient_color_end & 0xFF) * t;
            SDL_SetRenderDrawColor(g_current_window->renderer, r, g, b, (uint8_t)(style->opacity * 255));
            if (style->gradient == 1) // Horizontal
                SDL_RenderDrawLine(g_current_window->renderer, style->x + i, style->y, style->x + i, style->y + style->h);
            else // Vertical
                SDL_RenderDrawLine(g_current_window->renderer, style->x, style->y + i, style->x + style->w, style->y + i);
        }
    } else {
        uint32_t fc = style->fill_color;
        SDL_SetRenderDrawColor(g_current_window->renderer, (fc >> 16) & 0xFF, (fc >> 8) & 0xFF, fc & 0xFF, (uint8_t)(style->opacity * 255));
        
        if (style->corner_radius_tl > 0) {
            // Draw complex rounded rect (per corner)
            // Simplified for now: use ipe_draw_rounded_rect if any radius is set
             ipe_draw_rounded_rect(style->x, style->y, style->w, style->h, style->corner_radius_tl, style->fill_color);
        } else {
            SDL_Rect r = {style->x, style->y, style->w, style->h};
            SDL_RenderFillRect(g_current_window->renderer, &r);
        }
    }

    // 3. Border (Solid/Dashed placeholder)
    if (style->border_width > 0) {
        uint32_t bc = style->border_color;
        SDL_SetRenderDrawColor(g_current_window->renderer, (bc >> 16) & 0xFF, (bc >> 8) & 0xFF, bc & 0xFF, (uint8_t)(style->opacity * 255));
        for (int i=0; i<style->border_width; i++) {
            SDL_Rect r = {style->x - i, style->y - i, style->w + i*2, style->h + i*2};
            SDL_RenderDrawRect(g_current_window->renderer, &r);
        }
    }
}

void ipe_draw_circle_ex(ipe_circle_style_t* style) {
    if (!g_current_window || !g_current_window->renderer) return;
    // Radial gradient or solid
    if (style->color_center != style->color_edge) {
        for (int r = style->radius; r > 0; r--) {
            float t = (float)r / style->radius;
            uint8_t rr = (style->color_center >> 16 & 0xFF) * (1-t) + (style->color_edge >> 16 & 0xFF) * t;
            uint8_t gg = (style->color_center >> 8 & 0xFF) * (1-t) + (style->color_edge >> 8 & 0xFF) * t;
            uint8_t bb = (style->color_center & 0xFF) * (1-t) + (style->color_edge & 0xFF) * t;
            SDL_SetRenderDrawColor(g_current_window->renderer, rr, gg, bb, 255);
            ipe_draw_circle(style->x, style->y, r, (rr<<16|gg<<8|bb), 0);
        }
    } else {
        ipe_draw_circle(style->x, style->y, style->radius, style->color_center, 1);
    }
    
    if (style->border_width > 0) {
        ipe_draw_circle(style->x, style->y, style->radius, style->border_color, 0);
    }
}

void ipe_draw_polygon_ex(ipe_polygon_style_t* style) {
    if (!g_current_window || !g_current_window->renderer || !style->points) return;
    SDL_SetRenderDrawColor(g_current_window->renderer, (style->fill_color >> 16) & 0xFF, (style->fill_color >> 8) & 0xFF, style->fill_color & 0xFF, 255);
    // Very simplified polygon draw (outline only for now)
    for (int i = 0; i < style->point_count; i++) {
        int next = (i + 1) % style->point_count;
        SDL_RenderDrawLine(g_current_window->renderer, style->points[i*2], style->points[i*2+1], style->points[next*2], style->points[next*2+1]);
    }
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
            // Custom blend mode would be needed for perfect match
            sdl_mode = SDL_ComposeCustomBlendMode(SDL_BLENDFACTOR_ONE_MINUS_DST_COLOR, SDL_BLENDFACTOR_ONE, SDL_BLENDOP_ADD, SDL_BLENDFACTOR_ONE, SDL_BLENDFACTOR_ZERO, SDL_BLENDOP_ADD);
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
ipe_font_t *ipe_font_load(const char *path, int size) {
  TTF_Font *ttf_font = TTF_OpenFont(path, size);
  if (!ttf_font)
    return NULL;
  ipe_font_t *font = (ipe_font_t *)malloc(sizeof(ipe_font_t));
  font->ttf_font = ttf_font;
  font->data = NULL; // We use TTF internally
  TTF_SizeText(ttf_font, "W", &font->char_width, &font->char_height);
  return font;
}

void ipe_draw_text_ex(const char *text, int x, int y, ipe_font_t *font,
                      uint32_t color) {
  if (!g_current_window || !g_current_window->renderer || !font || !text)
    return;
  SDL_Color sdl_color = {(color >> 16) & 0xFF, (color >> 8) & 0xFF, color & 0xFF,
                         255};
  SDL_Surface *surface = TTF_RenderText_Blended(font->ttf_font, text, sdl_color);
  if (!surface)
    return;
  SDL_Texture *texture =
      SDL_CreateTextureFromSurface(g_current_window->renderer, surface);
  if (texture) {
    SDL_Rect dst = {x, y, surface->w, surface->h};
    SDL_RenderCopy(g_current_window->renderer, texture, NULL, &dst);
    SDL_DestroyTexture(texture);
  }
  SDL_FreeSurface(surface);
}

int ipe_text_width(const char *text, ipe_font_t *font) {
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
    
    ipe_control_t* child = parent->children;
    while (child) {
        if (child->visible) {
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
        child = child->next;
    }
}

// Layout Implementation
void ipe_control_set_anchors(ipe_control_t* ctrl, ipe_anchors_t anchors) {
    if (ctrl) ctrl->anchors = anchors;
}

// Animations implementation
ipe_animation_t* ipe_animate_sequence(ipe_animation_seq_t* seq) {
    // Start animation logic
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

  if (!parent->children) {
    parent->children = child;
  } else {
    ipe_control_t *curr = parent->children;
    while (curr->next) curr = curr->next;
    curr->next = child;
  }
}

void ipe_control_remove(ipe_control_t *ctrl) {
  if (!g_root_control || !ctrl) return;
  if (g_root_control == ctrl) {
    g_root_control = ctrl->next;
    return;
  }
  ipe_control_t *curr = g_root_control;
  while (curr->next && curr->next != ctrl) curr = curr->next;
  if (curr->next) curr->next = ctrl->next;
}

ipe_control_t *ipe_control_find(const char *name) {
  ipe_control_t *curr = g_root_control;
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

  while (SDL_PollEvent(&e)) {
    switch (e.type) {
    case SDL_QUIT:
      g_running = 0;
      g_last_event_type = 1;
      break;
    case SDL_MOUSEBUTTONDOWN:
      g_last_event_type = 2;
      g_mouse_x = e.button.x;
      g_mouse_y = e.button.y;
      if (g_mousedown_cb)
        g_mousedown_cb(g_mouse_x, g_mouse_y, e.button.button);
      break;
    case SDL_MOUSEBUTTONUP:
      g_last_event_type = 3;
      g_mouse_x = e.button.x;
      g_mouse_y = e.button.y;
      break;
    case SDL_MOUSEMOTION:
      g_mouse_x = e.motion.x;
      g_mouse_y = e.motion.y;
      if (g_mousemove_cb)
        g_mousemove_cb(g_mouse_x, g_mouse_y, 0);
      break;
    case SDL_KEYDOWN:
      g_last_event_type = 4;
      g_key_code = e.key.keysym.sym;
      if (g_keydown_cb)
        g_keydown_cb(g_key_code);
      break;
    case SDL_WINDOWEVENT:
      if (e.window.event == SDL_WINDOWEVENT_RESIZED) {
        if (g_resize_cb)
          g_resize_cb(e.window.data1, e.window.data2);
      }
      break;
    }
  }

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

IPE_EXPORT void ipe_window_close() {
  if (g_current_window) {
    ipe_window_destroy(g_current_window);
  }
  SDL_Quit();
}
