# Notes on reverse engineering the Speedball 2 Megadrive ROM

## File sections

Rough addresses (to be refined):

 * `0x000000` - 68k reset, interrupt, etc. vectors
 * `0x000100` - Sega ROM id
 * `0x000200`? - Entry point, start of code.
 * `0x0007c4` - Start of palettes
 * `0x000824`? - ???
 * `0x00f5e2` - Sound code
 * `0x010280` - FM sound bank
 * `0x011a42` - Sound table
 * `0x011b22` - Sound sequences
 * `0x0135b0` - Sequence table
 * `0x013770` - Sound instrument mapping
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
 * `0x02e620` - Z80 sound code
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

## Sound

Rough overview:

A sound is a sample, or set of 4 voices (Amiga-ism?) being played.
Each voice plays a sequence (stream of data) on an instrument (which
has an associated data structure). Each instrument playing a sequence
is mapped to a channel on the FM chip. There are 6 channels, arranged
in 2 banks of 3. Each channel has 4 FM operators which may be
configured to work together in a number of ways. Each operator has its
own config.

Channel and operator are the chip's terminology, the rest are my names
for what I've reversed.

 * Sound code runs from 0x00f5e2 to 0x010280.
 * FM sound bank runs from from 0x010280 to 0x011a42.
 * Sound table runs from 0x011a42 to 0x011b22.
 * Sequences run from 0x011b22 to 0x0135b0.
 * Sequence table runs from 0x0135b0 to 0x013770.
 * Sound instrument mapping table runs from 0x013770 to 0x013806.
 * Samples run from 0x013806 to 0x02246c.

Sound data is at 0xffea74..0xffed90.

### TODOs

Enter the memory map into the top-level map.

Code still to reverse:

 * Use of sound_var_unk_?
 * Look for any addresses remaining unnamed (All variables named up to fde4).
 * sound_op_* commands from sound_command_table need reversing.

Data to understand:

 * Sequence at 0x00012240 is never used. Could this be the missing
   entry for sound_seq_4f. Why's it disabled?
 * Build tools to extract instruments, sequences, sounds.

### Note information

The first part of the structure is processed by `sound_process_voice`:

 * A0[0x00]b - Is sound playing?
 * A0[0x01]b - Channel number
 * A0[0x02]l - Pointer to instrument data - loaded in A1 (see below).
 * A0[0x06]b - Pitch (semi-tones)
 * A0[0x07]b - Stereo control (L-enable = bit 1, R-enable = bit 0)
 * A0[0x08]b - Volume
 * A0[0x09]b - Appears to be unused? Alignment padding?
 * A0[0x0a]w - Note duration/time remaining
 * A0[0x0c]l - Pitch shift (additive change to cycle length)
 * A0[0x10]l - Glissando shift (pitch shift added each step
 * A0[0x14]w - Glissando duration (number of gliss steps remaining)
 * A0[0x16]l - Vibrato phase
 * A0[0x1a]l - Vibrato rate (added to phase each step).

TODO: The following is still a bit messy.

The rest are used by `sound_update`:
 * A0[0x1e]w - If zero, stop sound
 * A0[0x1f]  - ''
 * A0[0x20]l - Stored in A3, this is a sequence stack, for allowing nested
               sequences.
 * A0[0x21]  - ''
 * A0[0x22]  - ''
 * A0[0x23]  - ''
 * A0[0x24]  -
 * A0[0x25]  -
 * A0[0x26]b - If zero, step the note. Otherwise zero this byte and
   start the note (unless pitch < 2, in which case it's also stopped).
...
 * A0[0x26]w - Zeroed when playing stops
 * A0[0x28]l - Another command pointer
 * A0[0x2c]l - If zero, go straight to next voice. Otherwise loaded into D0, then A2, and... it's a pointer to the commands! Put in A2.
 * A0[0x30]w - If non-zero, decrement and go to next voice
 * A0[0x32]w - Value to reset 0x30 to, when processed.
 * A0[0x36]b - ?
 * A0[0x38]b - ?
 * A0[0x3a]w - ?
 * A0[0x40]w - Voice number. TODO: Index into some table, cleared when note stops.
 * A0[0x42]w - Time to next update length multiplier.

Total length of the structure is 0x44.

### Instrument information

Each instrument is 0x3f in size

 * A1[0x00]  - Name
 * A1[0x0a]b - Initial duration
 * A1[0x0b]b - Transposition (in semi-tones, 0x80 = nothing)
 * A1[0x0c]b - FM feedback setting (register 0xb0)
 * A1[0x0d]b - FM algorithm setting (register 0xb0)
 * A1[0x0e]b - Glissando size (in semi-tones, 0x80 = nothing)
 * A1[0x0f]b - Not gliss flag
 * A1[0x10]b - Gliss duration
 * A1[0x11]b - Vibrato size multiplier
 * A1[0x12]b - Vibrato rate
 * A1[0x13]  - Operator 1 (operator structure described below)
 * A1[0x1e]  - Operator 2
 * A1[0x29]  - Operator 3
 * A1[0x34]  - Operator 4

### Operator structure

 * 0x00b - Enable
 * 0x01b - FM detune
 * 0x02b - FM mul
 * 0x03b - TL (volume)
 * 0x04b - RS
 * 0x05b - AR
 * 0x06b - AM
 * 0x07b - D1R
 * 0x08b - D2R
 * 0x09b - D1L (multiplied by note volume, and otherwise adjusted)
 * 0x0ab - RR

## Done

Ranges fully understood:

0x00000000 - 0x00000200
0x000007c4 - 0x00000824
0x00013806 - 0x0002246c
0x0002260a - 0x0002914e
0x00029e5e - 0x00029f9e
0x0002e6ca - 0x0007ffff, except 64 bytes at 0x02fcca
