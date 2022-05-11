# Notes on reverse engineering the Speedball 2 Megadrive ROM

## File sections

Rough addresses (to be refined):

 * `0x000000` - 68k reset, interrupt, etc. vectors
 * `0x000100` - Sega ROM id
 * `0x000200`? - Entry point, start of code.
 * `0x02260a` - `splash_start1` - Arena & Bitmap Brothers
 * `0x025a5c` - `splash_start2` - ImageWorks and Bitmap Brothers

 * `0x02914e`? - ??? - 2, 0x02e6ca,  1, 1, false,  "undecoded"),
 * `0x02d580`? - 1x1 font, followed by something unobvious
 * `0x02e6ca` - Title screen font, 2x2.
 * `0x02fcca`? - 64 bytes of unknown
 * `0x02fd0a` - 2x2 sprites making up the score bar at the bottom of
                the screen
 * `0x03070a` - 1x1 sprites of the score digits 0-9
 * `0x03084a` - 12x8 sprites of the TV-like overlay (goal, injury, final score)
 * `0x032c4a` - 1x1 sprite of in-game font
 * `0x0330ca` - 2x2 sprites of various in-game on-screen bits
 * `0x034c4a` - 2x2 sprites of power-ups
 * `0x035dca` - 4x4 player sprites
 * `0x0425ca` - 1x1 sprites used to create the arena
 * `0x04286a` - 4x4-ish sprites of the arena edge graphics
 * `0x0454ca` - `splash_backdrop` - The backdrop for results tables etc.
 * `0x049bfc` - `splash_victory` - Victory image
 * `0x04e66e` - `splash_defeat` - Match loss
 * `0x051fe0` - `splash_win_league`
 * `0x053e1a` - `splash_win_promo`
 * `0x055c34` - `splash_win_cup`
 * `0x057a52` - `splash_win_knockout`
 * `0x059838` - `splash_title` - Title screen
 * `0x05d8ca` - `splash_arena` - Arena background for intro text.
 * `0x0610c4` - Big font, then 2x2 sprites associated with training
                screen. Palette 3.
 * `0x068244` - 4x4 sprites of buttons/body parts, used on training screen
 * `0x072444` - 1x1 font sprites
 * `0x074284` - Player images, used on training screen.
 * `0x07e006` - End of data

Entries with question marks haven't been fully explored.

All splash screens have a palette at offset 2, except
`splash_backdrop`, which is special-cased with 2 palettes, taken from
0x029f5e.

## Palettes

`find_palettes` suggests non-splash palettes located at:

 * 0x0007c4 - Palette 0
 * 0x029ede - Palette 3
 * 0x029efe - Palette 4

Colour scheme #3/#4 looks good for stat management screen buttons, faces.

## VRAM memory map

`display_splash` puts tile data at 0X7d00, tile map (1000 bytes) at
0xe010.

## IFF

TODO: IFF-looking stuff at 1ae16, probably audio. Extract?

## Done

Ranges fully understood:

0x0002260a - 0x0002914e
0x0002e6ca - 0x0007ffff, except 64 bytes at 0x02fcca
