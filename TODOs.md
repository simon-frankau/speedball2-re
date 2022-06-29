# Possible areas to investigate from

 * Have I got the button order incorrect?

## Data usage

 * Accessing sprites
 * Usage of identified variables

### More backgrounds and images etc.

 * Deal with calls to `draw_training_background`
  * Identify 2x2 maps
  * Update extractors to handle it.

 * `set_monitor_overlay` could be reversed more?
 * `todo_build_final_score_overlay` needs a lot of work.

## Loose ends

Anything containing "TODO" or "unk".

Remember to mark loose ends "TODO" or "unk"!

Code reached from the init process

## I/O locations

**All done**

 * All uses of the I/O area
 * All uses of VDP
 * All uses of FM chip
 * All accesses of Z80 resources

## Splash screens

**All done**

 * `splash_backdrop`
 * `splash_victory`
 * `splash_defeat`
 * `splash_win_league`
 * `splash_win_promo` 
 * `splash_win_cup`
 * `splash_win_knockout`
 * `splash_start1`
 * `splash_start2`
 * `splash_title`
 * `splash_arena`
 * `display_splash`

## Fonts

**All done**

 * `0x029fde` - Font for "PUSH START"
 * `0x02d7ec` - 1x1 font password font
 * `0x02e6ca` - Title screen font, 2x2.
 * `0x032c4a` - 1x1 sprite of in-game font
 * `0x0610c4` - Big font, then 2x2 sprites associated with training
                screen.
 * `0x072444` - 1x1 font sprites

## Other things

 * Text - pretty much done
 * Palettes - done
 * 
