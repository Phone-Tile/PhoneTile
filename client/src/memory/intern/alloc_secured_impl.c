/*
 * alloc_secured_impl.c
 * This file is part of PixSea
 *
 * Copyright (C) 2022 Mera Emmanuel <emmanuel.mera@live.fr>.
 *
 * MIT License (MIT), http://opensource.org/licenses/MIT
 * Full license can be found in the LICENSE file
 */

#include "alloc_secured_impl.h"

#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h> // memcpy, memset, strcmp
#include <stddef.h> // offsetof
#include <assert.h>
#include <stdint.h>

#include <pthread.h> // need a multy-platform alternativ

/* ---------------------------- */
/*  structures                  */
/* ---------------------------- */

typedef struct {
    void *first, *last;
} localListBase;

typedef struct localList{
    struct localList *next, *prev;
} localList;

typedef struct {
    int tag1;
    size_t len;
    void *next, *prev;
    const char *name;
    const char *nextname; // use to check memory overflow
    short alignment;
    int tag2;
} MemHead;

typedef struct {
    int tag3;
} MemTail;

enum MEM_TAGS {
    MEM_TAG1,
    MEM_TAG2,
    MEM_TAG3,
    MEM_FREE
};

/* ---------------------------- */
/*  local variables             */
/* ---------------------------- */

static volatile localListBase _membase;
static volatile localListBase *membase = &_membase;

static unsigned int totblock = 0;
static unsigned long long mem_in_use = 0;

static pthread_mutex_t mem_lock = PTHREAD_MUTEX_INITIALIZER;

/* ---------------------------- */
/*  local functions             */
/* ---------------------------- */

static void rm_memblock(MemHead *memh);
static void make_mem_header(MemHead *memh, size_t len, const char *str);
static void rm_memblock_from_memlist(MemHead *memh);
static void add_memblock(MemHead *memh);
static const char *check_memlist(MemHead *memh);

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
    
    printf("[\e[31m MEMORY ERROR \e[00m] : ");
    fflush(stdout);
    fputs(buffer, stderr);
    fputs("\n", stderr);
}

void print_mem_error(const char *block, const char *error)
{
    print_error("Block : %s : %s", block, error);
}

/* ---------------------------- */
/*  implementation              */
/* ---------------------------- */

void *MEM_secured_malloc(size_t len, const char *str)
{
    MemHead *memh;
    
    len = ALIGN_4(len);
    
    memh = (MemHead *)malloc( len + sizeof(MemHead) + sizeof(MemTail) );
    
    if (memh) {
        make_mem_header(memh, len, str);
        return ++memh;
    }
    print_error("Malloc : unable to allocate memory : len=%u for %s, total : %uMib",
                (unsigned int)len,
                str,
                (unsigned int)mem_in_use / (1024*1024));
    return NULL;
}

void *MEM_secured_malloc_aligned(size_t len, size_t alignment, const char *str)
{
    MemHead *memh;
    
    /* less than 8 alignment not supported */
    if (alignment < 8) {
        alignment = 8;
    }
    /* we don't want big alignment since it doesn't make sence.
     * The second part make sure alignment is a power of 2 */
    assert(alignment < 1024 && ((alignment - 1) & alignment) == 0);
    
    /* set the padding needed for this alignment */
    size_t padding = alignment - (sizeof(MemHead) % alignment);
    
    len = ALIGN_4(len);
    /* this make sure the size of the total block is a multiple of alignment */
    size_t extra_padding = alignment - ((sizeof(MemTail) + len) % alignment);
    
    memh = (MemHead *)aligned_alloc(alignment, padding + sizeof(MemHead) + len + sizeof(MemTail) + extra_padding);
    
    if (memh) {
        /* aligne the head so its steal accessible from the pointer */
        memh = (MemHead *)((char *)memh + padding);
        
        make_mem_header(memh, len, str);
        memh->alignment = (short)alignment;
        return ++memh;
    }
    print_error("Aligned malloc : unable to allocate memory : len=%u for %s, total : %uMib",
                (unsigned int)len,
                str,
                (unsigned int)mem_in_use / (1024*1024));
    
    return NULL;
}

