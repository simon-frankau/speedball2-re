# Z80 code

The z80 code from the ROM.

 * `reset` Routine extracted from 0x0002c4. Just clears everything and
   goes into an infinite loop.
 * `main` Routine extracted from 0x02e620. When triggered, reads data
   from the 68K RAM into the FM1 (channel 1-3) data register.

For each piece of code, I've included the hex dump, binary and
disassembly.
