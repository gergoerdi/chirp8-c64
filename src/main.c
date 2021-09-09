#include <stdint.h>
#include <stdio.h>
#include <chrout.h>

#include "interrupt.h"
#include "dir.h"

uint8_t timer_reg;
extern void clear_screen(uint8_t* scr);
extern void run(uint8_t* scr);

void print_label(const char* name)
{
    k_ldplot(0, 12);
    __chrout(0x12); // reverse on
    for (const char* i = name; *i; ++i)
        __chrout(*i);
    __chrout(0x92); // reverse off
}

void print_file(const char* name)
{
    k_ldplot(1, 12);
    for (const char* i = name; *i; ++i)
        __chrout(*i);
}

void dir()
{
    char* scr = (char*)0x0400;
    for (int i = 0; i < 1000; ++i)
        *(scr++) = ' ';

    uint8_t num_dirents = 0;
    dirent dirents[144];

    opendir(2,8);
    dirent* dirent = dirents;

    while (readdir(dirent))
    {
        /* if (disklabel) */
        /*     print_label(dirent.d_name); */
        /* else */
        /*     print_file(dirent.d_name); */
        /* disklabel = false; */
        dirent++;
        num_dirents++;
    }
    closedir(2);

    dirent = dirents;
    for (uint8_t i = 0; i < num_dirents; ++i, ++dirent)
    {
        if (i == 0)
            print_label(dirents[i].d_name);
        else
            print_file(dirents[i].d_name);
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
