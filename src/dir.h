#pragma once

#include "kernal.h"

typedef struct dirent
{
    char d_name[16+1];
} dirent;

void opendir(fd_t fd, uint8_t dev);
bool readdir(dirent *dirent);
void closedir(fd_t fd);
