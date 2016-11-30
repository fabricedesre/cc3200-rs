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

typedef int di_int;
#define CHAR_BIT 8

// Copied from From https://github.com/llvm-mirror/compiler-rt/blob/d89810afdc2862ea865f5c297ef4ea5839941d25/lib/builtins/mulodi4.c

di_int
__mulodi4(di_int a, di_int b, int* overflow)
{
    const int N = (int)(sizeof(di_int) * CHAR_BIT);
    const di_int MIN = (di_int)1 << (N-1);
    const di_int MAX = ~MIN;
    *overflow = 0;
    di_int result = a * b;
    if (a == MIN)
    {
        if (b != 0 && b != 1)
	    *overflow = 1;
	return result;
    }
    if (b == MIN)
    {
        if (a != 0 && a != 1)
	    *overflow = 1;
        return result;
    }
    di_int sa = a >> (N - 1);
    di_int abs_a = (a ^ sa) - sa;
    di_int sb = b >> (N - 1);
    di_int abs_b = (b ^ sb) - sb;
    if (abs_a < 2 || abs_b < 2)
        return result;
    if (sa == sb)
    {
        if (abs_a > MAX / abs_b)
            *overflow = 1;
    }
    else
    {
        if (abs_a > MIN / -abs_b)
            *overflow = 1;
    }
    return result;
}

