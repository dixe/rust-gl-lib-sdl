use gl_lib_sdl as gls;
use gl_lib_sdl::{
    gl_lib::text_rendering::font,
    gl_lib::{gl, na},
};
use failure;
use std::path::Path;

#[derive(Debug, Clone)]
pub enum Message {
    Add,
    Sub,
    Clear
}

fn main() -> Result<(), failure::Error> {
    let width = 1000;
    let height = 600;

    let font_path = Path::new("./assets/fonts/Arial.fnt");
    let font = font::Font::load_fnt_font(font_path).unwrap();
    let mut window = gls::window::SdlGlWindow::new("Button", width, height, font).unwrap();

    window.set_background_color(na::Vector4::new(0.9, 0.9, 0.9, 1.0));

    window.setup_blend();

    let mut world = World { total: 0 };

    while !window.should_quit() {

        window.update(&mut world);
    }

    Ok(())
}



struct World {
    pub total: i32
}

impl gls::State<Message> for World {

    fn handle_message(&mut self, message: &Message) {

        match message {
            Message::Add => { self.total += 1; },
            Message::Sub => { self.total -= 1; },
            Message::Clear => { self.total = 0; },
        }
    }


    fn view(&self, gl: &gl::Gl) -> gls::layout::node::Node<Message> {
        use gls::layout::row::*;
        use gls::layout::column::*;
        use gls::layout::element::*;
        use gls::layout::attributes::*;
        use gls::layout::button::*;

        use Length::*;

        let col = Column::new().width(Fill)
            .padding(10.0)
            .spacing(10.0)
            .add(Row::new()
                 .width(Fill)
                 .add_attribute(Attribute::Spacing(20.0))
                 .add(Button::new(gl, "Add", Some(Message::Add))
                      .width(Fill)
                      .height(Px(50.)))
                 .add(Button::new(gl, "Sub", Some(Message::Sub))
                      .width(Fill)
                      .height(Px(50.)))
                 .add(Button::new(gl, "Clear", Some(Message::Clear))
                      .width(Fill)
                      .height(Px(50.))))
            .add(Button::new(gl, &format!("Total = {}", self.total), {
                if self.total < 3 {
                    None
                }
                else {
                    Some(Message::Clear)
                }})
                 .width(Fill)
                 .height(Fill));


        col.into()
    }
}
