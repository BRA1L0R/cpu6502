pub trait DataBus {
    fn get(&self, addr: u16) -> u8;
    fn set(&mut self, addr: u16, x: u8);

    fn set_word(&mut self, offset: u16, x: u16) {
        self.set(offset, x as u8);
        self.set(offset + 1, (x >> 8) as u8);
    }

    fn get_word(&self, offset: u16) -> u16 {
        let ll = self.get(offset);
        let hh = self.get(offset + 1);

        ((hh as u16) << 8) + ll as u16
    }
}