void *MEM_secured_malloc_array(size_t len, size_t size, const char *str)
{
    size_t total_len = len*size;
    return MEM_secured_malloc(total_len, str);
}

void *MEM_secured_calloc(size_t len, const char *str)
{
    MemHead *memh;
    
    len = ALIGN_4(len);
    
    memh = (MemHead *)calloc( len + sizeof(MemHead) + sizeof(MemTail), 1 );
    
    if (memh) {
        make_mem_header(memh, len, str);
        return ++memh;
    }
    print_error("Malloc : unable to allocate memory : len=%u for %s, total : %uMib",
                (unsigned int)len,
                str,
                (unsigned int)mem_in_use / (1024*1024));
    return NULL;
}

void *MEM_secured_calloc_array(size_t len, size_t size, const char *str)
{
    size_t total_len = len*size;
    return MEM_secured_calloc(total_len, str);
}

void *MEM_secured_dupalloc(void *vmem, const char *str)
{
    MemHead *memh = vmem;
    if (memh == NULL) {
        return NULL;
    }
    memh--;
    
    MemHead *dupmemh;
    
    if (memh->alignment == 0) {
        dupmemh = MEM_secured_malloc(memh->len, str);
    } else {
        dupmemh = MEM_secured_malloc_aligned(memh->len, memh->alignment, str);
    }

    if (dupmemh) {
        memcpy(dupmemh, vmem, memh->len);
    } else {
        return NULL;
    }
    
    return dupmemh;
}

void *MEM_secured_realloc(void *vmem, size_t len)
{
    MemHead *memh = vmem;
    if (memh == NULL) {
        return MEM_secured_malloc(len, "NULL");
    }
    memh--;
    return MEM_secured_realloc_id(vmem, len, memh->name);
}

void *MEM_secured_realloc_id(void *vmem, size_t len, const char *str)
{
    MemHead *newmemh;
    
    if (vmem) {
        MemHead *memh = vmem;
        
        memh--;
        
        if (memh->alignment == 0) {
            newmemh = MEM_secured_calloc(len, str);
        } else {
            newmemh = MEM_secured_malloc_aligned(len, memh->alignment, str);
        }

        if (newmemh) {
            if (len > memh->len) {
                memcpy(newmemh, vmem, memh->len);
            } else {
                memcpy(newmemh, vmem, len);
            }
        }
        MEM_secured_free(vmem);
        return newmemh;
    } else {
        return MEM_secured_malloc(len, str);
    }
}

void MEM_secured_free(void *vmem)
{
    MemHead *memh = vmem;
    MemTail *memt;
    const char *name;
    
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
    
    if (memh->tag1 == MEM_FREE && memh->tag2 == MEM_FREE) {
        print_mem_error("free", "attempt to free a pointer already freed");
        return;
    }
    
    if (memh->tag1 == MEM_TAG1 && memh->tag2 == MEM_TAG2 && (memh->len & 0x3) == 0) {
        memt = (MemTail *)(((char *)memh) + sizeof(MemHead) + memh->len);
        if (memt->tag3 == MEM_TAG3) {
            
            memh->tag1 = MEM_FREE;
            memh->tag2 = MEM_FREE;
            memt->tag3 = MEM_FREE;
            
            rm_memblock(memh);
            
            return;
        } else {
            print_mem_error(memh->name, "end corrupt");
            mem_lock_thread();
            name = check_memlist(memh);
            mem_unlock_thread();
            if (name != NULL) {
                if (name != memh->name) {
                    print_mem_error(name, "name is also corrupt");
                }
            }
        }
    } else {
        mem_lock_thread();
        name = check_memlist(memh);
        mem_unlock_thread();
        if (name == NULL) {
            print_mem_error("free", "pointer not in memlist");
        } else {
            print_mem_error(name, "header corrupt");
        }
    }
    
    totblock--;
}

