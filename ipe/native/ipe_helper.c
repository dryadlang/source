#include <windows.h>
#include <stdint.h>

static HWND g_hwnd = NULL;
static HDC g_hdc = NULL;
static int g_running = 1;

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
