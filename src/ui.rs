
use std::default::Default;
use std::collections::HashMap;

use ::gl;
use ::font;

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
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

impl Cell {
    pub fn new(ch: char, foreground: Color, background: Color) -> Cell {
        Cell {
            ch: ch,
            foreground: foreground,
            background: background,
        }
    }
}



pub struct UI {
    cells: HashMap<(u32, u32), Cell>,
    
    background_color: Color,

    font_tex: u32,
    cell_width: f32,
    cell_height: f32,
}

impl Drop for UI {
    fn drop(&mut self) {
    }
}

impl UI {
    pub fn new(window_width: u32, window_height: u32) -> Result<UI, String> {
        let tex = match font::create_texture() {
            Ok(tex) => tex,
            Err(e) => return Err(e),
        };

        let cell_width = font::get_glyph(' ').unwrap().advance as f32;
        let cell_height = font::line_height();

        Ok(UI {
            cells: HashMap::new(),
            background_color: Color::new(40, 50, 60),
            font_tex: tex,
            cell_width: cell_width,
            cell_height: cell_height,
        })
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }

    pub fn print(&mut self, x: u32, y: u32, text: &str, foreground: Color, background: Color) {
        for (i, ch) in text.chars().enumerate() {
            let mut cell = self.cells.entry((x + i as u32, y)).or_insert(Cell::default());
            cell.ch = ch;
            cell.foreground = foreground;
            cell.background = background;
        }
    }

    pub fn set(&mut self, x: u32, y: u32, cell: Cell) {
        self.cells.insert((x, y), cell);
    }

    pub fn paint(&self, window_width: u32, window_height: u32) {
        unsafe {
            let (r, g, b, a) = self.background_color.to_float();

            gl::Viewport(0, 0, window_width as i32, window_height as i32);
            gl::MatrixMode(gl::PROJECTION);
            gl::LoadIdentity();
            gl::Ortho(0.0, window_width as f64, window_height as f64, 0.0, 1.0, -1.0);

            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            gl::BindTexture(gl::TEXTURE_2D, self.font_tex);

            let max_x = (window_width as f32 / self.cell_width).floor() as u32;
            let max_y = (window_height as f32 / self.cell_height).floor() as u32;

            for (&(x, y), cell) in &self.cells {
                if x <= max_x && y <= max_y {
                    let x = x as f32 * self.cell_width;
                    let y = y as f32 * self.cell_height;

                    let glyph = font::get_glyph(cell.ch).unwrap();
                    font::draw_glyph(x, y, glyph);
                }
            }

/*
            for ch in "AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZzÅåÄäÖö0123456789".chars() {
                let glyph = font::get_glyph(ch).unwrap();
                font::draw_glyph(x, 0.0, self.font_size, glyph);
                x += glyph.advance as f32 * scale;
            }
*/

        }
    }
}