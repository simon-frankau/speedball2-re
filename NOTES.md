# Notes on reverse engineering the Speedball 2 Megadrive ROM

## File sections

Rough addresses (to be refined):

 * `0x000000` - 68k reset, interrupt, etc. vectors
 * `0x000100` - Sega ROM id
 * `0x000200`? - Entry point, start of code.
 * `0x000614` - RAM initialisation code.
   * `0x0007c4` - Start of palettes
   * `0x000824`? - ???
 * `0x002be4'? - Main entry point
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

 * `0x02914e`? - Misc data
 * `0x0291ac`? - Code to do with Sega logo. TODO
 * `0x0291da` - Sega logo
 * `0x0297fa`? - Misc data
 * `0x02982a`? - Code TODO
 * `0x029b38`? - Strings
 * `0x029e20` - Checksum code
 * `0x029e5e` - Palettes
 * `0x029f9e` - Just 64 x 0x0e
 * `0x029fde` - Font for "PUSH START"
 * `0x02a0de`? - Data
 * `0x02a16e`? - Strings and string pointers
 * `0x02a910`? - Unknown TODO

 * `0x02d7ec` - 1x1 font
 * `0x02e0ec`? - Various data structures
 * `0x02e620` - Z80 sound code
 * `0x02e6ca` - Title screen font, 2x2.
 * `0x02fcaa` - ASCII code to title font index mapping
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
 * `0x074284` - Player faces, used on training screen.
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

## Graphics overview

There are two main graphics configurations:

 * Non-game graphics mode, configured by `display_configure_non_match`.
 * In-game graphics mode, configured by `display_configure_match`.

The details of these modes are described in the following
section. This subsection describes common features.

### Common VDP configuration

While `start` does a bit of VDP configuration, the real game-specific
configuration occurs in `init_vdp`. Key configuration includes:

 * Setting the screen to display 40x28 cells, no interlace.
 * Setting scroll size to 64x32 cells.
 * Setting Scroll A cell map's base address to 0xe000.
 * Setting Window cell map's base address to 0xf000.
 * (Setting Scroll B's base address to 0xe000 - unused,)
 * Setting the sprite attribute base address to 0xd800.
 * Putting H scroll data table at 0xdc00

Despite setting up the display for 28 cells vertically, only 25 (*) are
drawn to (hence most cell map writes starting at 0xe080/0xf080, a row
in), presumably due to CRT overscan meaning the othe rows aren't
guaranteed to be visible.

(*) "PUSH START" gets drawn on row 26.

Many functions use `vdp_address_set` to convert between a VRAM address
and the value that needs to be written to `vdp_control`.

### Palettes

TODO

`display_fade_in` and `display_fade_out`, also `display_splash`.

Updates in `v_interrupt_handler`.

## Non-game graphics mode

### VDP memory map

Configured by `display_configure_non_match`. The map looks like this:

 * 0x0000-0x7d00 holds the cells representing the Window layer.
 * 0x7d00-... holds the cells for the background/Scroll A layer.
 * 0xd800-0xd808 - Sprite attribute base, zero'd.
 * 0xdc00-0xdc40 - H scroll data table base address.
 * 0xdc40 is blank cell (filled with 0x0)
 * 0xdc60 is cell all filled with 0x7.
 * 0xdc80 is cell all filled with 0x6.
 * 0xdca0-0xdda0 - 8 cells for "PUSH START"
 * 0xe000-0xf000 holds the Scroll A cell map, initialised to point at
   0xdc40.
 * 0xf000-0xffff holds the Window cell map, initially zero'd, then
   configures as below.

The actually-used cell maps start at 0xe080/0xf080, consisting of a
40x25 grid of cells, with a stride between rows of 64 cells.

The Window cell mapping, points to a 25x40 transposed set of cells at
0x0000. Rows 3-22 use palette 1, the rest use palette 0. All cells
have the priority bit set. Hardware scrolling is reset.

The cells being transposed makes sub-cell Y positioning easier - you
can always add/subtract 4 bytes to go to the next/previous row.

While 25x40 cells are set up for the Window mapping, 0x0000-0x8c00 is
initially zero'd - enough storage for 28x40 cells. Presumably at some
point 28 rows were used for the overlay?

### Background layer

Usually, the background (Scroll A) layer is loaded by
`display_splash`, which also initialises the palette. Once the cell
mapping is written, the cell data itself is written to 0x7d00. The
cell data isn't a full screen (like the Window layer), but makes use
of the cell mapping to remove redundancy (empty/repeated cells, etc.).

Some backgrounds are compressed in ROM. They are decompressed into
`backbuffer` before being transferred to VDP RAM.

A few screens (the manager, transfer and gym screens) are
different. They are built by `draw_management_background`, which
copies the shared cell data for the backdrops into 0x7d00, and then
writes the cell map into 0xe080 onwards in 2x2 cell blocks.

### Back-buffering and blitting of foreground

Most of the non-game mode drawing on the Window/foreground layer is
handled by a simple back-buffered bitmap - a one-to-one cell mapping
is provided onto the screen, and then the cell contents are
updated. The back-buffering is done by drawing into the `backbuffer`
memory (X major array of cells, 25x40 in size).

`backbuffer` is modified by:

 * `draw_box_colour` (calling `draw_colour_h_line`)
 * `draw_xor_square` (calling `draw_xor_horizontal_line` and
   `draw_xor_vertical_line`)
 * `put_cell` - used by various display functions.
 * `put_masked_cell` - ditto.

 Once modified, the cells to blit into the VDP RAM need to be
 recorded. To do this, we have:

 * `schedule_box` - used by `draw_box_colour` and `draw_box_square`
 * `schedule_cell_transfer`, which returns an address to draw into,
   and is used by `put_cell` and `put_masked_cell`.

These functions write to `cell_list`, updating `cell_list_end`.

The function to then blit `backbuffer` to the VDP is then
`tranfer_cells` (which is also called by the transfer-scheduling
functions if the cell list overflows). `transfer_cells` synchronises
this transfer with a vsync.

### Misc drawing

A few routines don't go via `back_buffer`:

 * `display_push_start_non_match` writes the cells for "PUSH START"
   into row 26 of the Window layer.
 * `display_screen_block` draws a big block of the cell coloured 0x6
   (at 0xdc80) into the middle of the background (Scroll A)
   layer. This is used to provide a flat background for the various
   stats tables etc. in the UI.
 * `vdp_write_2_cell` is used by `display_title_font_char` to print
   parts of the intro text into the Window (overlay) layer cells.

## In-game graphics

### VDP memory map

Configured by `display_configure_match`. The map looks like this:

 * 0x0000-0x2f00 - 0x178 cells of cells representing the arena
 * 0x2f00-0xc100 - 0x490 cells of player sprites
 * 0xc100-0xc500 - 4x4 sprite of bumper
 * 0xc500-0xd100 - 0x60 cells reserved for TV monitor.
 * 0xd100-0xd780 - Sprites: 2 Medibots (4x4), ball launcher (4x4),
   ball (2x2)
 * 0xd780-0xd7c0 - 2 cells used for offscreen marker 1.
 * 0xd7c0-0xd800 - 2 cells used for offscreen marker 2.
 * 0xd800-0xd900 - 32 hardware sprite definitions
 * 0xd900-0xda00 - 8 cells for "PUSH START"
 * 0xda00-0xdc00 - 4 2x2 sprites - coin and 3 power-ups
 * 0xdc00-0xdc40 - H scroll data table base address.
 * 0xdc40 is blank cell (filled with 0x0)
 * 0xdc60 is a cell all filled with 0x7.
 * 0xdc80-0xe000 - Sprites: 2x player markers (2x2), player number
   (2x2), big ball (4x4)
 * 0xe000-0xee00 - Cell mapping for Scroll A (background) layer - 64x28 mapping
 * 0xf000-0xfe00 - Cell mapping for Window (foreground) layer - 64x28 mapping

 * One row 0xdc60 (all 7s), palette 2, priority set
 * 23 rows of 0xdc40 (blank cell)
 * 2 rows of status bar, palette 2, priority set
 * 2 rows of 0xdc60 (all 7s)

The cell mapping is then overwritten to display cells making up the
"TV monitor" from 0xc500 (12x8 cells).

The cells that make up the status bar are stored in all the "dead
space" of the cell mappings 0xe000-0xffff.

(The offscreen markers aren't initialised by
`display_configure_match`, they're set separately).

### Initialisation

The VDP memory is configured for game graphics in
`display_configure_match`.

### Background and cell mapping

`transfer_cell_map_with_scroll` is an implementation of a
semi-software scrolling for the Scroll A (background) layer. This is
needed because the cell map in VRAM is 64x32, but the pitch is
80x144. It reads from an 80x144 cell mapping in `backbuffer`, and
uses `set_hw_scroll` to manage sub-cell scrolling.

The mapping is initialised by `init_pitch_cell_map` (helped by
`write_background_cell_mapping`).

Some modifications are made dynamically:

 * `draw_background_sprite` will overwrite 2x2 cell regions, saving
   the originals to `background_save_stack`, so that they may be
   restored by `restore_background`.
 * `update_background_sides` will overwrite the cells associated with
   the stars and electric zappers, taking the data from `edge_blocks`.
 * The score multipliers are modified in the background layer in a
   complicated way that's handled in the "sprites" section.
 
### Monitor overlay (Window layer)

The TV monitor overlay is drawn into through cells at
0xc500-0xd100. These cells are permanently mapped in the Window layer.

`set_monitor_overlay` calls `add_monitor_overlay` to blit the image
into VDP RAM, or clears the cells if there's no overlay.

### Misc drawing (Window layer)

 * `draw_health_meter` calls `draw_health_meter_aux` to update the
   health bars in the status bar, by drawing on the associated cells.
 * `display_score_digit` is used to draw the score into the cells of
   the status bar.
 * `display_time_digit` draws the time remaining into the cells of the
   status bar.
 * `draw_status_bar_position` draws a 2x2 image into the cells of the
   status bar. Used for displaying the little green status lights.
 * `overwrite_cell` copies a cell into VRAM. It is used by
   `draw_status_bar_powerup` to draw the current power-up in the
   status bar. Very similar to what `draw_status_bar_position` does,
   except source image is not transposed.
 * `draw_cell_marker`, calling `draw_cell_markers_aux`, draws the
   markers for active players that are offscreen, into the cells at
   0xd780 and 0xd7c0, and then updates the cell mapping in the Window
   overlay layer to show them. The overwritten cells are saved so that
   `clear_offscreen_markers` and `clear_offscreen_markers_aux` can
   clear the mappings to hide the markers.

### Back-buffering and blitting of foreground/sprites

Just as non-game mode has `cell_list` and `cell_list_end` to schedule
cells, game mode has `game_cell_list` and `game_cell_list_end` to
store the set of cells scheduled to be transfered to VRAM.

To create a sprite, any cells that need to be transferred to VRAM are
queued on `game_cell_list_end`, and then the sprite itself is defined
by calling `build_hw_sprite`, which supports up to 32 sprites, writing
them into `hw_sprite_start`, with the current pointer `hw_sprite_ptr`
and counter `hw_sprite_count`.

The functions that append to `game_cell_list_end` are sprite drawing
functions, accessed via function pointers on the sprite objects
(offset 4 within the object), and called by `draw_sprite` and
`replay_frame` (see below). Not all sprites need to add cells to
transfer, since some cells are pre-allocated in the VRAM. These just
need to call `build_hw_sprite`.

The function `transfer_hw_sprites` then transfers the sprite list to
VRAM, transfers the needed cells to VRAM, and clears the lists.

### Sprite objects

In-game sprites are manipulated via sprite objects, which contain both
the information on how to draw them and their state and how to
animate/update them.

TODO: `draw_sprite` is also used to draw non-game sprites.

#### Sprite drawing functions

Each sprite has a sprite drawing function to specify how it's
displayed. The functions are as follows (sorted by location of code in
memory):

| Function name             | Cell transfer? | Build sprite? | Sprite address   |
|---------------------------|----------------|---------------|------------------|
| `sprite_fn_player`        | No (1)         | Yes           | 0x2f00-0xc100    |
| `sprite_fn_ball_launcher` | Yes            | No (2)        | 0xd500-0xd700    |
| `sprite_fn_bumper`        | Yes            | No (2)        | 0xc100-0xc500    |
| `sprite_fn_big_ball`      | Yes            | Yes           | 0xde00-0xe000    |
| `sprite_fn_ball`          | Yes            | Yes           | 0xd700-0xd780    |
| `sprite_fn_coin`          | Yes (3)        | No (4)        | 0xda00-0xda80    |
| `sprite_fn_power_up_1`    | Yes (3)        | No (4)        | 0xda80-0xdb00    |
| `sprite_fn_power_up_2`    | Yes (3)        | No (4)        | 0xdb00-0xdb80    |
| `sprite_fn_power_up_3`    | Yes (3)        | No (4)        | 0xdb80-0xdc00    |
| `sprite_fn_blue_marker`   | Yes            | Yes           | 0xdc80-0xdd00    |
| `sprite_fn_red_marker`    | Yes            | Yes           | 0xdd00-0xdd80    |
| `sprite_fn_player_number` | Yes            | Yes           | 0xdd80-0xde00    |
| `sprite_fn_medibot_1`     | Yes            | Yes           | 0xd100-0xd300    |
| `sprite_fn_medibot_2`     | Yes            | Yes           | 0xd300-0xd500    |
| `sprite_fn_multiplier_a`  | No (5)         | No (5)        | In 0x0000-0x2f00 |
| `sprite_fn_multiplier_b`  | No (5)         | No (5)        | In 0x0000-0x2f00 |

 1. Player sprites are always present in VRAM.
 2. This "sprite" appears in the arena background. The transfer simply
    overwrites the cells used by the background cell mapping.
 3. Using shared code in `sprite_fn_power_up_common`.
 4. Since these small sprites are cell-aligned, they're just drawn
    into the background cell mapping, saving the use of a hardware
    sprite.
 5. These sprite are not cell-aligned, but they're still drawn
    directly into the background cells directly in a surprisingly
    complicated way by these functions.

The score multipliers are particularly complicated. The bumper and
ball launcher are drawn into by overwriting the cells in the
background mapping, to make it easy to find the cells to write to. The
score multiplier code, instead, finds the cells in the background
mapping and overwrites them, perhaps because all VRAM was used at that
point.

As far as I can tell, they're not actually drawn as sprites with
`draw_sprite`, but the drawing functions are called directly by
`draw_score_multipliers`. This is probably because there are 4
sprites, for 2 sets of multiplier lights on each side, but we don't
actually need to draw both sides because overwriting the cells for one
side changes the cells for the other side at the same time.

Overall, it feels like the sprite system was written with a very
general approach, originally, and then modified with various pieces
moving away from a universal `draw_sprite` path.

### Replay

Action replay is provided by keeping a circular buffer of sprites to
draw. `draw_sprite` calls `save_sprite_for_replay`, which calls
`write_replay_buf` to save the data in `replay_buf` (pointed to by
`replay_buf_head_ptr` and `replay_buf_tail_ptr`. Frames are separated
by recorded screen origins, created by `save_replay_origin`.

Replays are then performed by `run_replay`, which calls `replay_frame`
and ultimately `read_replay_buffer`. One interesting constraints of
the replay infrastructure is that sprite appearance must only depend
on position and sprite id, no other factors that may not be
recalculated during the replay. There are a few hacks in the code to
meet this constraint.

### Animation

### Collision etc.

## RAM memory map

 * 0xff0114-0xff0d05 initialised from 0x0007c4-0x0013b5 by todo_init_vars.
 * 0xff1105-??? is scratch space used by e.g. QPAC-decompress.
 * 0xffcff4-0xffd470 initialised from 0x0013d4-0x001850 by init_todo_blah_table_ram.
 * 0xffd470-0xffe1a4 initialised from 0x001872-0x0025a6 by init_todo_something_table_ram.
 * 0xffe1a4-0xffe7c2 initialised from 0x0025cc-0x002bea by init_todo_another_table_ram.

I initialised these in Ghidra using code like this:

```
memory = currentProgram.getMemory()
for addr in range(0xffcff4, 0xffd470):
    memory.setByte(toAddr(addr), memory.getByte(toAddr(addr - 0xffcff4 + 0x0013d4)))
