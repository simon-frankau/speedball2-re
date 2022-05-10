# Notes on reverse engineering the Speedball 2 Megadrive ROM

## File sections

Rough addresses (to be refined):

 * `0x000000` - 68k reset, interrupt, etc. vectors
 * `0x000100` - Sega ROM id
 * `0x000200`? - Entry point, start of code.
 * `0x02260a` - `splash_start1` - Arena & Bitmap Brothers
 * `0x025a5c` - `splash_start2` - ImageWorks and Bitmap Brothers
 * `0x02914e`? - ???
 * `0x02d580`? - Font, followed by something unobvious
 * `0x02e6ca`? - Strange graphics/data
 * `0x02fcaa`? - Goal score picture or something? Palette 0
 * `0x032c4a`? - Another font, followed by graphics-looking stuff,
                palette 0, sprite fragments down to...
 * `0x0454ca` - `splash_backdrop` - The backdrop for results tables etc.
 * `0x049bfc` - `splash_victory` - Victory image
 * `0x04e66e` - `splash_defeat` - Match loss
 * `0x051fe0` - `splash_win_league`
 * `0x053e1a` - `splash_win_promo`
 * `0x055c34` - `splash_win_cup`
 * `0x057a52` - `splash_win_knockout`
 * `0x059838` - `splash_title` - Title screen
 * `0x05d8c6`? - 4 bytes of ?
 * `0x05d8ca` - `splash_arena` - Arena background for intro text.
 * `0x0610c4`? - Big font, then various bits associated with management
                pages, Palette 3.
 * `0x07e006` - End of data

Entries with question marks haven't been fully explored.

All have a palette at offset 2, except `splash_backdrop`, which is
special-cased with 2 palettes, taken from 0x029f5e.

## Palettes

`find_palettes` suggests non-splash palettes located at:

 * 0x0007c4 - Palette 0
 * 0x029ede - Palette 3
 * 0x029efe - Palette 4

Colour scheme #3/#4 looks good for stat management screen buttons, faces.

## VRAM memory map

`display_splash` puts tile data at 0X7d00, tile map (1000 bytes) at
0xe010.
