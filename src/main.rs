
extern crate glutin;
extern crate libc;

#[macro_use]
extern crate lazy_static;

mod gl;
mod ui;
mod font;
mod app;
mod doc;

use ui::*;
use app::*;

fn main() {

    let window = glutin::WindowBuilder::new()
                                        .with_title("Organizer")
                                        .with_dimensions(800, 500)
                                        .with_visibility(true)
                                        .build().unwrap();

    unsafe { window.make_current().unwrap(); }

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let ui = UI::new(800, 500).unwrap();
    let mut app = App::new(ui, &window);

    let background = Color::new(40, 50, 60);
    let foreground = Color::new(230, 230, 230);
    
    app.paint();
    window.swap_buffers().unwrap();

    for event in window.wait_events() {
        match event {
            glutin::Event::Closed => break,
            glutin::Event::ReceivedCharacter(ch) => println!("Wrote char: {}", ch),
            glutin::Event::Refresh => (),
            _ => ()
        }

        app.paint();
        window.swap_buffers().unwrap();
    }
}
