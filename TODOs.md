# Possible areas to investigate from

## Data usage

 * Accessing splash screens
 * Accessing sprites
 * Accessing fonts
 * Accessing palettes
 * Accessing text
 * Usage of identified variables

### Splash screens

TODO:

 * `splash_backdrop` - `show_title_todo`
 * `splash_victory` - `victory1_todo` and `victory2_todo`
 * `splash_defeat` - `defeat1_todo` and `defeat2_todo`
 * `splash_win_league` - `win_league_todo`
 * `splash_win_promo` -  `win_promo_todo`
 * `splash_win_cup` - `win_cup_todo`
 * `splash_win_knockout` - `win_knockout_todo`

DONE:

 * `splash_start1`
 * `splash_start2`
 * `splash_title`
 * `splash_arena`
 * `display_splash`

### More backgrounds and images etc.

 * Deal with calls to `draw_training_background`
 * Identify 2x2 maps
 * Update extractors to handle it.
 * Should split apart sprite_players to match what's loaded into RAM?

 * `set_monitor_overlay` could be reversed more?
 * `todo_build_final_score_overlay` needs a lot of work.
 * `transfer_cells` looks very interesting (or at least, its usage does!)

### Fonts

 * `0x029fde` - Font for "PUSH START"
    * **DONE**
 * `0x02d7ec` - 1x1 font
 * `0x02e6ca` - Title screen font, 2x2. `sprite_titles_font`
   * **DONE**
 * `0x032c4a` - 1x1 sprite of in-game font
 * `0x0610c4` - Big font, then 2x2 sprites associated with training
                screen. `sprites_big_font`
 * `0x072444` - 1x1 font sprites

## Loose ends

Anything containing "TODO" or "unk".

Remember to mark loose ends "TODO" or "unk"!

Code reached from the init process

Data that's part of the pile copied from ROM to RAM.

## I/O locations

**All done**

 * All uses of the I/O area
 * All uses of VDP
 * All uses of FM chip
 * All accesses of Z80 resources
