.include "../common/header.inc"

VERA_ADDR = $9F20
VERA_DATA_PORT = $9F23

.code
jmp start
.proc load_tileset
	lda #$0
	sta VERA_ADDR
	lda #$F8
	sta VERA_ADDR + 1
	lda #$10
	sta VERA_ADDR + 2

	lda #<imageset
	sta $00
	lda #>imageset
	sta $01
	ldy #0

TARGET = 512 ;loop until 512 reached

	loop: 
		lda ($00),y
		sta VERA_DATA_PORT
		clc
		lda $00
		adc #1
		sta $00
		lda $01
		adc #00
		sta $01
		cmp #>(imageset + TARGET) ;loop until 512 reached
		bne loop
		lda $00
		cmp #<(imageset + TARGET)
		bne loop
	rts
.endproc

start:
	jsr load_tileset
	rts

.segment "RODATA"
imageset:
	.include "output/imagesets/text_set_1.ca65.inc"
