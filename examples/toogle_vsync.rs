use gl_lib_sdl as gls;
use gl_lib_sdl::{
    window,
    gl_lib::text_rendering::font,
    gl_lib::{gl, na}
};


use failure;
use std::path::Path;


#[derive(Debug, Clone)]
pub enum Message {
    Toogle,
}

fn main() -> Result<(), failure::Error> {
    let width = 800;
    let height = 600;

    let font_path = Path::new("./assets/fonts/Arial.fnt");
    let font = font::Font::load_fnt_font(font_path).unwrap();
    let mut window = gls::window::SdlGlWindow::new("Button", width, height, font).unwrap();


    window.set_background_color(na::Vector4::new(0.9, 0.9, 0.9, 1.0));

    window.setup_blend();

    let mut state = World::default();

    while !window.should_quit() {

        let time_ms =  1.0 / window.deltatime();
        window.render_text(&format!("Fps = {}", time_ms));
        window.update(&mut state);

    }

    Ok(())
}



#[derive(Default)]
struct World {
    v_sync_on: bool
}

impl gls::State<Message> for World {

    fn handle_message(&mut self, message: &Message, window_access: &window::WindowComponentAccess) {

        match message {
            Message::Toogle => {
                self.v_sync_on = !self.v_sync_on;
                match self.v_sync_on {
                    false => {
                        window_access.set_swap_interval(0);
                    },
                    true => {
                        window_access.enable_vsync();

                    }
                }
            }
        }
    }


    fn view(&self, gl: &gl::Gl) -> gls::layout::node::Node<Message> {
        use gls::layout::element::*;
        use gls::layout::attributes::*;
        use gls::layout::button::*;

        use Length::*;

        Button::new(gl, &format!("Toogle") ,Some(Message::Toogle))
            .width(Px(100.))
            .height(Px(60.))
            .into()
    }
}
