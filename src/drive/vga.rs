pub struct Vga {
    vga_memory: *mut u8,
    cursor: isize,
}

impl Vga {
    pub fn print_char(&mut self, ch: u8) {
        unsafe {
            *self.vga_memory.offset(self.cursor as isize * 2) = ch;
            *self.vga_memory.offset(self.cursor as isize * 2 + 1) = 0xb;
        }
        self.cursor += 1;
    }
    pub fn print(&mut self, str: &[u8]) {
        for (_, &byte) in str.iter().enumerate() {
            self.print_char(byte)
        }
    }
    pub fn print_int(&mut self, number: i64) {
        if number < 0 {
            self.print_char(b'-');
            self.print_int(-number);
        } else {
            if number < 10 {
                self.print_char(number as u8 + b'0');
            } else {
                self.print_int(number / 10);
                self.print_char((number % 10) as u8 + b'0');
            }
        }
    }
}

pub const VGA: Vga = Vga {
    vga_memory: 0xb8000 as *mut u8,
    cursor: 0,
};