// C ABI runtime stubs for native AOT modules (no C++ runtime dependency).

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct DryadBox {
    int tag;
    int64_t i;
    double f;
    unsigned char b;
    char* str_owned;
    void* heap_ptr;
} DryadBox;

typedef struct DryadArray {
    void** items;
    size_t length;
    size_t capacity;
} DryadArray;

static DryadBox g_box;

/* Bump arena: one allocation per process, no per-box heap churn (embedded-friendly). */
#ifndef DRYAD_ARENA_DEFAULT_BYTES
#define DRYAD_ARENA_DEFAULT_BYTES (64 * 1024)
#endif

static uint8_t* g_arena = NULL;
static size_t g_arena_size = 0;
static size_t g_arena_used = 0;

static void arena_init(void) {
    if (g_arena != NULL) {
        return;
    }
    size_t size = DRYAD_ARENA_DEFAULT_BYTES;
    const char* env = getenv("DRYAD_ARENA_BYTES");
    if (env != NULL && env[0] != '\0') {
        unsigned long parsed = strtoul(env, NULL, 10);
        if (parsed > 0) {
            size = (size_t)parsed;
        }
    }
    g_arena = (uint8_t*)malloc(size);
    if (!g_arena) {
        return;
    }
    g_arena_size = size;
    g_arena_used = 0;
}

static DryadBox* box_alloc(void) {
    arena_init();
    if (!g_arena || g_arena_used + sizeof(DryadBox) > g_arena_size) {
        return NULL;
    }
    DryadBox* box = (DryadBox*)(g_arena + g_arena_used);
    g_arena_used += sizeof(DryadBox);
    memset(box, 0, sizeof(DryadBox));
    return box;
}

static void* box_null(void);

static const char* argv_cstring(char** argv, int32_t index) {
    if (!argv || index < 0) {
        return NULL;
    }
    return argv[index];
}

static void* box_int(int64_t value) {
    DryadBox* box = box_alloc();
    if (!box) {
        return box_null();
    }
    box->tag = 1;
    box->i = value;
    return box;
}

static void* box_float(double value) {
    DryadBox* box = box_alloc();
    if (!box) {
        return box_null();
    }
    box->tag = 2;
    box->f = value;
    return box;
}

static void* box_null(void) {
    DryadBox* box = box_alloc();
    if (!box) {
        g_box.tag = 0;
        return &g_box;
    }
    box->tag = 0;
    return box;
}

static DryadArray* array_from_box(DryadBox* box) {
    if (!box || box->tag != 5 || !box->heap_ptr) {
        return NULL;
    }
    return (DryadArray*)box->heap_ptr;
}

static DryadBox* box_array(DryadArray* array) {
    DryadBox* box = box_alloc();
    if (!box) {
        return (DryadBox*)box_null();
    }
    box->tag = 5;
    box->heap_ptr = array;
    return box;
}

static int64_t unbox_index(void* value) {
    DryadBox* box = (DryadBox*)value;
    if (!box) {
        return 0;
    }
    if (box->tag == 1) {
        return box->i;
    }
    if (box->tag == 2) {
        return (int64_t)box->f;
    }
    return 0;
}

void* dryad_box_i32(int32_t value) {
    return box_int(value);
}

void* dryad_box_f64(double value) {
    return box_float(value);
}

void* dryad_box_bool(int8_t value) {
    DryadBox* box = box_alloc();
    if (!box) {
        return box_null();
    }
    box->tag = 3;
    box->b = value ? 1 : 0;
    return box;
}

int32_t dryad_unbox_i32(void* value) {
    DryadBox* box = (DryadBox*)value;
    if (!box || box->tag != 1) {
        return 0;
    }
    return (int32_t)box->i;
}

double dryad_unbox_f64(void* value) {
    DryadBox* box = (DryadBox*)value;
    if (!box) {
        return 0.0;
    }
    if (box->tag == 2) {
        return box->f;
    }
    if (box->tag == 1) {
        return (double)box->i;
    }
    return 0.0;
}

int8_t dryad_unbox_bool(void* value) {
    DryadBox* box = (DryadBox*)value;
    if (!box || box->tag != 3) {
        return 0;
    }
    return box->b ? 1 : 0;
}

void* dryad_variant_add(void* left, void* right) {
    DryadBox* a = (DryadBox*)left;
    DryadBox* b = (DryadBox*)right;
    if (!a || !b) {
        return box_null();
    }
    if (a->tag == 1 && b->tag == 1) {
        return box_int(a->i + b->i);
    }
    if (a->tag == 2 || b->tag == 2) {
        double av = a->tag == 2 ? a->f : (double)a->i;
        double bv = b->tag == 2 ? b->f : (double)b->i;
        return box_float(av + bv);
    }
    return box_null();
}

void* dryad_new_array(void) {
    DryadArray* array = (DryadArray*)calloc(1, sizeof(DryadArray));
    if (!array) {
        return box_null();
    }
    array->capacity = 4;
    array->items = (void**)calloc(array->capacity, sizeof(void*));
    if (!array->items) {
        free(array);
        return box_null();
    }
    return box_array(array);
}

