/*
 * MEM_alloc.h
 * This file is part of PixSea
 *
 * Copyright (C) 2022 Mera Emmanuel <emmanuel.mera@live.fr>.
 *
 * MIT License (MIT), http://opensource.org/licenses/MIT
 * Full license can be found in the LICENSE file
 */

/** \file
 * \ingroup MEM
 *
 * \brief Read \ref MEMPage
 *
 * \page MEMPage Secured allocation
 *
 * \section aboutmem Memory allocation module
 *
 * \subsection memabout About the MEM module
 *
 * MEM provides secured malloc/calloc calls. All memory is enclosed by
 * pads, to detect out-of-bound writes. All blocks are placed in a
 * linked list, so they remain reachable at all times. There is no
 * back-up in case the linked-list related data is lost.
 *
 * \subsection memdependencies Dependencies
 * - stdlib
 * - stdio
 * - pthread
 * - stdbool
 */

#ifndef MEM_alloc_h
#define MEM_alloc_h

#include <stdlib.h>
#include <stdbool.h>

/**
 * Allocate a block of memory.
 *
 * \param len length of the block to allocate.
 * \param str the name of the allocated block. It MUST be static because only a pointer is saved.
 */
extern void *(*MEM_malloc)(size_t len, const char *str) /* ATTR_MALLOC */ __attribute__ ((warn_unused_result));

/**
 * Allocate a block of memory of size (len*size). The name MUST be static because
 * only a pointer is saved.
 */
extern void *(*MEM_malloc_array)(size_t len, size_t size, const char *str);

/**
 * Allocate an aligned block of memory of size len, with
 * an alignment of alignment. The name MUST be static
 * because only a pointer is saved.
 */
extern void *(*MEM_malloc_aligned)(size_t len, size_t alignment, const char *str);

/**
 * Allocate a block of memory of size len, with name str.
 * The memory is clear. The name MUST be static because
 * only a pointer is saved.
 */
extern void *(*MEM_calloc)(size_t len, const char *str);

/**
 * Allocate a block of memory of size (len*size), with name str.
 * The memory is clear. The name MUST be static because
 * only a pointer is saved.
 */
extern void *(*MEM_calloc_array)(size_t len, size_t size, const char *str);

/**
 * Duplicate the given memory block and set it with the
 * name str. The name MUST be static because
 * only a pointer is saved.
 */
extern void *(*MEM_dupalloc)(void *vmem, const char *str);

/**
 * Reallocate a block of memory with size len
 * and copy its content.
 */
extern void *(*MEM_realloc)(void *vmem, size_t len);

/**
 * Reallocate a block of memory with size len,
 * name str and copy its content. The name MUST
 * be static because only a pointer is saved.
 */
extern void *(*MEM_realloc_id)(void *vmem, size_t len, const char *str);

/**
 * Free a block of memory allocated by this
 * module.
 */
extern void (*MEM_free)(void *vmem);

/**
 * Print to stdout memory stats
 */
extern void (*MEM_print_memstats)(void);

/**
 * If there is no issue, return 0, else return a non-zero value.
 */
extern bool (*MEM_check_memory)(void);

/**
 * Return total memory in use (in octet)
 */
extern int (*MEM_get_mem_in_use)(void);

/**
 * Return total allocated block
 */
extern int (*MEM_get_totblock)(void);

/**
 * Switch to secured allocation impl
 */
extern void MEM_use_secured_allocator(void);

extern void MEM_use_light_allocator(void);

#endif /* MEM_alloc_h */
