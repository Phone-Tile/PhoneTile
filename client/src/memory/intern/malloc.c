/*
 * malloc.c
 * This file is part of PixSea
 *
 * Copyright (C) 2022 Mera Emmanuel <emmanuel.mera@live.fr>.
 *
 * MIT License (MIT), http://opensource.org/licenses/MIT
 * Full license can be found in the LICENSE file
 */

#include "alloc_secured_impl.h"
#include "alloc_light_impl.h"

#include <assert.h>

void *(*MEM_malloc)(size_t len, const char *str) = MEM_light_malloc;
void *(*MEM_malloc_array)(size_t len, size_t size, const char *str) = MEM_light_malloc_array;
void *(*MEM_malloc_aligned)(size_t len, size_t alignment, const char *str) = MEM_light_malloc_aligned;
void *(*MEM_calloc)(size_t len, const char *str) = MEM_light_calloc;
void *(*MEM_calloc_array)(size_t len, size_t size, const char *str) = MEM_light_calloc_array;
void *(*MEM_dupalloc)(void *vmem, const char *str) = MEM_light_dupalloc;
void *(*MEM_realloc)(void *vmem, size_t len) = MEM_light_realloc;
void *(*MEM_realloc_id)(void *vmem, size_t len, const char *str) = MEM_light_realloc_id;
void (*MEM_free)(void *vmem) = MEM_light_free;
void (*MEM_print_memstats)(void) = MEM_light_print_memstats;
bool (*MEM_check_memory)(void) = MEM_light_check_memory;
int (*MEM_get_mem_in_use)(void) = MEM_light_get_mem_in_use;
int (*MEM_get_totblock)(void) = MEM_light_get_totblock;

void MEM_use_secured_allocator() {
    assert(MEM_get_totblock() == 0);
    
    MEM_malloc = MEM_secured_malloc;
    MEM_malloc_array = MEM_secured_malloc_array;
    MEM_malloc_aligned = MEM_secured_malloc_aligned;
    MEM_calloc = MEM_secured_calloc;
    MEM_calloc_array = MEM_secured_calloc_array;
    MEM_dupalloc = MEM_secured_dupalloc;
    MEM_realloc = MEM_secured_realloc;
    MEM_realloc_id = MEM_secured_realloc_id;
    MEM_free = MEM_secured_free;
    MEM_print_memstats = MEM_secured_print_memstats;
    MEM_check_memory = MEM_secured_check_memory;
    MEM_get_mem_in_use = MEM_secured_get_mem_in_use;
    MEM_get_totblock = MEM_secured_get_totblock;
}

void MEM_use_light_allocator() {
    assert(MEM_get_totblock() == 0);
    
    MEM_malloc = MEM_light_malloc;
    MEM_malloc_array = MEM_light_malloc_array;
    MEM_malloc_aligned = MEM_light_malloc_aligned;
    MEM_calloc = MEM_light_calloc;
    MEM_calloc_array = MEM_light_calloc_array;
    MEM_dupalloc = MEM_light_dupalloc;
    MEM_realloc = MEM_light_realloc;
    MEM_realloc_id = MEM_light_realloc_id;
    MEM_free = MEM_light_free;
    MEM_print_memstats = MEM_light_print_memstats;
    MEM_check_memory = MEM_light_check_memory;
    MEM_get_mem_in_use = MEM_light_get_mem_in_use;
    MEM_get_totblock = MEM_light_get_totblock;
}