void* dryad_array_push(void* array_value, void* element_value) {
    DryadArray* array = array_from_box((DryadBox*)array_value);
    if (!array) {
        return box_null();
    }
    if (array->length >= array->capacity) {
        size_t new_capacity = array->capacity == 0 ? 4 : array->capacity * 2;
        void** items = (void**)realloc(array->items, new_capacity * sizeof(void*));
        if (!items) {
            return array_value;
        }
        array->items = items;
        array->capacity = new_capacity;
    }
    array->items[array->length++] = element_value;
    return array_value;
}

void* dryad_array_set(void* array_value, void* index_value, void* element_value) {
    DryadArray* array = array_from_box((DryadBox*)array_value);
    if (!array) {
        return box_null();
    }
    int64_t index = unbox_index(index_value);
    if (index < 0) {
        return array_value;
    }
    while ((size_t)index >= array->length) {
        dryad_array_push(array_value, box_null());
    }
    array->items[index] = element_value;
    return array_value;
}

void* dryad_variant_get_element(void* array_value, void* index_value) {
    DryadArray* array = array_from_box((DryadBox*)array_value);
    if (!array) {
        return box_null();
    }
    int64_t index = unbox_index(index_value);
    if (index < 0 || (size_t)index >= array->length) {
        return box_null();
    }
    void* item = array->items[index];
    return item ? item : box_null();
}

static void print_box(DryadBox* box) {
    if (!box) {
        fputs("null\n", stdout);
        return;
    }
    switch (box->tag) {
        case 1:
            printf("%lld\n", (long long)box->i);
            break;
        case 2:
            printf("%g\n", box->f);
            break;
        case 3:
            fputs(box->b ? "true\n" : "false\n", stdout);
            break;
        case 4:
            fputs(box->str_owned ? box->str_owned : "", stdout);
            fputs("\n", stdout);
            break;
        default:
            fputs("null\n", stdout);
            break;
    }
}

void dryad_print(void* value) {
    print_box((DryadBox*)value);
}

static int8_t variant_compare_eq(DryadBox* left, DryadBox* right) {
    if (!left || !right) {
        return left == right ? 1 : 0;
    }
    if (left->tag != right->tag) {
        return 0;
    }
    switch (left->tag) {
        case 1:
            return left->i == right->i ? 1 : 0;
        case 2:
            return left->f == right->f ? 1 : 0;
        case 3:
            return left->b == right->b ? 1 : 0;
        case 4: {
            const char* left_text = left->str_owned ? left->str_owned : "";
            const char* right_text = right->str_owned ? right->str_owned : "";
            return strcmp(left_text, right_text) == 0 ? 1 : 0;
        }
        default:
            return 0;
    }
}

int8_t dryad_variant_eq(void* left, void* right) {
    return variant_compare_eq((DryadBox*)left, (DryadBox*)right);
}

int8_t dryad_variant_neq(void* left, void* right) {
    return variant_compare_eq((DryadBox*)left, (DryadBox*)right) ? 0 : 1;
}

void* dryad_intrinsic_slow(uint16_t intrinsic_id, uint8_t argc) {
    (void)intrinsic_id;
    (void)argc;
    return box_null();
}

void* dryad_box_lit_cstring(const char* text) {
    DryadBox* box = box_alloc();
    if (!box) {
        return box_null();
    }
    box->tag = 4;
    box->str_owned = (char*)text;
    return box;
}

void* dryad_box_cstring(const char* text) {
    DryadBox* box = box_alloc();
    if (!box) {
        return box_null();
    }
    box->tag = 4;
    box->str_owned = text ? strdup(text) : NULL;
    return box;
}

void* dryad_argv_box_cstring(char** argv, int32_t index) {
    const char* text = argv_cstring(argv, index);
    DryadBox* box = box_alloc();
    if (!box) {
        return box_null();
    }
    box->tag = 4;
    box->str_owned = text ? strdup(text) : NULL;
    return box;
}

void* dryad_argv_box_i32(char** argv, int32_t index) {
    const char* text = argv_cstring(argv, index);
    if (!text) {
        return box_null();
    }
    DryadBox* box = box_alloc();
    if (!box) {
        return box_null();
    }
    box->tag = 1;
    box->i = (int64_t)strtol(text, NULL, 10);
    return box;
}

void* dryad_argv_box_f64(char** argv, int32_t index) {
    const char* text = argv_cstring(argv, index);
    if (!text) {
        return box_null();
    }
    DryadBox* box = box_alloc();
    if (!box) {
        return box_null();
    }
    box->tag = 2;
    box->f = strtod(text, NULL);
    return box;
}

void* dryad_argv_box_bool(char** argv, int32_t index) {
    const char* text = argv_cstring(argv, index);
    if (!text) {
        return box_null();
    }
    DryadBox* box = box_alloc();
    if (!box) {
        return box_null();
    }
    box->tag = 3;
    box->b = (strcmp(text, "1") == 0 || strcmp(text, "true") == 0 || strcmp(text, "TRUE") == 0) ? 1 : 0;
    return box;
}
