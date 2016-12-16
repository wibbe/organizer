
use std::default::Default;
use std::collections::HashMap;

use ::gl;
use ::font;

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
    cells: HashMap<(u32, u32), Cell>,
    
    background_color: Color,

    font_tex: u32,    
    font_size: u32,
    width: u32,
    height: u32,
}

impl Drop for UI {
    fn drop(&mut self) {
    }
}

impl UI {
    pub fn new(font_size: u32, window_width: u32, window_height: u32) -> Result<UI, String> {
        let w = window_width / font_size;
        let h = window_height / font_size;

        let tex = match font::create_texture() {
            Ok(tex) => tex,
            Err(e) => return Err(e),
        };

        Ok(UI {
            cells: HashMap::new(),
            background_color: Color::new(40, 50, 60),
            font_tex: tex,
            font_size: font_size,
            width: w,
            height: h,
        })
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

            gl::BindTexture(gl::TEXTURE_2D, self.font_tex);
            gl::Begin(gl::QUADS);
                gl::TexCoord2f(0.0, 0.0); gl::Vertex2f(0.0, 0.0);
                gl::TexCoord2f(1.0, 0.0); gl::Vertex2f(window_width as f32, 0.0);
                gl::TexCoord2f(1.0, 1.0); gl::Vertex2f(window_width as f32, window_height as f32);
                gl::TexCoord2f(0.0, 1.0); gl::Vertex2f(0.0, window_height as f32);
            gl::End();
        }
    }
}