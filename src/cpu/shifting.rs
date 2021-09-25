pub fn rotate_left(entering: bool, mut x: u8) -> (u8, bool) {
    let carry_out = x & 0b1000_0000 != 0;
    x <<= 1;
    x |= entering as u8;

    (x, carry_out)
}

pub fn rotate_right(entering: bool, mut x: u8) -> (u8, bool) {
    let carry_out = x & 0b0000_0001 != 0;
    x >>= 1;
    x |= (entering as u8) << 7;

    (x, carry_out)
}
