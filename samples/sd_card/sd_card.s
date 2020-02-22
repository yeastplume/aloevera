.include "../common/header.inc"
.include "../common/macros.inc"

.code
jmp start
.proc set_mode ;target layer 1
	;set layer 1 mode to tiled Bitmap Mode 8bpp
	v_address_set $F2000, 1
	lda #$E1 ;8bpp bitmap, on
	;lda #$81 ;tiled, 8bpp, on
	sta VERA_DATA0
	lda #$0 ;TileW is 320
	sta VERA_DATA0
	;map base not used
	lda #$00
	sta VERA_DATA0
	lda #$00
	sta VERA_DATA0
	;set tileset (bmp data) address to $00000
	lda #$00
	sta VERA_DATA0
	lda #$00
	sta VERA_DATA0
	lda #$00 
	sta VERA_DATA0 
	lda #$00
	sta VERA_DATA0 ; palette offset 0
	;set hscale, vscale
	v_address_set $F0001, 1
	lda #$40
	sta VERA_DATA0
	lda #$40
	sta VERA_DATA0
	rts
.endproc

;.proc load_palette
;	v_address_set $F1000, 1
;	set_const_16 $00, palette

;	TARGET = 512;loop until size reached

;	loop:
;		lda ($00),y
;		sta VERA_DATA0
;		add_constant_16 $00, 1
;		loop_till_eq_16 $00, (palette + TARGET), loop
;	rts
;.endproc

;.code
;jmp start
;.proc load_bitmap
;	v_address_set $00000, 1
;	set_const_16 $00, bitmap
;	TARGET = 32000;loop until size reached
;
;	loop:
;		lda ($00),y
;		sta VERA_DATA0
;		add_constant_16 $00, 1
;		loop_till_eq_16 $00, (bitmap + TARGET), loop
;	rts
;.endproc

start:
	;jsr set_mode
;	jsr load_palette
;	jsr load_bitmap
	;jsr clear_map
	;jsr load_tilemap
	;turn off layer 2 to see our handiwork
;	v_address_set $F3000, 0
;	lda #$0 ;default, off
;	sta VERA_DATA0
rts
