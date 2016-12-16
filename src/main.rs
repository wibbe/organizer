
extern crate glutin;
extern crate libc;

#[macro_use]
extern crate lazy_static;

mod gl;
mod ui;
mod font;

use ui::*;

fn main() {

    let window = glutin::WindowBuilder::new()
                                        .with_title("Organizer")
                                        .with_dimensions(800, 500)
                                        .with_visibility(true)
                                        .build().unwrap();

    unsafe { window.make_current().unwrap(); }

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut ui = UI::new(14, 800, 500).unwrap();

    //ui.paint();
    //window.swap_buffers().unwrap();

    println!("Glyph 'p': {}", font::get_glyph('p').unwrap());

    for event in window.wait_events() {
        let (w, h) = window.get_inner_size_pixels().unwrap();

        ui.paint(w, h);
        window.swap_buffers().unwrap();

        match event {
            glutin::Event::Closed => break,
            _ => ()
        }
    }
}