int MEM_secured_get_mem_in_use()
{
    size_t _mem_in_use;
    
    mem_lock_thread();
    _mem_in_use = mem_in_use;
    mem_unlock_thread();
    
    return _mem_in_use;
}

int MEM_secured_get_totblock()
{
    unsigned int _totblock;
    
    mem_lock_thread();
    _totblock = totblock;
    mem_unlock_thread();
    
    return totblock;
}

bool MEM_secured_check_memory()
{
    return (check_memlist(NULL) == NULL);
}

typedef struct MemInfo {
    const char *name;
    unsigned long long len;
    unsigned long items;
}MemInfo;

int compare_name(const void *ptr1, const void *ptr2)
{
    return strcmp(((MemInfo *)ptr1)->name, ((MemInfo *)ptr2)->name);
}

int compare_len(const void *ptr1, const void *ptr2)
{
    MemInfo *memstat1, *memstat2;
    memstat1 = (MemInfo *)ptr1;
    memstat2 = (MemInfo *)ptr2;
    
    if (memstat1->len < memstat2->len) {
        return 1;
    } else if (memstat1->len == memstat2->len) {
        return 0;
    } else {
        return -1;
    }
}

void MEM_secured_print_memstats(void)
{
    MemInfo *stats, *element;
    unsigned int i, j, totelement = 0;
    MemHead *block;
    
    mem_lock_thread();
    
    /* construct stats buffer */
    stats = (MemInfo *)malloc(totblock*sizeof(MemInfo));
    element = stats;
    
    if (stats == NULL) {
        mem_unlock_thread();
        return;
    }
    
    block = membase->first;
    if (block) {
        block = (MemHead *)((char *)block - offsetof(MemHead, next));
    }
    
    while (block && element) {
        element->name = block->name;
        element->len = block->len;
        element->items = 1;
        
        totelement++;
        element++;
        if (block->next) {
            block = (MemHead *)((char *)block->next - offsetof(MemHead, next));
        } else {
            break;
        }
    }
    
    if (totelement > 1) {
        qsort(stats, totelement, sizeof(MemInfo), compare_name);
    }
    
    /* sort by name */
    for (i=0, j=0; i<totblock; i++) {
        if (i == j) {
            continue;
        }
        if (strcmp(stats[i].name, stats[j].name) == 0) {
            stats[j].len += stats[i].len;
            stats[j].items++;
        } else {
            j++;
            memcpy(&stats[j], &stats[i], sizeof(MemInfo));
        }
    }
    totelement = j + 1;
    
    if (totelement > 1) {
        qsort(stats, totelement, sizeof(MemInfo), compare_len);
    }
    
    printf("Total memory allocated : %.3f Mio\n", (double) mem_in_use / (double) (1024*1024));
    printf("Number of allocated block : %d\n", totblock);
    printf("  ITEMS TOTAL-MiB AVERAGE-KiB NAME :\n");
    
    element = stats;
    for (i = 0; i<totelement; i++) {
        printf("%7lu (%8.3f %10.3f) %s\n",
               element->items,
               (double) element->len / (double) (1024*1024),
               (double) element->len / (double) (element->items*1024),
               element->name);
        element++;
    }
    mem_unlock_thread();
    
    free(stats);
}

/* ---------------------------- */
/*  local functions             */
/* ---------------------------- */

static void make_mem_header(MemHead *memh,
                            size_t len,
                            const char *str)
{
    memh->tag1 = MEM_TAG1;
    memh->len = len;
    memh->name = str;
    memh->nextname = NULL;
    memh->tag2 = MEM_TAG2;
    memh->alignment = 0;
    
    MemTail *memt = (MemTail *)(((char *)memh) + sizeof(MemHead) + len);
    memt->tag3 = MEM_TAG3;
    
    mem_lock_thread();
    totblock ++;
    mem_in_use += len;
    add_memblock(memh);
    mem_unlock_thread();
}

