#include <string.h>
#include <stdint.h>
#include "dir.h"
#include "kernal.h"

volatile uint8_t* border = (uint8_t*)(0xd020);

bool getbyte(uint8_t *chr)
{
    ++(*border);
    if (k_readst() != 0) return false;

    *chr = k_chrin();
    return true;
}

void opendir(fd_t fd, uint8_t dev)
{
    k_setnam("$");
    k_setlfs(fd, dev, 0);
    k_open();
    k_chkin(fd);

    uint8_t chr;
    // Skip target address
    getbyte(&chr);
    getbyte(&chr);
}

bool readdir(dirent *dirent)
{
    uint8_t chr;

    // Skip 2 bytes
    if (!getbyte(&chr)) return false;
    if (!getbyte(&chr)) return false;

    // BASIC line number
    if (!getbyte(&chr)) return false;
    if (!getbyte(&chr)) return false;

    // skip to opening quote
    while (chr != '"')
        if (!getbyte(&chr)) return false;

    // scan to closing quote
    char* name = dirent->d_name;

    if (!getbyte(&chr)) return false;
    while (chr != '"')
    {
        *(name++) = chr;
        if (!getbyte(&chr)) return false;
    }

    *name = '\0';

    // skip everything after the closing quote
    while (chr)
        if (!getbyte(&chr)) return false;

    return true;
}

void closedir(fd_t fd)
{
    k_close(fd);
    k_clrchn();
}
