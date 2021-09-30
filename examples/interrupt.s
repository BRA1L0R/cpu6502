* = $8000
reset
    ; initialize the stack
    LDX #$ff
    TXS 

    ; break interrupt
    brk
    ; break reason
    .byt $00

    .byt $22

break
    inx
    rti
    
end

* = $FFFA
.dsb (*-end), 0
* = $FFFA

.word break
.word reset