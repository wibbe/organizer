
use ::ui::*;
use ::doc::*;

use ::glutin;


pub struct App<'a> {
   ui: UI,
   window: &'a glutin::Window,

   doc: Box<Document>,
}

impl<'a> App<'a> {
   pub fn new(mut ui: UI, window: &'a glutin::Window) -> App {
      ui.print(10, 10, "Hello World", Color::new(230, 230, 230), Color::new(30, 40, 50));

      App {
         ui: ui,
         window: window,
         doc: Box::new(Document::default()),
      }
   }

   pub fn paint(&self) {
      let (w, h) = self.window.get_inner_size_pixels().unwrap();
      self.ui.paint(w, h);
   }
}