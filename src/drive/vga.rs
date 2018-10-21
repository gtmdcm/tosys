pub struct Vga {
    vga_memory: *mut u8,
    cursor: isize,
}

impl Vga {
    pub fn print_char(mut self, ch: u8) -> Vga {
        unsafe {
            *self.vga_memory.offset(self.cursor * 2) = ch;
            *self.vga_memory.offset(self.cursor * 2 + 1) = 0xb;
        }
        self.cursor += 1;
        self
    }
    pub fn print(mut self, str: &[u8]) -> Vga {
        for (_, &byte) in str.iter().enumerate() {
            self = self.print_char(byte)
        }
        self
    }
    pub fn print_uint(mut self, number: u64) -> Vga {
        if number < 10 {
            self = self.print_char(number as u8 + b'0');
        } else {
            self = self.print_uint(number / 10);
            self = self.print_char((number % 10) as u8 + b'0');
        }
        self
    }
    pub fn print_int(mut self, number: i64) -> Vga {
        if number < 0 {
            self = self.print_char(b'-');
            self = self.print_int(-number);
        } else {
            self = self.print_uint(number as u64);
        }
        self
    }
}

pub const VGA: Vga = Vga {
    vga_memory: 0xb8000 as *mut u8,
    cursor: 0,
};