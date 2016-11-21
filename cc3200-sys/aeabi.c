/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
 * Implements the ARM Run-time ABI, see
 *
 *   http://infocenter.arm.com/help/topic/com.arm.doc.ihi0043d/IHI0043D_rtabi.pdf
 */

#include <string.h>

void __aeabi_memclr4(void* dest, size_t n)
{
    memset(dest, 0, n);
}
