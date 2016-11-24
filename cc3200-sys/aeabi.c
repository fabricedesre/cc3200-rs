/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
 * Implements the ARM Run-time ABI, see
 *
 *   http://infocenter.arm.com/help/topic/com.arm.doc.ihi0043d/IHI0043D_rtabi.pdf
 */

#include <string.h>

// Version 4.9.3 comes with __aeabi_memclr4, and if we compile this one in then
// we get duplicate symbols errors. So only include it for version 5 and above.

#if __GNUC__ >= 5

void __aeabi_memclr4(void* dest, size_t n)
{
    memset(dest, 0, n);
}

#endif
