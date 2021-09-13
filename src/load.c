#include "kernal.h"

uint8_t load(uint8_t dev, const char* fname, void** dest)
{
    k_setnam(fname);
    k_setlfs(1, dev, 0);
    return k_load(0, dest);
}
