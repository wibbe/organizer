
use std::default::Default;
use std::vec::Vec;

use ::gl;

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}


impl Default for Color {
    fn default() -> Color {
        Color { r: 0, g: 0, b: 0, a: 255 }
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r: r, g: g, b: b, a: 255 }
    }

    #[inline]
    pub fn to_float(&self) -> (f32, f32, f32, f32) {
        (self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0, self.a as f32 / 255.0)
    }

}


struct Cell {
    ch: char,
    foreground: Color,
    background: Color,
}

impl Default for Cell {
    fn default() -> Cell {
        Cell {
            ch: ' ',
            foreground: Color::new(230, 230, 230),
            background: Color::new(0, 0, 0)
        }
    }
}


pub struct UI {
    cells: Vec<Vec<Cell>>,
    
    background_color: Color,

    font_size: u32,
    width: u32,
    height: u32,

}

impl UI {
    pub fn new(font_size: u32, window_width: u32, window_height: u32) -> UI {
        let w = window_width / font_size;
        let h = window_height / font_size;

        let mut cells = Vec::with_capacity(h as usize);
        
        for y in 0..h {
            let mut row = Vec::with_capacity(w as usize);

            for x in 0..w {
                row.push(Cell::default());
            }

            cells.push(row);
        }

        UI {
            cells: cells,
            background_color: Color::new(40, 50, 60),
            font_size: font_size,
            width: w,
            height: h,
        }
    }

    pub fn paint(&self) {
        unsafe {
            let (r, g, b, a) = self.background_color.to_float();
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}