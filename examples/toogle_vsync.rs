use gl_lib_sdl as gls;
use gl_lib_sdl::{
    components::base,
    gl_lib::text_rendering::font,
    gl_lib::{gl, na, ScreenBox}
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

    let mut state = true;

    let sb = ScreenBox::full_screen(width as f32, height as f32);

    while !window.should_quit() {

        let time_ms =  1.0 / window.deltatime();
        window.text_renderer().render_text(&gl, &format!("Fps = {}", time_ms), Default::default(), sb, 1.0);

        window.update(Some((&mut container, &mut state)));
    }

    Ok(())
}

fn setup_gui(gl: &gl::Gl) -> gls::components::container::ComponentContainer<bool> {
    let mut container = gls::components::container::ComponentContainer::new();
    let mut btn = base::button(gl);

    btn.update_content("Toogle".to_string());
    btn.base.set_width(200.0, 800.0);
    btn.base.set_height(200.0, 600.0);
    container.add_component(btn, button_handler);
    container
}

fn button_handler(
    event: gls::components::base::ComponentEvent,
    comp: &mut gls::components::base::Component,
    state: &mut bool,
    window_access: &gls::window::WindowComponentAccess,
) {
    use gls::components::base::ComponentEvent;
    match event {
        ComponentEvent::Clicked => {
            *state = !*state;
            println!("{:?}", state);
            match state {
                false => {
                    window_access.set_swap_interval(0);
                },
                true => {
                    window_access.enable_vsync();
                }
            }

        }
        _ => {}
    }
}
