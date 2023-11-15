/*
 * alloc_light_impl.c
 * This file is part of PixSea
 *
 * Copyright (C) 2022 Mera Emmanuel <emmanuel.mera@live.fr>.
 *
 * MIT License (MIT), http://opensource.org/licenses/MIT
 * Full license can be found in the LICENSE file
 */

#include "alloc_light_impl.h"

#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h> // memcpy, memset, strcmp
#include <stddef.h> // offsetof
#include <assert.h>
#include <stdint.h>

#include <pthread.h>

/* ---------------------------- */
/*  structures & defines        */
/* ---------------------------- */

#define MEM_ALIGNED_FLAG 1

typedef struct {
    size_t len;
} MemHead;

typedef struct {
    short alignment;
    size_t len;
} MemHeadAligned;

#define MEMHEAD_IS_ALIGNED(memh) (memh->len & MEM_ALIGNED_FLAG)
#define ALIGNMENT(vmem) (((MemHeadAligned *)vmem) - 1)

/* ---------------------------- */
/*  local variables             */
/* ---------------------------- */

static unsigned int totblock = 0;
static unsigned long long mem_in_use = 0;

static pthread_mutex_t mem_lock = PTHREAD_MUTEX_INITIALIZER;

/* ---------------------------- */
/*  local functions             */
/* ---------------------------- */
/*
static void rm_memblock(MemHead *memh);
static void make_mem_header(MemHead *memh, size_t len, const char *str);
static void rm_memblock_from_memlist(MemHead *memh);
static void add_memblock(MemHead *memh);
static const char *check_memlist(MemHead *memh);
*/
static void mem_lock_thread()
{
    pthread_mutex_lock(&mem_lock);
}

static void mem_unlock_thread()
{
    pthread_mutex_unlock(&mem_lock);
}

static void print_error(const char *format, ...)
{
    va_list ap;
    char buffer[1024];
    
    va_start(ap, format);
    vsnprintf(buffer, sizeof(buffer), format, ap);
    va_end(ap);
    buffer[sizeof(buffer) - 1] = '\0';
    
    printf("[\033[31m MEMORY ERROR \033[00m] : ");
    fflush(stdout);
    fputs(buffer, stderr);
    fputs("\n", stderr);
}

/* ---------------------------- */
/*  implementation              */
/* ---------------------------- */

void MEM_light_free(void *vmem) {
    MemHead *memh = vmem;
    
    if (memh == NULL) {
        print_error("attempt to free NULL pointer");
        return;
    }
    
    if (sizeof(intptr_t) == 8) {
        if ((intptr_t)memh & 0x7) {
            print_error("attempt to free illegal pointer");
            return;
        }
    } else {
        if ((intptr_t)memh & 0x3) {
            print_error("attempt to free illegal pointer");
            return;
        }
    }
    memh--;
    
    mem_lock_thread();
    
    totblock--;
    mem_in_use -= memh->len;
    
    mem_unlock_thread();
    
    if (MEMHEAD_IS_ALIGNED(memh)) {
        free(ALIGNMENT(vmem));
    } else {
        free(memh);
    }
}

void *MEM_light_malloc(size_t len, const char *str) {
    MemHead *memh;
    
    len = ALIGN_4(len);
    
    memh = (MemHead *)malloc( len + sizeof(MemHead) );
    
    if (memh) {
        memh->len = len;
        
        mem_lock_thread();
        
        totblock += 1;
        mem_in_use += len;
        
        mem_unlock_thread();
        
        return ++memh;
    }
    print_error("Malloc : unable to allocate memory : len=%u for %s, total : %uMib",
                (unsigned int)len,
                str,
                (unsigned int)mem_in_use / (1024*1024));
    return NULL;
}

void *MEM_light_malloc_array(size_t len, size_t size, const char *str) {
    return MEM_light_malloc(len*size, str);
}

