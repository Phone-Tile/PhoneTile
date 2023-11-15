/*
 * alloc_secured_impl.h
 * This file is part of PixSea
 *
 * Copyright (C) 2022 Mera Emmanuel <emmanuel.mera@live.fr>.
 *
 * MIT License (MIT), http://opensource.org/licenses/MIT
 * Full license can be found in the LICENSE file
 */

#ifndef alloc_secured_impl_h
#define alloc_secured_impl_h

// Windows aligned_alloc function
#ifdef __MINGW32__
    #include <malloc.h>
    #define aligned_alloc(aligment, size) __mingw_aligned_malloc(size, aligment)
#endif // __MINGW32__

#include <stdlib.h>

#define ALIGN_4(val) (val & 0x3) == 0x3 ? (val+1) : ((val & 0x2) == 0x2 ? (val+2) : ((val & 0x1) == 0x1 ? (val+3) : val))

void *MEM_secured_malloc(size_t len, const char *str);
void *MEM_secured_malloc_array(size_t len, size_t size, const char *str);
void *MEM_secured_malloc_aligned(size_t len, size_t alignment, const char *str);
void *MEM_secured_calloc(size_t len, const char *str);
void *MEM_secured_calloc_array(size_t len, size_t size, const char *str);
void *MEM_secured_dupalloc(void *vmem, const char *str);
void *MEM_secured_realloc(void *vmem, size_t len);
void *MEM_secured_realloc_id(void *vmem, size_t len, const char *str);
void MEM_secured_free(void *vmem);

#include <stdbool.h>

void MEM_secured_print_memstats(void);
bool MEM_secured_check_memory(void);
int MEM_secured_get_mem_in_use(void);
int MEM_secured_get_totblock(void);

#endif /* alloc_secured_impl_h */
