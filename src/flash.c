#include "c64.h"
#include "interrupt.h"

__attribute__((no_isr)) void flasher()
{
    (*border)++;
    asm("jmp 0xf6ed");
}

void start_flash()
{
    asm("sei");

    uint16_t ptr = (uint16_t)(flasher);
    POKE(0x0328, (uint8_t)(ptr >> 0));
    POKE(0x0329, (uint8_t)(ptr >> 8));

    asm("cli");
}

void end_flash()
{
    // TODO
}
