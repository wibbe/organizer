
extern crate gl;
extern crate glutin;
extern crate libc;

mod ui;
use ui::*;

fn main() {

    let window = glutin::WindowBuilder::new()
                                        .with_title("Organizer")
                                        .with_dimensions(800, 500)
                                        .with_visibility(true)
                                        .build().unwrap();

    unsafe { window.make_current(); }

    unsafe {
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    }

    let mut ui = UI::new(14, 800, 500);

    ui.paint();
    window.swap_buffers();

    for event in window.wait_events() {
        ui.paint();
        window.swap_buffers();

        match event {
            glutin::Event::Closed => break,
            _ => ()
        }
    }
}
