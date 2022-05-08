# Notes on reverse engineering the Speedball 2 Megadrive ROM

## File sections

 * `0x000000` 68k reset, interrupt, etc. vectors
 * `0x000100` Sega ROM id
 * `0x000200` Entry point, start of code.

Looks like graphics start around 0x022e1c, with a splash screen?

This will be some ongoing work, I think, to break it apart.

## Bits of graphics

Rough addresses (to be refined):

 * 0x022c00 - Splash screen
 * 0x026000 - Splash screen #2?
 * 0x029000 - Some data, not sure if graphics?
 * 0x02d580 - Font, followed by something unobvious
 * 0x02e6ca - Strange graphics/data
 * 0x02fcaa - Goal score picture or something? Palette 0
 * 0x032c4a - Another font, followed by graphics-looking stuff,
              palette 0, sprite fragments down to...
 * 0x0454ca - Weird  bits until
 * 0x045c00 - Background tiles. Palette 5.
 * 0x049bfc - Over to the weird data bits...
 * 0x4a400? - Victory picture? Palette 6.
 * 0x4ec00? - Loss picture? Palette ???, close to 6.
 * 0x051fe0 - Not-graphics-looking data
 * 0x0610c4 - Big font, then various bits associated with management
              pages, Palette 3.
 * 0x07e006 - End of data

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
