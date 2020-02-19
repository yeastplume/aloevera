; adds a constant to 16 bit value
; stored at addr
.macro add_constant_16 addr, constant
	clc
	lda addr
	adc #<constant
	sta addr
	lda addr + 1
	adc #>constant
	sta addr + 1
.endmacro

; loop until value stored in addr equals constant
.macro loop_till_eq_16 addr, constant, loop_label
	lda addr+1
	cmp #>constant ;loop until 512 reached
	bne loop_label
	lda addr
	cmp #<constant
	bne loop_label
.endmacro

; set a 16 bit constant in addr
.macro set_const_16 addr, constant
	lda #<constant
	sta addr
	lda #>constant
	sta addr+1
.endmacro

VERA_ADDR_LO = $9F20
VERA_ADDR_MID = VERA_ADDR_LO + 1;
VERA_ADDR_HI = VERA_ADDR_LO + 2;
VERA_DATA0 = VERA_ADDR_LO + 3;
VERA_DATA1 = VERA_ADDR_LO + 4;

.macro v_address_set addr, stride
	lda #<(addr >> 16) | stride << 4
	sta VERA_ADDR_HI
	lda #<(addr >> 8)
	sta VERA_ADDR_MID
	lda #<(addr)
	sta VERA_ADDR_LO
.endmacro

