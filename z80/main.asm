#target ROM
#code HOH, 0, $FFFF

L0000:	JP	Entry

	;; Stack.
L0003:	DEFB $00,$00,$00,$00,$00,$00
L0009:	DEFB $00
L000A:	DEFB $00
	DEFB $00,$00,$00,$00,$00,$00,$00,$00
	DEFB $00,$00,$00,$00,$00,$00
        ;; Stack grows down from here.
L0019:	LD	BC,$6000
	LD	DE,($0004)
	LD	A,E
	LD	E,$01
	RLCA
	AND	E
	LD	(BC),A
	LD	A,D
	AND	E
	LD	(BC),A
	RRC	D
	LD	A,D
	AND	E
	LD	(BC),A
	RRC	D
	LD	A,D
	AND	E
	LD	(BC),A
	RRC	D
	LD	A,D
	AND	E
	LD	(BC),A
	RRC	D
	LD	A,D
	AND	E
	LD	(BC),A
	RRC	D
	LD	A,D
	AND	E
	LD	(BC),A
	RRC	D
	LD	A,D
	AND	E
	LD	(BC),A
	RRC	D
	LD	A,D
	AND	E
	LD	(BC),A
	LD	DE,$8000
	RET

Entry:	DI
	LD	SP,$0019
	LD	A,$42
	LD	(L0009),A

L1:	LD	A,($0008)
	OR	A
L2:	JP	Z,L1
	CALL	$0019
	LD	A,$01
	LD	($0008),A
	LD	DE,($0003)
	SET	7,D
	LD	HL,($0006)
	LD	A,$99
L3:	LD	($0009),A

	;; Delay loop
	LD	B,$10
Delay:	DJNZ	Delay

	LD	A,($0008)
	OR	A
	JP	Z,L3
	JP	M,L2
	LD	A,H
	OR	L
	JP	Z,L3
	LD	A,(DE)
	ADD	A,$80
	LD	($4001),A
	INC	DE
	DEC	HL
	LD	A,D
	OR	E
	JP	NZ,L3
	LD	IX,($0004)
	LD	BC,$0080
	ADD	IX,BC
	LD	($0004),IX
	CALL	$0019
	JP	L3
