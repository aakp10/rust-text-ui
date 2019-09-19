//https://wiki.osdev.org/Text_UI
use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;

const MAX_ROW: usize = 25;
const MAX_COL: usize = 80;

#[allow(dead_code)]

#[derive(Copy, Clone)]
#[repr(u8)]         //since each color should be represented as a byte
pub enum Colors {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
}

#[derive(Copy, Clone)]
//wrapper to represent {bg, brightness, fg}
pub struct VgaColorScheme(u8);    

impl VgaColorScheme {
    //new function
    pub fn new(bg: Colors, fg: Colors) -> VgaColorScheme {
        VgaColorScheme ((bg as u8) << 4 | (fg as u8))
    }
}

#[derive(Copy, Clone)]
// screen character
#[repr(C)]
pub struct ScreenChar {
    data: u8,
    color: VgaColorScheme
}

impl ScreenChar {
    pub fn new(data: u8, color: VgaColorScheme) -> ScreenChar {
        ScreenChar {
            data,
            color,
        }
    }
}

// buffer 25 * 80 of [Screen_chars]
#[repr(transparent)]
pub struct BufferMemory{
    chars: [[Volatile<ScreenChar>; MAX_COL]; MAX_ROW],
}

pub struct VgaBuffer {
    current_row: usize,
    current_col: usize,
    color: VgaColorScheme,
    buffer: &'static mut BufferMemory, //this can be replaced by the array itself.
}

impl VgaBuffer {
    pub fn write_byte(&mut self, character: u8) {
        match character {
            b'\n' => self.insert_new_line(),
            0x20..=0x7e => { //valid ascii character
                self.buffer.chars[self.current_row][self.current_col].write(ScreenChar::new(character, self.color));
                self.update_cursor();
            }
            _ => {
                self.buffer.chars[self.current_row][self.current_col].write(ScreenChar::new(0xfe, self.color));
                self.update_cursor();
            }
        }
    }

    fn update_cursor(&mut self) {
        self.current_col = (self.current_col + 1) % MAX_COL;
        if self.current_col == 0 {
            self.current_row += 1;
        }
    }

    fn insert_new_line(&mut self) {
        self.current_row += 1;
    }

    pub fn write_string(&mut self, message: &str) {
        for char in message.bytes() {
            self.write_byte(char);
        }
    }
}

lazy_static!{
    pub static ref VGA_WRITER: Mutex<VgaBuffer> = Mutex::new(VgaBuffer {
        current_row: 0,
        current_col: 0,
        color: VgaColorScheme::new(Colors::Black, Colors::Cyan),
        buffer: unsafe{ &mut *(0xB8000 as *mut BufferMemory) },        
    });
}

/*
pub fn print_something(message: &str) {
    //get color code
    let color_code = VGA_color_scheme::new(Colors::Black, Colors::Cyan);
    //get a VGA buffer instance
    let mut buffer = VGA_buffer {
        current_row: 0,
        current_col: 0,
        color: color_code,
        buffer: unsafe{ &mut *(0xB8000 as *mut buffer_memory) },        
    };
    buffer.write_string(message);   
}
*/