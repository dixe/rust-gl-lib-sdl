use gl_lib_sdl::layout::{
    column::*,
    row::*,
    column::*,
    attributes::*,
    attributes::Length::*,
    button::*,
    element::*,
    node::*,
};
use gl_lib_sdl::{
    components::container::ComponentContainer,
    window,
    layout::*,
};

use gl_lib_sdl::{
    gl_lib::text_rendering::{text_renderer::TextRenderer, font},
    gl_lib::{gl, na},
};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
enum Message {
    Msg1
}



#[test]
fn parent_constraint() {

    let width = 1000;
    let height = 600;

    let font_path = Path::new("./assets/fonts/Arial.fnt");
    let font = font::Font::load_fnt_font(font_path).unwrap();
    let mut window: window::SdlGlWindow<Message> = window::SdlGlWindow::new("Button", width, height, font).unwrap();


    let btn_width = 20;
    let spacing = 10;


    let btn_height = 211;
    let col = Column::new()
        .width(Fill)
        .height(FitContent)
        .padding(10.0)
        .spacing(spacing as f32)
        .add((Button::new(&window.gl(), "Right", Some(Message::Msg1))
              .width(Px(btn_width as u32))
              .height(Fill)));


    let node: Node<Message>  = col.into();
    let size = RealizedSize { x: 0.0, y: 0.0, width: width as f32, height: height as f32};
    let mut cont = ComponentContainer::new();
    node.add_to_container(&mut cont, &size, window.text_renderer());
    for (_, comp) in &cont.components {
        println!("{:?}", comp.base);
        // X should be width - width - spacing
        assert_eq!(comp.base.height as i32, 211);
    }
}


#[test]
fn parent_unconstraint() {

    let width = 1000;
    let height = 600;

    let font_path = Path::new("./assets/fonts/Arial.fnt");
    let font = font::Font::load_fnt_font(font_path).unwrap();
    let mut window: window::SdlGlWindow<Message> = window::SdlGlWindow::new("Button", width, height, font).unwrap();


    let btn_width = 20;

    let spacing = 10;

    let padding = 10;

    let col = Column::new()
        .width(Fill)
        .height(Fill)
        .padding(padding as f32)
        .spacing(spacing as f32)
        .add((Button::new(&window.gl(), "Right", Some(Message::Msg1))
              .width(Px(btn_width as u32))
              .height(Fill)));


    let node: Node<Message>  = col.into();
    let size = RealizedSize { x: 0.0, y: 0.0, width: width as f32, height: height as f32};
    let mut cont = ComponentContainer::new();
    node.add_to_container(&mut cont, &size, window.text_renderer());
    for (_, comp) in &cont.components {
        println!("{:?}", comp.base);

        assert_eq!(comp.base.height as i32, height as i32 - padding * 2 );
    }
}
