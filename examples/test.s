    .byte $00
    ; padding

    .org $3000
start:
    lda #$FC
    pha
    lda #$BC
    pla

    .org $fffc
reset:
    .word $0030