void *MEM_light_malloc_aligned(size_t len, size_t alignment, const char *str) {
    MemHeadAligned *memh;
    
    if (alignment < 8) {
        alignment = 8;
    }
    assert(alignment < 1024 && ((alignment - 1) & alignment) == 0);
    
    /* set the padding needed for this alignment */
    size_t padding = alignment - (sizeof(MemHeadAligned) % alignment);
    
    len = ALIGN_4(len);
    /* this make sure the size of the total block is a multiple of alignment */
    size_t extra_padding = alignment - (len % alignment);
    
    memh = (MemHeadAligned *)aligned_alloc(alignment, len + sizeof(MemHeadAligned) + padding + extra_padding );
    
    if (memh) {
        memh->len = len | MEM_ALIGNED_FLAG;
        memh->alignment = alignment;
        
        mem_lock_thread();
        
        totblock += 1;
        mem_in_use += len;
        
        mem_unlock_thread();
        
        return ++memh;
    }
    print_error("Malloc : unable to allocate memory : len=%u for %s, total : %uMib",
                (unsigned int)len,
                str,
                (unsigned int)mem_in_use / (1024*1024));
    return NULL;
}

void *MEM_light_calloc(size_t len, const char *str) {
    MemHead *memh;
    
    len = ALIGN_4(len);
    
    memh = (MemHead *)calloc( 1 , len + sizeof(MemHead) );
    
    if (memh) {
        memh->len = len;
        
        mem_lock_thread();
        
        totblock += 1;
        mem_in_use += len;
        
        mem_unlock_thread();
        
        return ++memh;
    }
    print_error("Malloc : unable to allocate memory : len=%u for %s, total : %uMib",
                (unsigned int)len,
                str,
                (unsigned int)mem_in_use / (1024*1024));
    return NULL;
}

void *MEM_light_calloc_array(size_t len, size_t size, const char *str) {
    size_t total_len = len*size;
    return MEM_light_calloc(total_len, str);
}

void *MEM_light_dupalloc(void *vmem, const char *str) {
    MemHead *memh = vmem;
    if (memh == NULL) {
        return NULL;
    }
    memh--;
    
    MemHead *dupmemh;
    
    if (MEMHEAD_IS_ALIGNED(memh)) {
        dupmemh = MEM_light_malloc_aligned(memh->len, ALIGNMENT(vmem)->alignment, str);
    } else {
        dupmemh = MEM_light_malloc(memh->len, str);
    }

    if (dupmemh) {
        memcpy(dupmemh, vmem, memh->len);
    } else {
        return NULL;
    }
    
    return dupmemh;
}

void *MEM_light_realloc_id(void *vmem, size_t len, const char *str) {
    if (vmem) {
        MemHead *memh = vmem;
        void *newmemh;
        
        memh--;
        
        if (MEMHEAD_IS_ALIGNED(memh)) {
            newmemh = MEM_light_malloc_aligned(len, ALIGNMENT(vmem)->alignment, str);
        } else {
            newmemh = MEM_light_calloc(len, str);
        }
        if (newmemh) {
            if (len > memh->len) {
                memcpy(newmemh, vmem, memh->len);
            } else {
                memcpy(newmemh, vmem, len);
            }
        }
        MEM_light_free(vmem);
        return newmemh;
    } else {
        return MEM_light_malloc(len, str);
    }
}

void *MEM_light_realloc(void *vmem, size_t len) {
    MemHead *memh = vmem;
    if (memh == NULL) {
        return MEM_light_malloc(len, "NULL");
    }
    return MEM_light_realloc_id(memh, len, "realloc");
}

int MEM_light_get_mem_in_use(void) {
    size_t _mem_in_use;
    
    mem_lock_thread();
    _mem_in_use = mem_in_use;
    mem_unlock_thread();
    
    return _mem_in_use;
}

int MEM_light_get_totblock(void) {
    unsigned int _totblock;
    
    mem_lock_thread();
    _totblock = totblock;
    mem_unlock_thread();
    
    return totblock;
}

bool MEM_light_check_memory(void) {
    return true;
}

void MEM_light_print_memstats(void) {
    mem_lock_thread();
    
    printf("Total memory allocated : %.3f Mio\n", (double) mem_in_use / (double) (1024*1024));
    printf("Number of allocated block : %d\n", totblock);
    
    mem_unlock_thread();
}
