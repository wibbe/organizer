
use std::collections::HashMap;
use std::fmt;
use std::mem;

use ::lazy_static;
use ::gl;

static FONT_DATA: &'static [u8] = include_bytes!("../res/font.data");

include!(concat!(env!("OUT_DIR"), "/font_def.rs"));

#[derive(Copy, Clone, Debug)]
pub struct Glyph {
   x: u32,
   y: u32,
   width: u32,
   height: u32,
   x_offset: u32,
   y_offset: u32,
   advance: u32,
}

impl fmt::Display for Glyph {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "Glyph {{ x: {}, y: {}, width: {}, height: {} }}", self.x, self.y, self.width, self.height)
   }
}


pub fn get_glyph(ch: char) -> Option<Glyph> {
   let idx = ch as u32;
   GLYPHS.get(&idx).cloned()
}

pub fn create_texture() -> Result<u32, String> {
   let mut tex = 0;

   unsafe {
      gl::Enable(gl::TEXTURE_2D);
      gl::GenTextures(1, &mut tex);
      gl::BindTexture(gl::TEXTURE_2D, tex);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER as u32, gl::LINEAR as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER as u32, gl::LINEAR as i32);
      gl::TexImage2D(gl::TEXTURE_2D, 0, gl::LUMINANCE as i32, 512, 512, 0, gl::LUMINANCE, gl::UNSIGNED_BYTE, mem::transmute(FONT_DATA.as_ptr()));
      
      let err = gl::GetError();
      if err != gl::NO_ERROR {
         return Err(format!("Could not create font texture, error: {}", err));
      }
      
      gl::BindTexture(gl::TEXTURE_2D, 0);
   }

   Ok(tex as u32)
}