use gl_lib_sdl as gls;
use gl_lib_sdl::{
    gl_lib::text_rendering::font,
    gl_lib::na,
};

use failure;
use std::path::Path;

mod game;

use game::*;


#[derive(Debug, Clone)]
pub enum Message {
    Restart
}

fn main() -> Result<(), failure::Error> {
    let width = 600;
    let height = 600;

    let font_path = Path::new("./assets/fonts/Arial.fnt");
    let font = font::Font::load_fnt_font(font_path).unwrap();
    let mut window = gls::window::SdlGlWindow::new("MineSweeper", width, height, font).unwrap();

    window.set_background_color(na::Vector4::new(0.9, 0.9, 0.9, 1.0));

    window.setup_blend();


    let mut state = GameState { };
    while !window.should_quit() {

        window.update(&mut state);
    }

    Ok(())
}


struct GameState {

}






impl gls::State<Message> for GameState {

    fn handle_message(&mut self, message: &Message, _window_access: &gls::window::WindowComponentAccess) {

        match message {
            Message::Restart => { },
        }
    }


    fn view(&self) -> gls::layout::node::Node<Message> {
        use gls::layout::row::*;
        use gls::layout::column::*;
        use gls::layout::element::*;
        use gls::layout::attributes::*;
        use gls::layout::button::*;

        use Length::*;

        let col = Column::new()
            .add(Row::new()
                 .padding(5.0)
                 .width(Fill)
                 .add(Button::new("Time", Some(Message::Restart))
                      .height(Px(50))
                 )
                 .add(Button::new("Restart", Some(Message::Restart))
                      .height(Px(50))
                      .align_center()
                 )
                 .add(Button::new("Score", Some(Message::Restart))
                      .height(Px(50))
                      .align_right()
                 ))
            .add(GameLayout::new()
                 .height(Fill));
        col.into()
    }
}
