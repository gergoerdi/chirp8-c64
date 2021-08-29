#include <stdint.h>
#include <stdio.h>

uint8_t rows[][8] = {
    { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 },
    { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 },
    { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 },
    { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 },
    { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 },
    { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 },
    { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 },
    { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 },
    { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 },
    { 0x00, 0x08, 0xbe, 0xf3, 0xcf, 0xa2, 0x10, 0x00 },
    { 0x00, 0x08, 0x88, 0x49, 0x28, 0x32, 0x10, 0x00 },
    { 0x00, 0x0f, 0x88, 0x49, 0x2e, 0x2a, 0x10, 0x00 },
    { 0x00, 0x08, 0x88, 0x49, 0x28, 0x26, 0x00, 0x00 },
    { 0x00, 0x08, 0xbe, 0xf3, 0xcf, 0xa2, 0x10, 0x00 },
    { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 },
    { 0x00, 0x00, 0x00, 0x8e, 0x22, 0x80, 0x00, 0x00 },
    { 0x00, 0x00, 0x00, 0x8a, 0x3b, 0x80, 0x00, 0x00 },
    { 0x00, 0x00, 0x00, 0xae, 0x39, 0x00, 0x00, 0x00 },
    { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 },
    { 0x00, 0x0f, 0x08, 0xa8, 0xbe, 0xfb, 0xc0, 0x00 },
    { 0x00, 0x04, 0x88, 0xac, 0x88, 0x82, 0x20, 0x00 },
    { 0x00, 0x04, 0x8a, 0xaa, 0x88, 0xe3, 0xc0, 0x00 },
    { 0x00, 0x04, 0x8a, 0xa9, 0x88, 0x82, 0x80, 0x00 },
    { 0x00, 0x0f, 0x25, 0x28, 0x88, 0xfa, 0x40, 0x00 },
    { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 },
    { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 },
    { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 },
    { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 },
    { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 },
    { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 },
    { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 },
    { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 },
    { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 }
};

int main ()
{
    uint8_t* vicPtr = (uint8_t*)0xd018;
    uint8_t* scr = (uint8_t*)0x8400;
    uint8_t* krnlScr = (uint8_t*)0x0288;
    uint8_t* font = (uint8_t*)0x8000;

    uint8_t i, j, k;

    uint8_t* cia2aDDR = (uint8_t*)0xdd02;
    uint8_t* cia2aDR = (uint8_t*)0xdd00;

    uint8_t *wp, *wpBase;

    /* Use 0x4000..0x7FFF for VIC graphics */
    *cia2aDDR |= 0x3;
    *cia2aDR = *cia2aDR & 0xfc | 0x1;
    *krnlScr = 0x80;

    /* Start char font at 0x8000 */
    *vicPtr = (*vicPtr & 0xf0) | 0x0 | (*vicPtr & 0x01);

    /* Create charset */
    for (i = 0; i < 4; ++i)
    {
        for (j = 0; j < 0xf; ++j)
        {
            *(font + (j << 3) + i) =
                (j & 0x1 ? 0x0f : 0x00) |
                (j & 0x2 ? 0xf0 : 0x00);
        }
    }

    for (i = 4; i < 8; ++i)
    {
        for (j = 0; j < 0xf; ++j)
        {
            *(font + (j << 3) + i) =
                (j & 0x4 ? 0x0f : 0x00) |
                (j & 0x8 ? 0xf0 : 0x00);
        }
    }

    wpBase = scr; // + 6 * 40;
#if 1
    for (i = 0, scr = (uint8_t*)0x8400 + 6 * 40; i < 32; i += 2)
    {
        wp = wpBase;
        for (j = 0; j < 8; ++j)
        {
            uint8_t b1 = rows[i][j];
            uint8_t b2 = rows[i + 1][j];

            for (k = 0; k < 4; ++k)
            {
                uint8_t ch = ((b1 & 0xc0) >> 6) | ((b2 & 0xc0) >> 4);
                b1 <<= 2;
                b2 <<= 2;
                *(wp++) = ch;
            }
        }
        wpBase = wpBase + 40;
    }
#else
    for (i = 0, scr = (uint8_t*)0x8400 + 6 * 40; i < 32; i += 1)
    {
        wp = wpBase;
        for (j = 0; j < 8; ++j)
        {
            uint8_t b1 = rows[i][j];

            for (k = 0; k < 4; ++k)
            {
                uint8_t ch = ((b1 & 0xc0) >> 4);
                b1 <<= 2;
                *(wp++) = ch;
            }
        }
        wpBase = wpBase + 40;
    }
#endif
    while (1) {}

    return 0;
}
