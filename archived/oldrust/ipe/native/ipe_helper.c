#include <stdint.h>
#include <windows.h>

static HWND g_hwnd = NULL;
static HDC g_hdc = NULL;
static int g_running = 1;

// Event state
static int g_last_event_type =
    0; // 0=None, 1=Quit, 2=MouseDown, 3=MouseUp, 4=KeyDown
static int g_mouse_x = 0;
static int g_mouse_y = 0;
static int g_key_code = 0;

LRESULT CALLBACK WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam,
                            LPARAM lParam) {
  switch (uMsg) {
  case WM_DESTROY:
    PostQuitMessage(0);
    g_running = 0;
    g_last_event_type = 1;
    return 0;
  case WM_LBUTTONDOWN:
    g_last_event_type = 2;
    g_mouse_x = LOWORD(lParam);
    g_mouse_y = HIWORD(lParam);
    return 0;
  case WM_LBUTTONUP:
    g_last_event_type = 3;
    g_mouse_x = LOWORD(lParam);
    g_mouse_y = HIWORD(lParam);
    return 0;
  case WM_KEYDOWN:
    g_last_event_type = 4;
    g_key_code = (int)wParam;
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

  if (!RegisterClass(&wc))
    return 0;
  return 1;
}

void *ipe_window_create(int width, int height, const char *title) {
  g_hwnd = CreateWindowEx(0, "IpeWindowClass", title,
                          WS_OVERLAPPEDWINDOW | WS_VISIBLE, CW_USEDEFAULT,
                          CW_USEDEFAULT, width, height, NULL, NULL,
                          GetModuleHandle(NULL), NULL);

  if (g_hwnd) {
    g_hdc = GetDC(g_hwnd);
  }
  return (void *)g_hwnd;
}

void ipe_clear_background(uint32_t color) {
  if (!g_hwnd)
    return;
  RECT rect;
  GetClientRect(g_hwnd, &rect);
  HBRUSH brush = CreateSolidBrush(
      RGB((color >> 16) & 0xFF, (color >> 8) & 0xFF, color & 0xFF));
  FillRect(g_hdc, &rect, brush);
  DeleteObject(brush);
}

void ipe_present() {
  // GDI draws directly to the window DC
}

void ipe_draw_rect(int x, int y, int w, int h, uint32_t color) {
  if (!g_hdc)
    return;
  HBRUSH brush = CreateSolidBrush(
      RGB((color >> 16) & 0xFF, (color >> 8) & 0xFF, color & 0xFF));
  RECT rect = {x, y, x + w, y + h};
  FillRect(g_hdc, &rect, brush);
  DeleteObject(brush);
}

int ipe_process_events() {
  MSG msg;
  g_last_event_type = 0;
  while (PeekMessage(&msg, NULL, 0, 0, PM_REMOVE)) {
    if (msg.message == WM_QUIT) {
      g_running = 0;
    }
    TranslateMessage(&msg);
    DispatchMessage(&msg);
  }
  return g_running;
}

int ipe_get_last_event() { return g_last_event_type; }
int ipe_get_mouse_x() { return g_mouse_x; }
int ipe_get_mouse_y() { return g_mouse_y; }
int ipe_get_key_code() { return g_key_code; }

int ipe_is_window_open() { return g_running; }

void ipe_window_close() {
  if (g_hwnd) {
    ReleaseDC(g_hwnd, g_hdc);
    DestroyWindow(g_hwnd);
    g_hwnd = NULL;
  }
}
