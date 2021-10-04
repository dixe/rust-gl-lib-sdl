use gl_lib::{gl, na};
use gl_lib::text_rendering::{text_renderer, font};
use gl_lib_sdl as gls;
use failure;
use std::path::Path;

fn main() -> Result<(), failure::Error> {


    let width = 800;
    let height = 600;

    let mut window = gls::window::SdlGlWindow::new("Text window", width, height).unwrap();

    let gl = &window.gl().clone();


    window.set_background_color(na::Vector4::new(0.9, 0.9, 0.9, 1.0));

    let font_path = Path::new("./assets/fonts/Arial.fnt");
    let font = font::Font::load_fnt_font(font_path).unwrap();
    let mut text_renderer = text_renderer::TextRenderer::new(&gl, font);

    text_renderer.setup_blend(window.gl());


    loop {

        unsafe {
            window.gl().Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        text_renderer.render_text(&gl, &format!("Fps = {} ms", window.deltatime() * 1000.0 ), -0.0, 0.0, 1.0);

        window.gl_swap_window_and_update();

    }

}
