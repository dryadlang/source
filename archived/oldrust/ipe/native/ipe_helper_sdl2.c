#include <SDL2/SDL.h>
#include <stdint.h>

static SDL_Window *g_window = NULL;
static SDL_Renderer *g_renderer = NULL;
static int g_running = 1;

// Event state
static int g_last_event_type =
    0; // 0=None, 1=Quit, 2=MouseDown, 3=MouseUp, 4=KeyDown
static int g_mouse_x = 0;
static int g_mouse_y = 0;
static int g_key_code = 0;

int ipe_init() {
  if (SDL_Init(SDL_INIT_VIDEO) < 0)
    return 0;
  return 1;
}

void *ipe_window_create(int width, int height, const char *title) {
  g_window =
      SDL_CreateWindow(title, SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED,
                       width, height, SDL_WINDOW_SHOWN);
  if (!g_window)
    return NULL;
  g_renderer = SDL_CreateRenderer(g_window, -1, SDL_RENDERER_ACCELERATED);
  if (!g_renderer)
    return NULL;
  return (void *)g_window;
}

void ipe_clear_background(uint32_t color) {
  if (!g_renderer)
    return;
  SDL_SetRenderDrawColor(g_renderer, (color >> 16) & 0xFF, (color >> 8) & 0xFF,
                         color & 0xFF, 255);
  SDL_RenderClear(g_renderer);
}

void ipe_present() {
  if (g_renderer)
    SDL_RenderPresent(g_renderer);
}

void ipe_draw_rect(int x, int y, int w, int h, uint32_t color) {
  if (!g_renderer)
    return;
  SDL_SetRenderDrawColor(g_renderer, (color >> 16) & 0xFF, (color >> 8) & 0xFF,
                         color & 0xFF, 255);
  SDL_Rect rect = {x, y, w, h};
  SDL_RenderFillRect(g_renderer, &rect);
}

#include "font8x8_basic.h"

void ipe_draw_text(const char *text, int x, int y, uint32_t color) {
  if (!g_renderer || !text)
    return;

  SDL_SetRenderDrawColor(g_renderer, (color >> 16) & 0xFF, (color >> 8) & 0xFF,
                         color & 0xFF, 255);

  int cursor_x = x;
  int cursor_y = y;

  while (*text) {
    char c = *text;
    // Basic mapping for ASCII 32-127
    if (c >= 32 && c <= 127) {
      int index = c - 32;
      const uint8_t *bitmap = font8x8_basic[index];

      for (int row = 0; row < 8; row++) {
        for (int col = 0; col < 8; col++) {
          if (bitmap[row] & (1 << col)) {
            // Flip bit order if needed, but standard 8x8 usually LSB or MSB
            // Let's assume bit 0 is left-most for simplicity or trial-and-error
            // Actually, commonly bit 0 is right-most (LSB).
            // Let's try drawing: if (byte & (1 << col)) -> x + col
            SDL_RenderDrawPoint(g_renderer, cursor_x + col, cursor_y + row);
          }
        }
      }
    }
    cursor_x += 8; // Advance cursor
    text++;
  }
}

int ipe_process_events() {
  SDL_Event e;
  g_last_event_type = 0;

  if (SDL_PollEvent(&e)) {
    switch (e.type) {
    case SDL_QUIT:
      g_running = 0;
      g_last_event_type = 1;
      break;
    case SDL_MOUSEBUTTONDOWN:
      g_last_event_type = 2;
      g_mouse_x = e.button.x;
      g_mouse_y = e.button.y;
      break;
    case SDL_MOUSEBUTTONUP:
      g_last_event_type = 3;
      g_mouse_x = e.button.x;
      g_mouse_y = e.button.y;
      break;
    case SDL_KEYDOWN:
      g_last_event_type = 4;
      g_key_code = e.key.keysym.sym;
      break;
    }
  }
  return g_running;
}

int ipe_get_last_event() { return g_last_event_type; }
int ipe_get_mouse_x() { return g_mouse_x; }
int ipe_get_mouse_y() { return g_mouse_y; }
int ipe_get_key_code() { return g_key_code; }

int ipe_is_window_open() { return g_running; }

void ipe_window_close() {
  if (g_renderer)
    SDL_DestroyRenderer(g_renderer);
  if (g_window)
    SDL_DestroyWindow(g_window);
  SDL_Quit();
}
