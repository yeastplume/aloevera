;Basic Header
.segment "LOADADDR"
	.word $0801

.segment "EXEHDR"
.word next, 10
.byte $9e, "2064", 0, 0
.word 0
next: .word 0
