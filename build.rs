
extern crate gl_generator;

use std::char;
use std::env;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, LineWriter};
use std::path::Path;

use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator};

fn parse_options(options: Vec<String>) -> Vec<(String, i32)> {
   options.iter().fold(Vec::new(), |mut acc, ref x| {
      let option = x.split('=').collect::<Vec<&str>>();
      
      if option.len() == 2 {
         acc.push((String::from(option[0]), option[1].parse::<i32>().unwrap_or(0)));
      }

      acc
   })
}

fn find_option(name: &str, options: &Vec<(String, i32)>) -> Option<i32> {
   options.iter().find(|ref el| el.0.as_str() == name).map(|ref el| el.1)
}

fn generate_font_def() {
   let out_dir = env::var("OUT_DIR").unwrap();
   let out_path = Path::new(&out_dir).join("font_def.rs");
   let in_path = Path::new("res/font.fnt");

   let out = File::create(&out_path).unwrap();
   let mut out = LineWriter::new(out);
   let reader = BufReader::new(File::open(&in_path).unwrap());

   let mut tex_w = 0;
   let mut tex_h = 0;
   let mut line_height = 0;
   let mut base_line = 0;
   let mut chars = Vec::new();
   let mut size = 0;

   for line in reader.lines() {
      let line = line.unwrap();

      let mut line_parts = line.split_whitespace().map(|x| String::from(x)).collect::<Vec<String>>();

      match line_parts[0].as_str() {
         "common" => {
            let options = parse_options(line_parts);
            tex_w = find_option("scaleW", &options).unwrap();
            tex_h = find_option("scaleH", &options).unwrap();
            line_height = find_option("lineHeight", &options).unwrap();
            base_line = find_option("base", &options).unwrap();
         },

         "info" => {
            let options = parse_options(line_parts);
            size = find_option("size", &options).unwrap().abs();
         }
         
         "char" => {
            let options = parse_options(line_parts);
            chars.push((
               find_option("id", &options).unwrap(),
               find_option("x", &options).unwrap(),
               find_option("y", &options).unwrap(),
               find_option("width", &options).unwrap(),
               find_option("height", &options).unwrap(),
               find_option("xoffset", &options).unwrap(),
               find_option("yoffset", &options).unwrap(),
               find_option("xadvance", &options).unwrap()
            ));
         },

         _ => (),
      }
   }

   println!("cargo:rerun-if-changed={}", in_path.display());

   // Generate output file
   writeln!(out, "pub static FONT_TEX_WIDTH: u32 = {};", tex_w);
   writeln!(out, "pub static FONT_TEX_HEIGHT: u32 = {};", tex_h);
   writeln!(out, "pub static FONT_LINE_HEIGHT: f32 = {:.1};", line_height as f32);
   writeln!(out, "pub static FONT_BASE: f32 = {:.1};", base_line as f32);
   writeln!(out, "pub static FONT_SIZE: f32 = {:.1};", size as f32);
   out.write(b"\n").unwrap();
   out.write(b"lazy_static! {\n").unwrap();
   out.write(b"   static ref FONT_GLYPHS: HashMap<u32, Glyph> = {\n").unwrap();
   out.write(b"      let mut m = HashMap::new();\n").unwrap();

   let tw = tex_w as f32;
   let th = tex_h as f32;

   for (ch, x, y, w, h, xo, yo, a) in chars {
      if ch == 0 || ch == 13 {
         continue;
      }

      writeln!(out, "      m.insert({:4}, Glyph {{ x: {:.6}, y: {:.6}, width: {:.6}, height: {:.6}, x_offset: {:2}, y_offset: {:2}, advance: {:2} }});     // Char({})", 
         ch,
         x as f32 / tw,
         y as f32 / th,
         w as f32 / tw,
         h as f32 / th,
         xo, yo, a, char::from_u32(ch as u32).unwrap());
   }

   out.write(b"      m\n").unwrap();
   out.write(b"   };\n").unwrap();
   out.write(b"}\n").unwrap();
}

fn generate_gl_bindings() {
    let dest = env::var("OUT_DIR").unwrap();
    let path = Path::new(&dest).join("gl_bindings.rs");
    let mut file = File::create(&path).unwrap();

    Registry::new(Api::Gl, (1, 1), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();
}

fn main() {
   generate_font_def();
   generate_gl_bindings();
}
