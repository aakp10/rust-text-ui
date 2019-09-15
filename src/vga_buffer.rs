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
pub struct VGA_color_scheme(u8);    

impl VGA_color_scheme {
    //new function
    fn new(bg: Colors, fg: Colors) -> VGA_color_scheme {
        VGA_color_scheme ((bg as u8) << 4 | (fg as u8))
    }
}

#[derive(Copy, Clone)]
// screen character
#[repr(C)]
struct Screen_char {
    data: u8,
    color: VGA_color_scheme
}

impl Screen_char {
    fn new(data: u8, color: VGA_color_scheme) -> Screen_char {
        Screen_char {
            data,
            color,
        }
    }
}

const MAX_ROW: usize = 25;
const MAX_COL: usize = 80;
// buffer 25 * 80 of [Screen_chars]
//#[repr(transparent)]
struct VGA_buffer {
    current_row: usize,
    current_col: usize,
    color: VGA_color_scheme,
    buffer: [[Screen_char; MAX_COL]; MAX_ROW]
}

impl VGA_buffer {
    pub fn write_byte(&mut self, character: u8) {
        match character {
            b'\n' => self.insert_new_line(),
            0x20..=0x7e => { //valid ascii character
                self.buffer[self.current_row][self.current_col] = Screen_char::new(character, self.color);
                self.update_cursor();
            }
            _ => {
                let vga_char = Screen_char {
                    data: character,
                    color: self.color
                };
                self.buffer[self.current_row][self.current_col] = Screen_char::new(0xfe, self.color);
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

    }
}

