#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include "wasm_export.h"

#define BUF_SIZE 100000
static char error_buf[128];

/* Read WASM file into buffer */
static bool read_wasm_file(const char *filename, uint8_t **buf, uint32_t *size) {
    FILE *f = fopen(filename, "rb");
    if (!f) {
        printf("Failed to open wasm file: %s\n", filename);
        return false;
    }

    fseek(f, 0, SEEK_END);
    *size = (uint32_t)ftell(f);
    fseek(f, 0, SEEK_SET);

    *buf = (uint8_t *)malloc(*size);
    if (!*buf) {
        fclose(f);
        return false;
    }

    if (fread(*buf, *size, 1, f) != 1) {
        free(*buf);
        fclose(f);
        return false;
    }

    fclose(f);
    return true;
}

static uint8_t* get_image_data(uint32_t *size) { 
    *size = 0;
    return NULL; 
}

int main(void) {
    wasm_module_t module = NULL;
    wasm_module_inst_t module_inst = NULL;
    RuntimeInitArgs init_args;
    uint8_t *wasm_buffer = NULL;
    uint32_t wasm_size = 0;

    memset(&init_args, 0, sizeof(init_args));
    init_args.mem_alloc_type = Alloc_With_Allocator;
    init_args.mem_alloc_option.allocator.malloc_func = malloc;
    init_args.mem_alloc_option.allocator.realloc_func = realloc;
    init_args.mem_alloc_option.allocator.free_func = free;

    if (!wasm_runtime_full_init(&init_args)) {
        printf("Failed to init WAMR\n");
        return 1;
    }
    
    if (!read_wasm_file("grayscale_project.wasm", &wasm_buffer, &wasm_size)) {
        printf("Failed to load WASM file\n");
        wasm_runtime_destroy();
        return 1;
    }

    module = wasm_runtime_load(wasm_buffer, wasm_size, error_buf, sizeof(error_buf));
    if (!module) {
        printf("Failed to load WASM module: %s\n", error_buf);
        free(wasm_buffer);
        wasm_runtime_destroy();
        return 1;
    }

    module_inst = wasm_runtime_instantiate(module, 8192, 8192, error_buf, sizeof(error_buf));
    if (!module_inst) {
        printf("Failed to instantiate WASM module: %s\n", error_buf);
        wasm_runtime_unload(module);
        free(wasm_buffer);
        wasm_runtime_destroy();
        return 1;
    }

    wasm_function_inst_t func = wasm_runtime_lookup_function(module_inst, "process_hex_image");
    if (!func) {
        printf("Failed to find process_hex_image function\n");
        wasm_runtime_deinstantiate(module_inst);
        wasm_runtime_unload(module);
        free(wasm_buffer);
        wasm_runtime_destroy();
        return 1;
    }

    uint32_t image_size;
    uint8_t *image_data = get_image_data(&image_size);
    wasm_val_t args[1] = { {.kind = WASM_I32, .of.i32 = 0} };

    wasm_runtime_deinstantiate(module_inst);
    wasm_runtime_unload(module);
    free(wasm_buffer);
    wasm_runtime_destroy();
    return 0;
}