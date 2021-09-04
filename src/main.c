#include <stdint.h>
#include <stdio.h>

extern void clrText(uint8_t* scr);
extern void drawScreen(uint8_t* scr);

int main ()
{
    uint8_t* const vicPtr = (uint8_t*)0xd018;
    uint8_t* const scr = (uint8_t*)0xc400;
    uint8_t* const krnlScr = (uint8_t*)0x0288;
    uint8_t* const font = (uint8_t*)0xc000;

    uint8_t* const cia2aDDR = (uint8_t*)0xdd02;
    uint8_t* const cia2aDR = (uint8_t*)0xdd00;

    /* Use 0xc000..0xffff for VIC graphics */
    *cia2aDDR |= 0x3;
    *cia2aDR = *cia2aDR & 0xfc | 0x00;
    *krnlScr = 0xc0;

    /* Start char font at 0x8000 */
    *vicPtr = (*vicPtr & 0xf0) | 0x0 | (*vicPtr & 0x01);

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

    clrText(scr);
    drawScreen(scr);
    while (1) {}

    return 0;
}
