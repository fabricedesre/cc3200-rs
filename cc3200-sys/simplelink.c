// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#include "board.h"
#include "fs.h"

// These functions remain in C because they use C macros. We might possibly
// be able to move these to rust by adding a pre-processing step.

const char *simplelink_get_driver_version(uint32_t *len) {
    *len = sizeof(SL_DRIVER_VERSION) - 1; // sizeof includes the terminating nul
    return SL_DRIVER_VERSION;
}

_u32 sl_FsMode(bool write, bool create, bool failsafe, _u32 maxCreateSize)
{
    if (create) {
        _u32 flags = failsafe ? _FS_FILE_OPEN_FLAG_COMMIT : 0;
        return FS_MODE_OPEN_CREATE(maxCreateSize, flags);
    } else if (write) {
        return FS_MODE_OPEN_WRITE;
    }
    return FS_MODE_OPEN_READ;
}
