use gl_lib_sdl as gls;
use gl_lib_sdl::{
    gl_lib::text_rendering::font,
    gl_lib::{gl, na}
};

use failure;
use std::path::Path;

fn main() -> Result<(), failure::Error> {
    let width = 800;
    let height = 600;

    let font_path = Path::new("./assets/fonts/Arial.fnt");
    let font = font::Font::load_fnt_font(font_path).unwrap();
    let mut window = gls::window::SdlGlWindow::new("Button", width, height, font).unwrap();

    let gl = &window.gl().clone();

    window.set_background_color(na::Vector4::new(0.9, 0.9, 0.9, 1.0));

    window.setup_blend();

    let mut container = setup_gui(&gl);

    let mut state = 1;


    while !window.should_quit() {
        window
            .text_renderer()
            .render_text(&gl, &format!("State = {}", state), 0.0, 0.0, 1.0);

        window.update(Some((&mut container, &mut state)));
    }

    Ok(())
}

fn setup_gui(gl: &gl::Gl) -> gls::components::container::ComponentContainer<u32> {
    let mut container = gls::components::container::ComponentContainer::new();

    let button = Box::new(gls::components::button::Button::new(gl, 0));

    container.add_component(button, button_handler);
    container
}

fn button_handler(
    event: gls::components::base::ComponentEvent,
    comp: &mut dyn gls::components::base::Component,
    state: &mut u32,
) {
    use gls::components::base::ComponentEvent;
    match event {
        ComponentEvent::Clicked => {
            *state += 1;
            comp.update_content(format!("Btn state {}", state));
        }
        _ => {}
    }
}
