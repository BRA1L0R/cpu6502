* = $0
reset
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