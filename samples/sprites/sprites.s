.include "../common/header.inc"
.include "../common/macros.inc"

.code
jmp start

.proc load_palette
	v_address_set $1FA00, 1
	ldx #0
	loop:
		lda palette,x
		sta VERA_DATA0
		inx
		cpx #24
		bne loop
	end:
		rts
.endproc

.code
jmp start
.proc load_sprite
	v_address_set $1A800, 1
	set_const_16 $00, sprite

	TARGET = 768 ;loop until size reached

	loop:
		lda ($00),y
		sta VERA_DATA0
		add_constant_16 $00, 1
		loop_till_eq_16 $00, (sprite + TARGET), loop
	rts
.endproc

;Set up our sprite
.proc sprite_setup
	v_address_set $1FC00, 1
	lda #$40
	sta VERA_DATA0
	lda #$D ; Mode 0 - 4BPP
	sta VERA_DATA0
	lda #30 ; X Pos 0 - Low bits
	sta VERA_DATA0
	lda #01 ; X Pos 0 - High bits
	sta VERA_DATA0
	lda #0 ; Y Pos 0 - Low bits
	sta VERA_DATA0
	lda #0 ; Y Pos 0 - High bits
	sta VERA_DATA0
	lda #$C ; Z Depth 3 - in front of layer 1
	sta VERA_DATA0
	lda #$90 ; Height 32, width 16, Pal offset 0
	sta VERA_DATA0
	rts
.endproc

;Small delay loop between frames
.proc delay
	ldx #$02
	lda #$ff
	ldy #$ff
	loop:
		cpy #1 ; 2 cycles
		dey    ; 2
		sbc #0 ; 2
		bcs loop ; 3
	dex
	cpx #0
	bne loop
	rts
.endproc

;little loop to run through terra's walk-cycle
.proc main_loop
	loop:
		v_address_set $1fc04, 0 ; Move down the screen a bit
		inc VERA_DATA0
		inc VERA_DATA0
		v_address_set $1fc00, 0
		lda #$40
		sta VERA_DATA0
		jsr delay
		lda #$48
		sta VERA_DATA0
		jsr delay
		v_address_set $1fc04, 0 ; Move down the screen a bit
		inc VERA_DATA0
		inc VERA_DATA0
		v_address_set $1fc00, 0
		lda #$50
		sta VERA_DATA0
		jsr delay
		lda #$48
		sta VERA_DATA0
		jsr delay
	jmp loop
.endproc

start:
	jsr load_palette
	jsr load_sprite
	jsr sprite_setup
	jsr delay
	lda #$43	; sprites, rgb output
	sta $9F29
	v_address_set $F3000, 0
	jsr main_loop
rts

.segment "RODATA"
palette:
	.include "output/palettes/terra_pal.ca65.inc"
sprite:
	.include "output/sprites/terra_sprite.ca65.inc"
