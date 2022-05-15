#target ROM
#code HOH, 0, $FFFF

;; Initial 0x26 bytes of code that's copied from the ROM.
L0000:	XOR		A
	LD		BC,$1FD9
	LD		DE,$0027
	LD		HL,$0026
	LD		SP,HL
	LD		(HL),A
	LDIR
	POP		IX
	POP		IY
	LD		I,A
	???
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
	DI
	IM		1
	LD		(HL),$E9
	JP		(HL)

;; Following Z80 code in the ROM that doesn't get copied over initially.
	ADD		A,C
	INC		B
	ADC		A,A
	LD		(BC),A
	RET		NZ
	NOP
	NOP
	NOP
	LD		B,B
	NOP
	NOP
	DJNZ	$FFFFFFD2
	CP		A
	RST		$18
	RST		$38
