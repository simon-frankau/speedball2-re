# Speedball II reverse-engineering custom tools

This directory contains hacky tools I've built to pull stuff out of
the Speedball II ROM. Megadrive graphics aren't really obvious if you
do a naive conversion of the data to an image, so the tools decode the
Megadrive in-memory graphics format.

The tools are all extremely messy, set up for this specific ROM, and
have neither error handling nor command-line configurability. They
just about get a very specific job done.

 * `extract_cells` displays the memory based on the Megadrive's
   2-pixels per byte, 8x8 pixels tile ("cell") format. Most graphics
   are jumbled because the tiles are not simply arranged on the
   screen, but have some intermediate mapping. The colours come from a
   palette, these are hard-wired, coming from...

 * `find_palettes` searches the ROM for things that look like palettes
   for the Megadrive's 16-colour, 9-bit palette system. It has a few
   heuristics that may not be generally applicable to try to narrow
   down the results for this ROM.

 * `display_splash` reconstructs images by emulating the ROM's
   `display_splash` function, pulling out a palette, tile map and set
   of tiles to regenerate an image.
