.include "../common/header.inc"
.include "../common/macros.inc"

.code
jmp start
.proc set_mode ;target layer 0
	lda #$13	; layer 0, rgb output
	sta $9F29
	;set layer 0 mode to tiled Bitmap Mode 8bpp
	lda #$07	; 8bpp bitmap
	sta $9F2D
	lda #$0		; map base not used
	sta $9F2E
	lda #$0		; tileset address to $00000, tilewidth = 0 (320)
	sta $9F2F
	sta $9F30	; hscroll=0, palette base = 0
	sta $9F31	
	sta $9F32	; vscroll=0
	sta $9F33
	lda #$40	; hscale, vscale
	sta $9F2A
	sta $9F2B
	rts
.endproc

.proc load_palette
	v_address_set $1FA00, 1
	set_const_16 $00, palette

	TARGET = 512;loop until size reached

	loop:
		lda ($00),y
		sta VERA_DATA0
		add_constant_16 $00, 1
		loop_till_eq_16 $00, (palette + TARGET), loop
	rts
.endproc

.code
jmp start
.proc load_bitmap
	v_address_set $00000, 1
	set_const_16 $00, bitmap
	TARGET = 32000;loop until size reached

	loop:
		lda ($00),y
		sta VERA_DATA0
		add_constant_16 $00, 1
		loop_till_eq_16 $00, (bitmap + TARGET), loop
	rts
.endproc

start:
	jsr set_mode
	jsr load_palette
	jsr load_bitmap
	;jsr clear_map
	;jsr load_tilemap
	lda #$13	; layer 0, rgb output
	sta $9F29
rts

.segment "RODATA"
palette:
	.include "output/palettes/kq5_pal.ca65.inc"
bitmap:
	.include "output/bitmaps/kq5_bmp.ca65.inc"
