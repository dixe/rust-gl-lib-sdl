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
fn align_left() {

    let width = 1000;
    let height = 600;

    let font_path = Path::new("./assets/fonts/Arial.fnt");
    let font = font::Font::load_fnt_font(font_path).unwrap();
    let mut window: window::SdlGlWindow<Message> = window::SdlGlWindow::new("Button", width, height, font).unwrap();


    let btn_width = 20;
    let spacing = 10;

    let col = Column::new()
        .width(Fill)
        .height(FitContent)
        .padding(10.0)
        .spacing(spacing as f32)
        .add(Row::new()
             .width(Fill)
             .add_attribute(Attribute::Spacing(10.0))
             .add(Button::new(&window.gl(), "Right", Some(Message::Msg1))
                  .width(Px(btn_width as u32))
                  .align_left()
                  .height(Fill)));


    let node: Node<Message>  = col.into();
    let size = RealizedSize { x: 0.0, y: 0.0, width: width as f32, height: height as f32};
    let mut cont = ComponentContainer::new();
    node.add_to_container(&mut cont, &size, window.text_renderer());
    for (_, comp) in &cont.components {
        println!("{:?}",comp.base.coords.x as i32);
        // X should be width - width - spacing
        assert_eq!(comp.base.coords.x as i32, spacing);
        assert_eq!(comp.base.coords.y as i32, spacing);
    }
}


#[test]
fn align_right() {

    let width = 1000;
    let height = 600;

    let font_path = Path::new("./assets/fonts/Arial.fnt");
    let font = font::Font::load_fnt_font(font_path).unwrap();
    let mut window: window::SdlGlWindow<Message> = window::SdlGlWindow::new("Button", width, height, font).unwrap();


    let btn_width = 20;
    let spacing = 10;

    let col = Column::new()
        .width(Fill)
        .height(FitContent)
        .padding(10.0)
        .spacing(spacing as f32)
        .add(Row::new()
             .width(Fill)
             .add_attribute(Attribute::Spacing(10.0))
             .add(Button::new(&window.gl(), "Right", Some(Message::Msg1))
                  .width(Px(btn_width as u32))
                  .align_right()
                  .height(Fill)));


    let node: Node<Message>  = col.into();
    let size = RealizedSize { x: 0.0, y: 0.0, width: width as f32, height: height as f32};
    let mut cont = ComponentContainer::new();
    node.add_to_container(&mut cont, &size, window.text_renderer());
    for (_, comp) in &cont.components {
        println!("{:?}",comp.base.coords.x as i32);
        // X should be width - width - spacing
        assert_eq!(comp.base.coords.x as i32, (width as i32 - btn_width - spacing));
        assert_eq!(comp.base.coords.y as i32, spacing);
    }

}

#[test]
fn align_center() {

    let width = 1000;
    let height = 600;

    let font_path = Path::new("./assets/fonts/Arial.fnt");
    let font = font::Font::load_fnt_font(font_path).unwrap();
    let mut window: window::SdlGlWindow<Message> = window::SdlGlWindow::new("Button", width, height, font).unwrap();


    let btn_width = 20;
    let spacing = 10;

    let col = Column::new()
        .width(Fill)
        .height(FitContent)
        .padding(spacing as f32)
        .spacing(spacing as f32)
        .add(Row::new()
             .width(Fill)
             .add_attribute(Attribute::Spacing(10.0))
             .add(Button::new(&window.gl(), "Right", Some(Message::Msg1))
                  .width(Px(btn_width as u32))
                  .align_center()
                  .height(Fill)));


    let node: Node<Message>  = col.into();
    let size = RealizedSize { x: 0.0, y: 0.0, width: width as f32, height: height as f32};
    let mut cont = ComponentContainer::new();
    node.add_to_container(&mut cont, &size, window.text_renderer());
    for (_, comp) in &cont.components {
        println!("{:?}",comp.base.coords.x);
        assert_eq!(comp.base.coords.x as u32,  width/2 - (btn_width / 2 + spacing) as u32 );
        assert_eq!(comp.base.coords.y as i32, spacing);
    }
}
