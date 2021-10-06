use gl_lib_sdl as gls;
use gl_lib_sdl::{gl_lib::{gl, na}, gl_lib::text_rendering::font};

use failure;
use std::path::Path;

fn main() -> Result<(), failure::Error> {


    let width = 800;
    let height = 600;

    let font_path = Path::new("./assets/fonts/Arial.fnt");
    let font = font::Font::load_fnt_font(font_path).unwrap();
    let mut window = gls::window::SdlGlWindow::new("Text window", width, height, font).unwrap();


    let gl = &window.gl().clone();


    window.set_background_color(na::Vector4::new(0.9, 0.9, 0.9, 1.0));

    window.setup_blend();

    let mut container = setup_gui();

    let mut state = 1;
    while !window.should_quit() {

        unsafe {
            window.gl().Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        window.text_renderer().render_text(&gl, &format!("State = {}", state), 0.0, 0.0, 1.0);

        container.handle_events();
        window.gl_swap_window_and_update(Some(&mut container));
    }

    Ok(())

}


fn setup_gui() -> gls::components::container::ComponentContainer {

    let mut container = gls::components::container::ComponentContainer::new();

    let button = Box::new(gls::components::button::Button::new(0));

    container.add_component(button);
    container

}
