* = $8000
reset
    LDX #$ff
    TXS

    brk
    brk
    jmp reset

break
    inx
    rti

end

* = $FFFA
.dsb (*-end), 0
* = $FFFA

.word break
.word reset