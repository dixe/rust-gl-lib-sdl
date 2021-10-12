use gl_lib_sdl as gls;
use gl_lib_sdl::{
    components::base,
    gl_lib::text_rendering::font,
    gl_lib::{gl, na, BoxCoords, ScreenBox}
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


    let sb:ScreenBox = Default::default();
    let text_coords = BoxCoords {x:0.5, y:0.5};

    while !window.should_quit() {

        window
            .text_renderer()
            .render_text(&gl, &format!("State = {}", state), text_coords, Some(sb), 1.0);


        window.update(Some((&mut container, &mut state)));

        //window.update::<()>(None);
    }

    Ok(())
}

fn setup_gui(gl: &gl::Gl) -> gls::components::container::ComponentContainer<u32> {
    let mut container = gls::components::container::ComponentContainer::new();
    let mut btn = base::button(gl);
    btn.base.set_width(200.0, 800.0);
    btn.base.set_height(200.0, 600.0);
    container.add_component(btn, button_handler);
    container
}

fn button_handler(
    event: gls::components::base::ComponentEvent,
    comp: &mut gls::components::base::Component,
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
