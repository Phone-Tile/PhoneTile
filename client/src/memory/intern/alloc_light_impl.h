/*
 * alloc_light_impl.h
 * This file is part of PixSea
 *
 * Copyright (C) 2022 Mera Emmanuel <emmanuel.mera@live.fr>.
 *
 * MIT License (MIT), http://opensource.org/licenses/MIT
 * Full license can be found in the LICENSE file
 */

#ifndef alloc_light_impl_h
#define alloc_light_impl_h

// Windows aligned_alloc function
#ifdef __MINGW32__
    #include <malloc.h>
    #define aligned_alloc(aligment, size) __mingw_aligned_malloc(size, aligment)
#endif // __MINGW32__

#include <stdlib.h>
#include <stdbool.h>

#define ALIGN_4(val) (val & 0x3) == 0x3 ? (val+1) : ((val & 0x2) == 0x2 ? (val+2) : ((val & 0x1) == 0x1 ? (val+3) : val))

void MEM_light_free(void *vmem);
void *MEM_light_malloc(size_t len, const char *str);
void *MEM_light_malloc_array(size_t len, size_t size, const char *str);
void *MEM_light_malloc_aligned(size_t len, size_t alignment, const char *str);
void *MEM_light_calloc(size_t len, const char *str);
void *MEM_light_calloc_array(size_t len, size_t size, const char *str);
void *MEM_light_dupalloc(void *vmem, const char *str);
void *MEM_light_realloc_id(void *vmem, size_t len, const char *str);
void *MEM_light_realloc(void *vmem, size_t len);
int MEM_light_get_mem_in_use(void);
int MEM_light_get_totblock(void);
void MEM_light_print_memstats(void);
bool MEM_light_check_memory(void);

#endif /* alloc_light_impl_h */
