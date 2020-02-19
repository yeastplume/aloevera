.include "../common/header.inc"

VERA_ADDR = $9F20
VERA_DATA_PORT = $9F23

.code
jmp start
.proc load_palette
	lda #$0
	sta VERA_ADDR
	lda #$10
	sta VERA_ADDR + 1
	lda #$1F
	sta VERA_ADDR + 2
	ldx #0
	loop:
		lda palette,x
		sta VERA_DATA_PORT
		inx
		cpx #112
		bne loop
	end:
		rts
.endproc

start:
	jsr load_palette
	rts

.segment "RODATA"
palette:
	.include "output/palettes/palette_1.ca65.inc"
