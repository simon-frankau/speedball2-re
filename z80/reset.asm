#target ROM
#code HOH, 0, $FFFF

;; Initial 0x26 bytes of code that's copied from the ROM.

        ;; Zero from after this code to end of RAM (0x2000)
L0000:	XOR		A
	LD		BC,$1FD9
	LD		DE,$0027
	LD		HL,$0026	; Just after end of code.
	LD		SP,HL
	LD		(HL),A  	; Zero start of unused.
	LDIR				; Copy byte to next up to 0x2000
        ;; Zero all registers
	POP		IX
	POP		IY
	LD		I,A
	LD		R,A
	POP		DE
	POP		HL
L0018:	POP		AF
	EX		AF,AF'
	EXX
	POP		BC
	POP		DE
	POP		HL
	POP		AF
	LD		SP,HL
        ;; Interrupts disabled with interrupt mode 1
        ;; (interrupts trigger RST38)
	DI
	IM		1
        ;; Write 'JP (HL)' to address zero.
	LD		(HL),$E9
        ;; And then jump to address zero, for a nice infinite loop.
	JP		(HL)
