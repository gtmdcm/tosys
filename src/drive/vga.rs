const WIDTH: isize = 80;

pub struct Vga {
    vga_memory: *mut u8,
    cursor_x: isize,
    cursor_y: isize,
}

impl Vga {
    pub fn new() -> Vga {
        Vga {
            vga_memory: 0xb8000 as *mut u8,
            cursor_x: 0,
            cursor_y: 0,
        }
    }
    pub fn print_char(&mut self, ch: u8) -> &mut Vga {
        if ch == b'\n' {
            self.println(b"");
        } else {
            unsafe {
                *self.vga_memory.offset((self.cursor_y * WIDTH + self.cursor_x) * 2) = ch;
                *self.vga_memory.offset((self.cursor_y * WIDTH + self.cursor_x) * 2 + 1) = 0xb;
            }
            self.cursor_x += 1;
            if self.cursor_x == 80 {
                self.cursor_x = 0;
                self.cursor_y += 1;
            }
        }
        self
    }
    pub fn print(&mut self, str: &[u8]) -> &mut Vga {
        for (_, &byte) in str.iter().enumerate() {
            self.print_char(byte);
        }
        self
    }
    pub fn println(&mut self, str: &[u8]) -> &mut Vga {
        self.print(str);
        self.cursor_x = 0;
        self.cursor_y += 1;
        self
    }
    pub fn print_uint(&mut self, number: u64) -> &mut Vga {
        if number < 10 {
            self.print_char(number as u8 + b'0');
        } else {
            self.print_uint(number / 10);
            self.print_char((number % 10) as u8 + b'0');
        }
        self
    }
    pub fn print_int(&mut self, number: i64) -> &mut Vga {
        if number < 0 {
            self.print_char(b'-');
            self.print_int(-number);
        } else {
            self.print_uint(number as u64);
        }
        self
    }
}
