// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

#include <stddef.h>
#include <string.h>
#include "FreeRTOS.h"
#include "portable.h"

// Provide a realloc function since FreeRTOS doesn't
void *realloc_helper(void *old_ptr, size_t old_size, size_t new_size) {
    void *new_ptr = NULL;
    if (new_size) {
        void *new_ptr = pvPortMalloc(new_size);
        if (new_ptr) {
            size_t copy_size = new_size;
            if (new_size > old_size) {
                copy_size = old_size;
            }
            memcpy(new_ptr, old_ptr, copy_size);
            vPortFree(old_ptr);
        }
    }
    return new_ptr;
}
