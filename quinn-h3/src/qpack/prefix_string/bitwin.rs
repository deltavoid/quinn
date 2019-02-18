#[derive(Debug, PartialEq, Clone)]
pub struct BitWindow {
    pub byte: u32,
    pub bit: u32,
    pub count: u32,
}

impl BitWindow {
    pub fn new() -> BitWindow {
        BitWindow {
            byte: 0,
            bit: 0,
            count: 0,
        }
    }

    pub fn forwards(&mut self, step: u32) {
        self.bit += self.count;

        self.byte += self.bit / 8;
        self.bit %= 8;

        self.count = step;
    }

    pub fn opposite_bit_window(&self) -> BitWindow {
        BitWindow {
            byte: self.byte,
            bit: self.bit,
            count: 8 - (self.bit % 8),
        }
    }
}
