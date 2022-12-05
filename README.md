# Reverse engineering of Speedball 2 for Sega Megadrive (Genesis)

This repo is a reverse engineering of Speedball 2, using Ghidra to
reverse the assembly. Graphics, sound, etc. are extracted by with some
tools I wrote in Rust.

It's an absolute classic of a game, and I'd always kinda wondered how
it was put together, how the AI works, etc. So, why not find out?

I chose the Megadrive version not only because that's the version that
I played, but because the image would be a direct ROM mapping, and the
hardware involved looked relatively tractable. It gave me a great
excuse to learn how the Megadrive hardware worked (and I'd love to
give a great shout-out to the excellent "GenesisSoftwareManual.pdf"
that the internet provided me with). I also didn't need to worry about
any fancy disk image loading, which took away an initial hurdle.

In many ways, the ideal version would have been the Amiga version,
with a nicer playing pitch, the famous "Ice cream!" sample, and
generally more sensible audio. There's a bunch of code in the
Megadrive sound subsystem that only makes sense in terms of a port
from another platform. In other areas, there's dead code that looks
like it was never removed from a porting effort. On the other hand,
this is all evidence that as a 68K Speedball 2, it shares a lot of
heritage with the Amiga version, so perhaps not all is lost.

This is not a perfect reversing - I wanted to get something out, after
all this messing around, so here it is, with a first pass on all the
(reachable) code, but not everything completely worked out.

My approach was to attack the code from a few angles - the ROM entry
point, the obvious graphics resources, code that referenced hardware
and/or those resources, etc. It was fun watching Ghidra trying to work
out what was code and what wasn't. There aren't a huge number of
function pointers, but it took quite a while to identify all the code
nonetheless.

To simplify, my approach roughly proceeded in stages:

 * Work through all the accesses to the GDP that I could find, as well
   as other hardware accesses, interrupt handlers, etc. By
   understanding all the places where the code touches the outside
   world, you can get a handle on what it's trying to achieve. This
   also helped me get a start on the practical side of GDP
   programming.
 * Understand the title sequence, and associated splash
   screens/backdrops. Relatively straightforward code to ramp up on,
   but it includes an interesting little decompression algorithm.
 * Pull apart the sound subsystem, which includes both the sequencer
   and sample-player (with a short excursion into the Z80 sound
   coprocessor!).
 * Menus and training screens - non-game code uses its own sprite
   display system, so this involved reversing that, and from there the
   training screens and so on revealed the structures for player stats
   etc.
 * The core match loop. This included a second set of sprite-display
   routines, and the code to run the match and update the various game
   entities, leaving until last...
 * The player routines. I started with the user-controlled player
   routines; how pressing buttons on the keypad makes your selected
   player behave. From there I incrementally worked my way through the
   game's AI, which includes distinct logic for the active player and
   the rest of the team, and special code for the goalies.

And that, I think pulls apart the whole game. It was a fascinating
journey for me, I don't know if anyone else will enjoy reading it!

The notes I made along the way are in [NOTES.md](NOTES.md). They're a
bit out of date right now. Indeed, there are lots of loose ends,
documented in [TODOs.md](TODOs.md). Maybe I'll tidy them up when I've
recovered a bit from the first pass!

The tools I built along the way are documented in
[tools/README.md](tools/README.md).
