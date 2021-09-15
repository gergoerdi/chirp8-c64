#include <stdint.h>
#include <stdbool.h>

#include "interrupt.h"

#define SEI() __asm__("sei")
#define CLI() __asm__("cli")

void set_frame_irq(void (*fun)())
{
    /* https://www.c64-wiki.com/wiki/Raster_interrupt */
    SEI();
    POKE(0xdc0d, 0b01111111);

    PEEK(0xdc0d);
    PEEK(0xdd0d);

    POKE(0xd012, 210); // select raster line 210
    POKE(0xd011, PEEK(0xd011) & 0x7f);

    uint16_t ptr = (uint16_t)(fun);
    POKE(0x0314, (uint8_t)(ptr >> 0));
    POKE(0x0315, (uint8_t)(ptr >> 8));

    POKE(0xd01a, 0x01); // enable raster interrupt signal
    CLI();
}
