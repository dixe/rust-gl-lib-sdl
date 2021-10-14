use gl_lib::*;
use gl_lib::text_rendering::{font};
use gl_lib_sdl as gls;
use failure;
use std::path::Path;

fn main() -> Result<(), failure::Error> {


    let width = 800;
    let height = 600;

    let font_path = Path::new("./assets/fonts/Arial.fnt");
    let font = font::Font::load_fnt_font(font_path).unwrap();
    let mut window = gls::window::SdlGlWindow::new("Fps", width, height, font).unwrap();


    let gl = &window.gl().clone();


    window.set_background_color(na::Vector4::new(0.9, 0.9, 0.9, 1.0));

    window.setup_blend();

    let sb:ScreenBox = Default::default();
    let fps_coords = BoxCoords {x:0., y: 0.0};

    loop {

        unsafe {
            window.gl().Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }


        let time_ms =  1.0 / window.deltatime();
        window.text_renderer().render_text_with_box(&gl, &format!("Fps = {}", time_ms), fps_coords, Some(sb), 1.0);

        window.update::<()>(None);

    }

}
