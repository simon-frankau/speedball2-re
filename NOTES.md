# Notes on reverse engineering the Speedball 2 Megadrive ROM

## File sections

Rough addresses (to be refined):

 * `0x000000` - 68k reset, interrupt, etc. vectors
 * `0x000100` - Sega ROM id
 * `0x000200`? - Entry point, start of code.
 * `0x0007c4` - Start of palettes
 * `0x000824`? - ???
 * `0x013806` - 8SVX audio: start.smp
 * `0x01736e` - 8SVX audio: end.smp
 * `0x01ae16` - 8SVX audio: getready.smp
 * `0x01e392` - 8SVX audio: replay.smp
 * `0x02246c`? - ??? End of audio bit
 * `0x02260a` - `splash_start1` - Arena & Bitmap Brothers
 * `0x025a5c` - `splash_start2` - ImageWorks and Bitmap Brothers
 * `0x02914e`? - ??? Random stuff
 * `0x029e5e` - Palettes
 * `0x009f9e`? - More random stuff?
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
                screen.
 * `0x068244` - 4x4 sprites of buttons/body parts, used on training screen
 * `0x072444` - 1x1 font sprites
 * `0x074284` - Player images, used on training screen.
 * `0x07e006` - End of data

Entries with question marks haven't been fully explored.

All splash screens have a palette at offset 2, except
`splash_backdrop`, which is special-cased inside `display_splash` to
use the 2 palettes `palette_backdrop_[ab]`.

## Palettes

Each "splash" screen comes with its own palette. Other palettes are
used for the sprites, mostly multiple palettes for the different
teams.

There are two sets of other palettes. The first are the in-game
palettes, used when playing matches:

 * `0x0007c4` - `palette_game_a` Red team
 * `0x0007e4` - `palette_game_b` Blue team
 * `0x000804` - `palette_game_c` Red team

These palettes end at 0x000824.

The second set is for outside matches:

 * `0x029e5e` - `palette_gold_a` Golden version of `palette_game_a`
 * `0x029e7e` - `palette_gold_b` Golden version of `palette_game_b`
 * `0x029e9e` - `palette_gold_c` Golden version of `palette_game_c`.
                 Like `palette_gold_a`, except one of the browns is
                 now black?!
 * `0x029ebe` - `palette_mono` Monochromatic palette
 * `0x029ede` - `palette_training_a` Used on the training screens
 * `0x029efe` - `palette_training_b` Identical to 3a.
 * `0x029f1e` - `palette_magenta_a` Pure magenta palette. Placeholder?
 * `0x029f3e` - `palette_magenta_b` Pure magenta palette. Placeholder?
 * `0x029f5e` - `palette_backdrop_a` 1st half of palette used by
                `splash_backdrop`. Blue tinted. Colour 7 has a bit
                less blue than the palette actually in
                splash_backdrop.
 * `0x029f7e` - `palette_backdrop_b` 2nd half of palette used by
                `splash_backdrop`. Gold tinted.

These palettes end at `0x029f9e`.

There's also some colour-like data at:

 * `0x02914e` - Not a real palette, but looks like a colour-cycle of
   fading blues?
 * `0x02cb3e` and later has words counting up that could be shades of
   red. Seems unlikely, probably some other lookup tables.

## VRAM memory map

`display_splash` puts tile data at 0X7d00, tile map (1000 bytes) at
0xe010.

## Done

Ranges fully understood:

0x00000000 - 0x00000200
0x000007c4 - 0x00000824
0x00013806 - 0x0002246c
0x0002260a - 0x0002914e
0x00029e5e - 0x00029f9e
0x0002e6ca - 0x0007ffff, except 64 bytes at 0x02fcca
