#include <stdint.h>
#include <stdio.h>
#include <chrout.h>

#include "interrupt.h"
#include "dir.h"

uint8_t timer_reg;
extern void clear_screen(uint8_t* scr);
extern void run(uint8_t* scr);

void selectFile(dirent* dirents, uint8_t num_dirents);
void initSelector();

const uint8_t window_size = 8;

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

void dir()
{
    uint8_t num_dirents = 0;
    dirent dirents[144];

    opendir(2,8);
    dirent* dirent = dirents;

    while (readdir(dirent))
    {
        dirent++;
        num_dirents++;
    }
    closedir(2);

    initSelectorScreen();
    selectFile(dirents, num_dirents);
}

void selectFile(dirent* dirents, uint8_t num_dirents)
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
            volatile uint8_t key = *keyptr;

            if (key == 0x0c || key == 0x17 || key == 0x14 || key == 0x1f) // down
            {
                if (sel == num_dirents - 1) continue;
                wait_release = true;

                ++sel;
                if (sel - offset > window_size) ++offset;
                goto redraw;
            } else if(key == 0x3e || key == 0x09 || key == 0x0e || key == 0x11) // up
            {
                if (sel == 1) continue;
                wait_release = true;

                --sel;
                if (sel <= offset) --offset;
                goto redraw;
            }
        }
    redraw:
        ;
    }
}

int main ()
{
    dir();
    return 0;

    opendir(2,8);
    dirent dirent;
    while (readdir(&dirent))
    {
        for (char* ptr = dirent.d_name; *ptr; ++ptr)
            __chrout(*ptr);
        __chrout('\r');
    }
    closedir(2);
    return 0;

    uint8_t* const scr = (uint8_t*)0xc400;
    uint8_t* const font = (uint8_t*)0xc000;

    /* Create charset */
    for (uint8_t i = 0; i < 4; ++i)
    {
        for (uint8_t j = 0; j <= 0xf; ++j)
        {
            *(font + (j << 3) + i) =
                (j & 0x1 ? 0x0f : 0x00) |
                (j & 0x2 ? 0xf0 : 0x00);
        }
    }

    for (uint8_t i = 4; i < 8; ++i)
    {
        for (uint8_t j = 0; j <= 0xf; ++j)
        {
            *(font + (j << 3) + i) =
                (j & 0x4 ? 0x0f : 0x00) |
                (j & 0x8 ? 0xf0 : 0x00);
        }
    }

    clear_screen(scr);

    /* Use 0xc000..0xffff for VIC graphics */
    uint8_t* const cia2aDDR = (uint8_t*)0xdd02;
    uint8_t* const cia2aDR = (uint8_t*)0xdd00;
    uint8_t* const krnlScr = (uint8_t*)0x0288;
    uint8_t* const vicPtr = (uint8_t*)0xd018;
    uint8_t* const borderColor = (uint8_t*)0xd020;
    uint8_t* const bgColor = (uint8_t*)0xd021;

    *cia2aDDR |= 0x3;
    *cia2aDR = *cia2aDR & 0xfc | 0x00;
    *krnlScr = 0xc0;

    /* Color scheme */
    *borderColor = 0x0b;
    *bgColor = 0x00;

    /* Start char font at 0x8000 */
    *vicPtr = (*vicPtr & 0xf0) | 0x0 | (*vicPtr & 0x01);

    set_frame_irq(&irq);
    run(scr);

    return 0;
}

__attribute__((no_isr))
void irq() {
    POKE(0xd019, 0xff);
    /* POKE(0xd021, PEEK(0xd021) + 1); */

    if (timer_reg > 0) --timer_reg;

    __asm__("jmp 0xea31");
}
