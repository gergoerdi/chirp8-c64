#include "kernal.h"

#include <string.h>

void k_setnam(const char *fname)
{
    uint16_t ptr = (uint16_t)(fname);

    __attribute__((leaf)) asm volatile(
        "jsr $ffbd"
        :
        : "a" ((uint8_t)(strlen(fname)))
        , "x" ((uint8_t)(ptr >> 0))
        , "y" (((uint8_t)(ptr >> 8)))
        :
        );
}

void k_setlfs(uint8_t fd, uint8_t dev, uint8_t secondary)
{
    __attribute__((leaf)) asm volatile(
        "jsr $ffba"
        :
        : "a" (fd), "x" (dev), "y" (secondary)
        : "p"
        );
}

bool k_open()
{
    // TODO: error checking: if Carry flag is set, then error
    __attribute__((leaf)) asm volatile (
        "jsr $ffc0"
        :
        :
        : "p"
        );
    return true;
}

void k_chkin(fd_t fd)
{
    __attribute__((leaf)) asm volatile (
        "jsr $ffc6"
        : "+x" (fd)
        :
        : "a"
        );
}

uint8_t k_readst()
{
    uint8_t st;
    __attribute__((leaf)) asm volatile(
        "jsr $ffb7"
        : "=a" (st)
        :
        : "p"
        );
    return st;
}

uint8_t k_chrin()
{
    uint8_t chr;
    __attribute__((leaf)) asm volatile(
        "jsr $ffcf"
        : "=a" (chr)
        :
        : "p"
        );
    return chr;
}

void k_close(fd_t fd)
{
    __attribute__((leaf)) asm volatile(
        "jsr $ffc3"
        :
        : "a" (fd)
        : "x", "y"
        );
}

void k_clrchn()
{
    __attribute__((leaf)) asm volatile(
        "jsr $ffcc"
        :
        :
        : "a", "x"
        );
}

void k_ldplot (uint8_t col, uint8_t row)
{
    __attribute__((leaf)) asm volatile(
        "clc\n"
        "jsr $fff0"
        :
        : "x" (col), "y"(row)
        : "a", "p"
        );
}
