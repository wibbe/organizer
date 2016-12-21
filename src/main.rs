
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

    let ui = UI::new(14, 800, 500).unwrap();
    let mut app = App::new(ui, &window);

    let background = Color::new(40, 50, 60);
    let foreground = Color::new(230, 230, 230);
/*
    for i in 0..10 {
        ui.print(i, i, format!("{}", i).as_str(), foreground, background);
    }

    ui.print(10, 10, "Hello World", foreground, background);
*/
    //ui.paint();
    
    app.paint();
    window.swap_buffers().unwrap();


    for event in window.wait_events() {
        //ui.paint(w, h);
        app.paint();
        window.swap_buffers().unwrap();

        match event {
            glutin::Event::Closed => break,
            _ => ()
        }
    }
}
