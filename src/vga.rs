use core::fmt;
use spin::Mutex;
use volatile::Volatile;

lazy_static! {
    pub static ref VGA: Mutex<Vga> = Mutex::new(Vga {
        cursor_x: 0,
        cursor_y: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 25;

struct Buffer {
    chars: [[Volatile<ScreenChar>; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

pub struct Vga {
    cursor_x: usize,
    cursor_y: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Vga {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.cursor_x >= SCREEN_WIDTH {
                    self.new_line();
                }
                let color_code = self.color_code;
                self.buffer.chars[self.cursor_y][self.cursor_x].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.cursor_x += 1;
            }
        }
    }

    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(b' '),
            }
        }
    }

    fn new_line(&mut self) {
        self.cursor_y += 1;
        self.cursor_x = 0;
        if self.cursor_y == SCREEN_WIDTH {
            self.cls();
        }
    }

    fn cls(&mut self) {
        self.cursor_y = 0;
        self.cursor_x = 0;
        for _ in 0..SCREEN_WIDTH * SCREEN_HEIGHT {
            self.write_byte(b' ');
        }
        self.cursor_y = 0;
        self.cursor_x = 0;
    }
}

impl fmt::Write for Vga {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        VGA.lock().write_fmt(args).unwrap();
    });
}