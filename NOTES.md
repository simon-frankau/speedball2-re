# Notes on reverse engineering the Speedball 2 Megadrive ROM

## File sections

 * `0x000000` 68k reset, interrupt, etc. vectors
 * `0x000100` Sega ROM id
 * `0x000200` Entry point, start of code.

Looks like graphics start around 0x022e1c, with a splash screen?

This will be some ongoing work, I think, to break it apart.

## Palettes

`find_palettes` suggests palettes located at:

 * 0x0007c4
 * 0x02260c
 * 0x025a5e
 * 0x029ede
 * 0x029efe
 * 0x0454cc
 * 0x049bfe
 * 0x051fe2
 * 0x053e1c
 * 0x055c36
 * 0x057a54
 * 0x05983a
 * 0x05d8cc

This list might not be complete, but it feels like a good start!