```

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
 * A0[0x20]l - Stored in A3, this is a sequence stack, for allowing nested
               sequences.
 * A0[0x24]  - ?
 * A0[0x25]  - ?
 * A0[0x26]b - If zero, step the note. Otherwise zero this byte and
   start the note (unless pitch < 2, in which case it's also stopped).
 * A0[0x27]b - ?
 * A0[0x26]w - Zeroed when playing stops.
 * A0[0x28]l - Pointer to command representing the start of the sequence.
 * A0[0x2c]l - Command pointer. Usually loaded into A2. If zero, do nothing.
 * A0[0x30]w - Every update, ff non-zero, decrement and go to next
               voice. i.e. time-to-next-update
 * A0[0x32]w - Frames between updates. Copied into 0x30 when it reaches zero.
 * A0[0x34]b - ?
 * A0[0x35]b - ?
 * A0[0x36]w - Unknown variable, set by sequencer.
 * A0[0x38]b - Unknown variable, updated by sequencer.
 * A0[0x39]b - ?
 * A0[0x3a]w - Unknown variable, modified by sequencer.
 * A0[0x3c]b - ?
 * A0[0x3d]b - ?
 * A0[0x3e]b - ?
 * A0[0x3f]b - ?
 * A0[0x40]w - Voice number. TODO: Index into some table, cleared when note stops.
 * A0[0x42]w - Tempo, stored as number of frames per beat

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

## Cheats

There are two cheat codes, I decoded from the hex values with the
following Haskell:

```
let decode = map (\x -> if x == 0x3f then '-' else if x >= 26 && x < 36 then (chr (x + ord '0' - 26)) else (chr $ x + 0x41))
Prelude Data.Char> decode input
"THE-EASIER-GAME-PASSWORD01234567"
Prelude Data.Char> decode input2
"THE-PLAYTESTERS-PASSWORD31415926"
```

## Ghidra colours

Colour scheme is:

 * **Green** Pretty completely understood code
 * **Yellow** Incompletely understood code
 * **Blue** Misc ROM data
 * **Pink** Data copied to RAM
 * **Purple** Understood data
 * **Grey** is dead code

## Done

Ranges fully understood:

0x00000000 - 0x00000200
0x000007c4 - 0x00000824
0x00013806 - 0x0002246c
0x0002260a - 0x0002914e
0x00029e5e - 0x00029f9e
0x0002e620 - 0x0007ffff
