start   lda #$FC
        sta $ABCD
repeat  pha
        jmp (repeat)

