#include <stdint.h>

#include "interrupt.h"

uint8_t timer_reg;
extern void select_and_load_file(uint8_t* mem);
extern void make_charset();
extern void clear_screen(uint8_t* scr);
extern void run(uint8_t* mem, uint8_t* scr);

void irq();
extern void print_dir();

int main ()
{
    uint8_t mem[4 * 1024 - 0x200];
    select_and_load_file(mem);

    make_charset();
    uint8_t* const scr = (uint8_t*)0xc400;
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

    /* Start char font at 0xc000 */
    *vicPtr = (*vicPtr & 0xf0) | 0x0 | (*vicPtr & 0x01);

    set_frame_irq(&irq);
    run(mem, scr);

    return 0;
}

__attribute__((no_isr))
void irq() {
    POKE(0xd019, 0xff);
    /* POKE(0xd021, PEEK(0xd021) + 1); */

    if (timer_reg > 0) --timer_reg;

    __asm__("jmp 0xea31");
}
