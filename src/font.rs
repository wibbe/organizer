
use std::collections::HashMap;
use std::fmt;
use std::mem;

use ::gl;

static FONT_DATA: &'static [u8] = include_bytes!("../res/font.data");

include!(concat!(env!("OUT_DIR"), "/font_def.rs"));

#[derive(Copy, Clone, Debug)]
pub struct Glyph {
   pub x: f32,
   pub y: f32,
   pub width: f32,
   pub height: f32,
   pub x_offset: i32,
   pub y_offset: i32,
   pub advance: i32,
}

impl fmt::Display for Glyph {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "Glyph {{ x: {}, y: {}, width: {}, height: {} }}", self.x, self.y, self.width, self.height)
   }
}


#[inline]
pub fn get_glyph<'a>(ch: char) -> Option<&'a Glyph> {
   let idx = ch as u32;
   FONT_GLYPHS.get(&idx)
}

#[inline]
pub fn line_height() -> f32 {
   FONT_LINE_HEIGHT
}

pub fn create_texture() -> Result<u32, String> {
   let mut tex = 0;

   unsafe {
      gl::Enable(gl::TEXTURE_2D);
      gl::GenTextures(1, &mut tex);
      gl::BindTexture(gl::TEXTURE_2D, tex);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER as u32, gl::LINEAR as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER as u32, gl::LINEAR as i32);
      gl::TexImage2D(gl::TEXTURE_2D, 0, gl::ALPHA as i32, FONT_TEX_WIDTH as i32, FONT_TEX_HEIGHT as i32, 0, gl::ALPHA, gl::UNSIGNED_BYTE, mem::transmute(FONT_DATA.as_ptr()));
      
      let err = gl::GetError();
      if err != gl::NO_ERROR {
         return Err(format!("Could not create font texture, error: {}", err));
      }
      
      gl::BindTexture(gl::TEXTURE_2D, 0);
   }

   Ok(tex as u32)
}

pub fn draw_glyph(x: f32, y: f32, glyph: &Glyph) {
   let w = glyph.width * FONT_TEX_WIDTH as f32;
   let h = glyph.height * FONT_TEX_HEIGHT as f32;
   let ox = glyph.x_offset as f32;
   let oy = glyph.y_offset as f32;

   let tl = (x + ox, y + oy);
   let tr = (x + ox + w, y + oy);
   let bl = (x + ox, y + oy + h);
   let br = (x + ox + w, y + oy + h);

   unsafe {
      gl::Begin(gl::QUADS);
         gl::TexCoord2f(glyph.x, glyph.y);
         gl::Vertex2f(tl.0, tl.1);

         gl::TexCoord2f(glyph.x + glyph.width, glyph.y);
         gl::Vertex2f(tr.0, tr.1);

         gl::TexCoord2f(glyph.x + glyph.width, glyph.y + glyph.height);
         gl::Vertex2f(br.0, br.1);

         gl::TexCoord2f(glyph.x, glyph.y + glyph.height);
         gl::Vertex2f(bl.0, bl.1);
      gl::End();
   }
}