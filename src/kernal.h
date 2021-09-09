#pragma once

#include <stdint.h>
#include <stdbool.h>

typedef uint8_t fd_t;

void    k_setnam (const char *fname);
void    k_setlfs (fd_t fd, uint8_t dev, uint8_t secondary);
bool    k_open   ();
void    k_chkin  (fd_t fd);
uint8_t k_readst ();
uint8_t k_chrin  ();
void    k_close  (fd_t fd);
void    k_clrchn ();
void    k_ldplot (uint8_t col, uint8_t row);
