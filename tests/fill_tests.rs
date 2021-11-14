use gl_lib_sdl::layout::{
    column::*,
    attributes::Length::*,
    button::*,
    element::*,
};
use gl_lib_sdl::{
    window,
    layout::*,
};

use gl_lib_sdl::{
    gl_lib::text_rendering::font,
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
        .add(Button::new("Right", Some(Message::Msg1))
             .width(Px(btn_width as u32))
             .height(Fill));



    let aligned_elements = engine::align_tree(col.into(), engine::Size { w: width as f32, h: height as f32 }, window.text_renderer());

    let mut found = false;
    for elm in &aligned_elements {
        println!("{}.size = {:?}", elm.node.name(), elm.realized_size);
    }

    for elm in &aligned_elements {
        if elm.node.name() == "button" {
            assert_eq!(elm.realized_size.height as i32, btn_height );
        }
        found = true;
    }

    assert!(found);
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
        .add(Button::new("Right", Some(Message::Msg1))
             .width(Px(btn_width as u32))
             .height(Fill));



    let aligned_elements = engine::align_tree(col.into(), engine::Size { w: width as f32, h: height as f32 }, window.text_renderer());

    let mut found = false;
    for elm in &aligned_elements {
        println!("{}.size = {:?}", elm.node.name(), elm.realized_size);
    }

    for elm in &aligned_elements {
        if elm.node.name() == "button" {
            assert_eq!(elm.realized_size.height as i32, height as i32 - padding * 2);
        }
        found = true;
    }

    assert!(found);

}
