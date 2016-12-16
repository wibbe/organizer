
use std::collections::HashMap;
use std::fmt;

use ::lazy_static;

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