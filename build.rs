
use std::char;
use std::env;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, LineWriter};
use std::path::Path;


fn main() {
   let out_dir = env::var("OUT_DIR").unwrap();
   let out_path = Path::new(&out_dir).join("font_def.rs");
   let in_path = Path::new("res/font.fnt");

   let out = File::create(&out_path).unwrap();
   let mut out = LineWriter::new(out);
   let reader = BufReader::new(File::open(&in_path).unwrap());

   println!("cargo:rerun-if-changed={}", in_path.display());

   out.write(b"lazy_static! {\n");
   out.write(b"   static ref GLYPHS: HashMap<u32, Glyph> = {\n");
   out.write(b"      let mut m = HashMap::new();\n");

   for line in reader.lines() {
      let line = line.unwrap();
      let parts = line.split_whitespace().map(|x| String::from(x)).collect::<Vec<String>>();
      
      if parts.len() == 11 && parts[0] == "char" {
         let mut ch: u32 = 0;
         let mut x: i32 = -1;
         let mut y: i32 = -1;
         let mut xo: i32 = -1;
         let mut yo: i32 = -1;
         let mut w: i32 = -1;
         let mut h: i32 = -1;
         let mut a: i32 = 0;

         for part in parts.iter() {
            let el = part.split('=').collect::<Vec<_>>();
            if el.len() == 2 {
               match el[0] {
                  "id" => { ch = el[1].parse().unwrap_or(0); },
                  "x" => { x = el[1].parse().unwrap_or(-1); },
                  "y" => { y = el[1].parse().unwrap_or(-1); },
                  "width" => { w = el[1].parse().unwrap_or(-1); },
                  "height" => { h = el[1].parse().unwrap_or(-1); },
                  "xoffset" => { xo = el[1].parse().unwrap_or(-1); },
                  "yoffset" => { yo = el[1].parse().unwrap_or(-1); },
                  "xadvance" => { a = el[1].parse().unwrap_or(-1); },
                  _ => ()
               }
            }
         }

         if ch != 0 && x != -1 && y != -1 && xo != -1 && yo != -1 && w != -1 && h != -1 && a != -1 {
            writeln!(out, "      m.insert({:4}, Glyph {{ x: {:3}, y: {:3}, width: {:3}, height: {:3}, x_offset: {:3}, y_offset: {:3}, advance: {:3} }});     // Char({})", ch, x, y, w, h, xo, yo, a, char::from_u32(ch).unwrap());
         }
      }
   }

   out.write(b"      m\n");
   out.write(b"   };\n");
   out.write(b"}\n");
}