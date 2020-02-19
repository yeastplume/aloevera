.include "../common/header.inc"
.include "../common/macros.inc"

.code
jmp start
.proc set_mode ;target layer 1
	;set layer 1 mode to tiled 4bpp, 64x32 
	v_address_set $F2000, 1
	lda #$61 ;tiled, 4bpp, on
	;lda #$81 ;tiled, 8bpp, on
	sta VERA_DATA0
	lda #$31 ;16x16 tile, 64x32 map width/height
	sta VERA_DATA0
	;set tilemap address to $10000
	lda #$00
	sta VERA_DATA0
	lda #$40
	sta VERA_DATA0
	;set tileset (imageset) address to $1A800
	lda #$00
	sta VERA_DATA0
	lda #$6A
	sta VERA_DATA0
	rts
.endproc

.proc load_palette
	v_address_set $F1000, 1
	ldx #0
	loop:
		lda palette,x
		sta VERA_DATA0
		inx
		cpx #30
		bne loop
	end:
		rts
.endproc

.code
jmp start
.proc load_imageset
	v_address_set $1A800, 1
	set_const_16 $00, imageset

	TARGET = 2176 ;loop until size reached

	loop:
		lda ($00),y
		sta VERA_DATA0
		add_constant_16 $00, 1
		loop_till_eq_16 $00, (imageset + TARGET), loop
	rts
.endproc

;Set tilemap to a blank tile
.proc clear_map
	VAR_ENTRIES_WRITTEN = $00
	set_const_16 VAR_ENTRIES_WRITTEN, 0
	v_address_set $10000, 1
	loop:
		;index at 0 is an empty tile in our map
		lda #0
		sta VERA_DATA0
		lda #0
		sta VERA_DATA0
		add_constant_16 VAR_ENTRIES_WRITTEN, 1
		loop_till_eq_16 VAR_ENTRIES_WRITTEN, 2048, loop
	rts
.endproc

.proc load_tilemap
	VAR_VERA_WRITE_ADDR = $00
	VAR_CUR_TILEMAP_ADDR = $02
	VAR_BYTES_WRITTEN_CUR_ROW = $04
	VAR_BYTES_WRITTEN_TOTAL = $06

	set_const_16 VAR_VERA_WRITE_ADDR, $500
	set_const_16 VAR_CUR_TILEMAP_ADDR, tilemap
	set_const_16 VAR_BYTES_WRITTEN_CUR_ROW, 0
	set_const_16 VAR_BYTES_WRITTEN_TOTAL, 0

	BYTES_PER_ROW = 46
	TOTAL_BYTES = 414
	SKIP = 82

	outer_loop:
		lda $00
		sta VERA_ADDR_LO
		lda $01
		sta VERA_ADDR_MID
		lda #$11
		sta VERA_ADDR_HI
		set_const_16 VAR_BYTES_WRITTEN_CUR_ROW, 0
		loop: 
			lda ($02),y
			sta VERA_DATA0
			add_constant_16 VAR_BYTES_WRITTEN_TOTAL, 1
			add_constant_16 VAR_CUR_TILEMAP_ADDR, 1
			add_constant_16 VAR_BYTES_WRITTEN_CUR_ROW, 1
			loop_till_eq_16 VAR_BYTES_WRITTEN_CUR_ROW, BYTES_PER_ROW, loop
		add_constant_16 VAR_VERA_WRITE_ADDR, (BYTES_PER_ROW + SKIP)
		loop_till_eq_16 VAR_BYTES_WRITTEN_TOTAL, TOTAL_BYTES, outer_loop
		rts
.endproc

start:
	jsr set_mode
	jsr load_palette
	jsr load_imageset
	jsr clear_map
	jsr load_tilemap
	;turn off layer 2 to see our handiwork
	v_address_set $F3000, 0
	lda #$0 ;default, off
	sta VERA_DATA0
rts

.segment "RODATA"
palette:
	.include "output/palettes/tile_wall_pal.ca65.inc"
imageset:
	.include "output/imagesets/wall_tiles.ca65.inc"
tilemap:
	.include "output/tilemaps/wall_tilemap.ca65.inc"
