#target ROM
#code HOH, 0, $FFFF

	;; Bank select register
BankReg:	EQU	$6000
	;; Banked data location
BankBase:	EQU	$8000
	;; Data register for FM chip
FM1Data:	EQU	$4001

Start:		JP	Entry

	;; Effectively the lower byte of the address,
	;; but is indexed in z80 space, not 68k space.
Source:		DEFB	$00
	;; Mid and upper byte of 68k address.
BankNum:	DEFW	$0000
	;; Number of bytes to write.
ByteCount:	DEFW	$0000
	;; Signal to do next step. Non-zero triggers it,
	;; Top bit set breaks out of current write cycle.
SignalIn:	DEFB	$00
	;; Signal out about whether we're still processing.
SignalOut:	DEFB	$00
	;; Stack space
StackBase:	DEFB	$00,$00,$00,$00,$00,$00,$00,$00
		DEFB	$00,$00,$00,$00,$00,$00,$00
	;; Stack grows down from here.
StackTop:

	;; Select bank by writing A15-A23 serially into LSB of BankReg.
SelectBank:	LD	BC,BankReg
		LD	DE,(BankNum)
	;; Write top bit of E.
		LD	A,E
		LD	E,$01
		RLCA
		AND	E
		LD	(BC),A
	;; Bottom bit of D.
		LD	A,D
		AND	E
		LD	(BC),A
	;; Next bit...
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
	;; Until all 9 bits have been written.
	;; Point DE at the bank area and return.
		LD	DE,BankBase
		RET

Entry:		DI
		LD	SP,StackTop
		LD	A,$42
		LD	(SignalOut),A

L1:		LD	A,(SignalIn)
		OR	A
L2:		JP	Z,L1
		CALL	SelectBan
	;; Claer top bit, so we don't immediately exit the loop?
		LD	A,$01
		LD	(SignalIn),A
	;; Z80 memory address to start at is 0x8000 | Source,
	;; where Source is the low-order byte at address 3.
	;; Slightly tricky code creating a 3-byte 68K addr.
		LD	DE,(Source)
		SET	7,D
		LD	HL,(ByteCount)
		LD	A,$99

	;; Write bytes from memory to the FM data register.
	;; HL counts bytes remaining, DE the source address.
FMWriteLoop:	LD	(SignalOut),A

		;; Delay loop
		LD	B,$10
Delay:		DJNZ	Delay

		LD	A,(SignalIn)
		OR	A
		JP	Z,FMWriteLoop
		JP	M,L2
	;; If HL is zero, nothing to do, loop.
		LD	A,H
		OR	L
		JP	Z,FMWriteLoop
	;; *FM1Data = *DE++ + 0x80, decrement HL loop counter.
		LD	A,(DE)
		ADD	A,$80
		LD	(FM1Data),A
		INC	DE
		DEC	HL
	;; If DE != 0, loop..
		LD	A,D
		OR	E
		JP	NZ,FMWriteLoop
	;; DE is zero, we're off the end of the bank, so map
	;; in the next bank (32k/0x8000 higher)..
		LD	IX,(BankNum)
		LD	BC,$0080
		ADD	IX,BC
		LD	(BankNum),IX
		CALL	SelectBank
	;; And loop.
		JP	FMWriteLoop
