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


    window.set_background_color(na::Vector4::new(0.9, 0.9, 0.9, 1.0));

    window.setup_blend();

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

    fn handle_message(&mut self, _message: &Message, _window_access: &gls::window::WindowComponentAccess) {
    }

    fn view(&self) -> gls::layout::node::Node<Message> {
        use gls::layout::column::*;


        Column::new().into()

    }
}
