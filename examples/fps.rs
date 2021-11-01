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

    let sb = ScreenBox::full_screen(width as f32, height as f32);

    let mut state = State {};

    loop {

        let time_ms =  1.0 / window.deltatime();
        window.render_text(&format!("Fps = {}", time_ms));

        window.update(&mut state);
    }
}


#[derive(Debug, Clone)]
pub enum Message {
}


struct State {

}

impl gls::State<Message> for State {

    fn handle_message(&mut self, message: &Message) {
    }


    fn view(&self, gl: &gl::Gl) -> Box::<gls::layout::element::Element<Message>> {
        use gls::layout::row::*;
        use gls::layout::column::*;
        use gls::layout::*;
        use gls::layout::element::*;
        use gls::layout::attributes::*;
        use gls::layout::button::*;

        Box::new(Column::new())

    }
}