static void add_memblock(MemHead *memh) {
    localList *link = (localList *)&memh->next;
    
    link->next = NULL;
    link->prev = membase->last;
    
    if (membase->last) {
        MemHead *prevh = (MemHead *)((char *)membase->last - offsetof(MemHead, next));
        prevh->nextname = memh->name;
        ((localList *)membase->last)->next = link;
    }
    if (membase->first == NULL) {
        membase->first = link;
    }
    membase->last = link;
}

static void rm_memblock(MemHead *memh) {
    mem_lock_thread();
    rm_memblock_from_memlist(memh);
    
    totblock--;
    mem_in_use -= memh->len;
    mem_unlock_thread();
    
    if (memh->alignment == 0) {
        free(memh);
    } else {
        free((char*)memh - (memh->alignment - (sizeof(MemHead) % memh->alignment)));
    }
}

static void rm_memblock_from_memlist(MemHead *memh)
{
    localList *link = (localList *)&memh->next;
    
    if (link->prev) {
        (link->prev)->next = link->next;
    }
    
    if (link->next) {
        (link->next)->prev = link->prev;
    }
    
    if (link == membase->first) {
        membase->first = link->next;
    }
    
    if (link == membase->last) {
        membase->last = link->prev;
    }
}

static const char *check_memlist(MemHead *memh) {
    MemHead *forw, *backw, *forwfine, *backwfine;
    const char *name;
    forw = membase->first;
    backw = membase->last;
    
    /* search for errors in the memroy base */
    if (forw) {
        forw = (MemHead *)((char *)forw - offsetof(MemHead, next));
    }
    forwfine = NULL;
    while (forw) {
        if (forw->tag1 != MEM_TAG1 || forw->tag2 != MEM_TAG2) {
            break;
        }
        forwfine = forw;
        if (forw->next) {
            forw = (MemHead *)((char *)forw->next - offsetof(MemHead, next));
        } else {
            forw = NULL;
        }
    }
    
    if (backw) {
        backw = (MemHead *)((char *)backw - offsetof(MemHead, next));
    }
    backwfine = NULL;
    while (backw) {
        if (backw->tag1 != MEM_TAG1 || backw->tag2 != MEM_TAG2) {
            break;
        }
        backwfine = backw;
        if (backw->prev) {
            backw = (MemHead *)((char *)backw->prev - offsetof(MemHead, next));
        } else {
            backw = NULL;
        }
    }
    
    if (backw != forw) {
        return ("more than one memory block corrupt");
    }
    
    /* if there is no corrupt header, then find the block in the list */
    if (backw == NULL && forw == NULL) {
        forw = membase->first;
        backw = membase->last;
        
        if (forw) {
            forw = (MemHead *)((char *)forw - offsetof(MemHead, next));
        }
        forwfine = NULL;
        while (forw) {
            if (forw == memh) {
                break;
            }
            forwfine = forw;
            if (forw->next) {
                forw = (MemHead *)((char *)forw->next - offsetof(MemHead, next));
            } else {
                forw = NULL;
            }
        }
        
        if (backw) {
            backw = (MemHead *)((char *)backw - offsetof(MemHead, next));
        }
        backwfine = NULL;
        while (backw) {
            if (backw == memh) {
                break;
            }
            backwfine = backw;
            if (backw->prev) {
                backw = (MemHead *)((char *)backw->prev - offsetof(MemHead, next));
            } else {
                backw = NULL;
            }
        }
    }
    
    if (forwfine) {
        name = forwfine->nextname;
    } else {
        name = "no name found";
    }
    
    /* remove correctly the block from the list and then free it */
    if (forw == memh) {
        if (forwfine) {
            if (backwfine) {
                forwfine->next = (void *)&backwfine->next;
                backwfine->prev = (void *)&forwfine->next;
                forwfine->nextname = backwfine->name;
            } else {
                forwfine->next = NULL;
                membase->last = (void *)&forwfine->next;
            }
        } else {
            if (backw) {
                backwfine->prev = NULL;
                membase->first = (void *)&backwfine->next;
            } else {
                membase->first = membase->last = NULL;
            }
        }
    }
    
    return name;
}
