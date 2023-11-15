/*
 * secured_tests.c
 * This file is part of PixSea
 *
 * Copyright (C) 2022 Mera Emmanuel <emmanuel.mera@live.fr>.
 *
 * MIT License (MIT), http://opensource.org/licenses/MIT
 * Full license can be found in the LICENSE file
 */

#include "secured_tests.h"
#include "../MEM_alloc.h"

#define TEST_ALLOC_SIZE sizeof(int)
#define TEST_ALLOC_LEN 4096

bool run_secured_tests()
{
    MEM_use_secured_allocator();
    
    /* make some "standards" allocations */
    void *test_malloc = MEM_malloc(TEST_ALLOC_LEN, "standards");
    if (test_malloc) {
        return MEM_check_memory();
    }
    
    void *test_calloc = MEM_calloc(TEST_ALLOC_LEN, "standards");
    if (test_calloc) {
        return MEM_check_memory();
    }
    
    void *test_malloc_array = MEM_malloc_array(TEST_ALLOC_LEN, TEST_ALLOC_SIZE, "standards");
    if (test_malloc_array) {
        return MEM_check_memory();
    }
    
    void *test_calloc_array = MEM_calloc_array(TEST_ALLOC_LEN, TEST_ALLOC_SIZE, "standards");
    if (test_calloc_array) {
        return MEM_check_memory();
    }
    
    if (MEM_check_memory) {
        return EXIT_FAILURE; // maybe change this to proper error codes
    }
    
    return true;
}

int main()
{
    return run_secured_tests();
}
