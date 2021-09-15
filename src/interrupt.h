#pragma once

void set_frame_irq(void (*fun)());

#define POKE(addr, val) \
    (*(volatile uint8_t*)(addr) = val)

#define PEEK(addr) \
    (*(volatile uint8_t*)(addr))
