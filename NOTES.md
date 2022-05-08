# Notes on reverse engineering the Speedball 2 Megadrive ROM

## File sections

 * `0x000000` 68k reset, interrupt, etc. vectors
 * `0x000100` Sega ROM id
 * `0x000200` Entry point, start of code.

Looks like graphics start around 0x022e1c, with a splash screen?

This will be some ongoing work, I think, to break it apart.

## Bits of graphics

Should become file sections. Viewed at 32 cells across.

8 rows = 32 cells x 32 bytes/cell - 1024 bytes
1 row = 128 bytes

 * Row 1112 - Splash screen
 * Row 1216 - Splash screen #2?
 * Row 1312 - Some data, not sure if graphics?
 * Row 1451 - Font, followed by something unobvious
 * Row 1488 - Strange graphics/data
 * Row 1527 - Goal score picture or something? Palette 0
 * Row 1624 - Another font, followed by graphics-looking stuff,
              palette 0, sprite fragments down to...
 * Row 2216 - Weird  bits until
 * Row 2232 - Background tiles. Palette 5.
 * Row 2360 - Over to the weird data bits...
 * Row 2376 - Victory picture? Palette 6.
 * Row 2520 - Loss picture? Palette ???, close to 6.
 * Row 2624 - Not-graphics-looking data
 * Row 3104 - Big font, then various bits associated with management
              pages, Palette 3.
 * Row 4031 - End of data

## Palettes

`find_palettes` suggests palettes located at:

 * 0x0007c4 - Palette 0
 * 0x02260c - Palette 1
 * 0x025a5e - Palette 2
 * 0x029ede - Palette 3
 * 0x029efe - Palette 4
 * 0x0454cc - Palette 5
 * 0x049bfe - Palette 6
 * 0x051fe2 - Palette 7
 * 0x053e1c - Palette 8
 * 0x055c36 - Palette 9
 * 0x057a54 - Palette 10
 * 0x05983a - Palette 11
 * 0x05d8cc - Palette 12

This list might not be complete, but it feels like a good start!

Colour scheme #3/#4 looks good for stat management screen buttons, faces.

Colour scheme #5 looks good for the background panel.

#6 maybe for the defeat image?

Each row - 32 cells,

Colour scheme #0 - Row 1516 - victory screen? Maybe sprites at 1728?
Colour scheme #1/2 - -Nope
Colour scheme #3/4 - Row 3103 (tail of resources)onwards looking pretty sweet.
#5 - Backdrop at 2232
#6 - Nothing useful
#7-#10 - Nothing useful. Misaligned?
#11-#12 - Nothing useful?
