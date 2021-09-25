* = $8000
start:
    LDA #1
    LSR A
end

* = $FFFC
.dsb (*-end), 0
* = $FFFC

.word start