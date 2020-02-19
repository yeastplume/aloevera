.include "../common/header.inc"
.include "../common/macros.inc"

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

	set_const_16 $00, imageset

	TARGET = 512 ;loop until 512 reached

	loop:
		lda ($00),y
		sta VERA_DATA_PORT
		add_constant_16 $00, 1
		loop_till_eq_16 $00, (imageset + TARGET), loop
	rts
.endproc

.proc load_tilemap
	VAR_VERA_WRITE_ADDR = $00
	VAR_CUR_TILEMAP_ADDR = $02
	VAR_BYTES_WRITTEN_CUR_ROW = $04
	VAR_BYTES_WRITTEN_TOTAL = $06

	set_const_16 VAR_VERA_WRITE_ADDR, $0016
	set_const_16 VAR_CUR_TILEMAP_ADDR, tilemap
	set_const_16 VAR_BYTES_WRITTEN_CUR_ROW, 0
	set_const_16 VAR_BYTES_WRITTEN_TOTAL, 0

	BYTES_PER_ROW = 70
	TOTAL_BYTES = 630
	SKIP = 186

	outer_loop:
		lda $00
		sta VERA_ADDR
		lda $01
		sta VERA_ADDR + 1
		lda #$10
		sta VERA_ADDR + 2
		set_const_16 VAR_BYTES_WRITTEN_CUR_ROW, 0
		loop: 
			lda ($02),y
			sta VERA_DATA_PORT
			add_constant_16 VAR_BYTES_WRITTEN_TOTAL, 1
			add_constant_16 VAR_CUR_TILEMAP_ADDR, 1
			add_constant_16 VAR_BYTES_WRITTEN_CUR_ROW, 1
			loop_till_eq_16 VAR_BYTES_WRITTEN_CUR_ROW, BYTES_PER_ROW, loop
		add_constant_16 VAR_VERA_WRITE_ADDR, (BYTES_PER_ROW + SKIP)
		loop_till_eq_16 VAR_BYTES_WRITTEN_TOTAL, TOTAL_BYTES, outer_loop
		rts
.endproc

start:
	jsr load_tileset
	jsr load_tilemap; comment out if you just want to see the tileset result
	rts

.segment "RODATA"
imageset:
	.include "output/imagesets/text_set_1.ca65.inc"
tilemap:
	.include "output/tilemaps/tilemap_1.ca65.inc"
	; Or comment out above and comment in below to observe 256 color text mode
	;.include "output/tilemaps/tilemap_2.ca65.inc"
