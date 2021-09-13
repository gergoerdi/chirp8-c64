#include <stdint.h>
#include <stdio.h>
#include <chrout.h>

#include "file_selector.h"
#include "load.h"
#include "c64.h"

const uint8_t window_size = 8;

void selectAndLoadFile(void* dest)
{
    uint8_t num_dirents = 0;
    dirent dirents[144];

    uint8_t border_before = *border;
    opendir(2,8);
    dirent* dirent = dirents;

    while (readdir(dirent))
    {
        dirent++;
        num_dirents++;
    }
    closedir(2);
    *border = border_before;

    initSelectorScreen();
    uint8_t selection = selectFile(dirents, num_dirents);
    void* dest_end = dest;
    load(8, dirents[selection].d_name, &dest_end);
}

void initSelectorScreen()
{
    char* scr = (char*)0x0400;
    for (int i = 0; i < 1000; ++i)
        *(scr++) = ' ';

    k_ldplot(10, 11);
    __chrout(0x75);
    for (int i = 0; i < 16; ++i)
        __chrout(0x60);
    __chrout(0xae);

    k_ldplot(10 + window_size + 1, 11);
    __chrout(0xad);
    for (int i = 0; i < 16; ++i)
        __chrout(0x60); // 64 looks quite nice here
    __chrout(0x6b);

    for (int i = 1; i < window_size + 1; ++i)
    {
        k_ldplot(10 + i, 11);
        __chrout(0x62);
        k_ldplot(10 + i, 12 + 16);
        __chrout(0x62);
    }
}

uint8_t selectFile(dirent* dirents, uint8_t num_dirents)
{
    uint8_t sel = 1;
    uint8_t offset = 0;

    bool wait_release = false;

    dirent *dirent = dirents;
    while (true)
    {
        dirent = &(dirents[offset + 1]);
        for (uint8_t i = 1; i <= window_size && offset + i < num_dirents; ++i, ++dirent)
        {
            bool current = offset + i == sel;
            k_ldplot(10 + i, 12);
            if (current)
                __chrout(0x12); // reverse on
            const char* ptr = dirent->d_name;
            for (int i = 0; i < 16; ++i)
            {
                if (*ptr)
                    __chrout(*(ptr++));
                else
                    __chrout(' ');
            }
            if (current)
                __chrout(0x92); // reverse off
        }

        volatile uint8_t* keyptr = (uint8_t*)(0x00c5);

        if (wait_release)
        {
            while(true)
            {
                if (*keyptr == 0x40) break;
            }
            wait_release = false;
        }

        while (true)
        {
            switch (*keyptr)
            {
            case 0x0c: // Z
            case 0x17: // X
            case 0x14: // C
            case 0x1f: // V
            {
                if (sel == num_dirents - 1) continue;
                wait_release = true;

                ++sel;
                if (sel - offset > window_size) ++offset;
                goto redraw;
            }

            case 0x3e: // Q
            case 0x09: // W
            case 0x0e: // E
            case 0x11: // R
            {
                if (sel == 1) continue;
                wait_release = true;

                --sel;
                if (sel <= offset) --offset;
                goto redraw;
            }

            case 0x01: // Enter
                return sel;
            }
        }
    redraw:
        ;
    }
}
