use gl_lib_sdl as gls;
use gl_lib_sdl::{
    components::base,
    gl_lib::text_rendering::font,
    gl_lib::{gl, na, BoxCoords, ScreenCoords, ScreenBox}
};

use failure;
use std::path::Path;

fn main() -> Result<(), failure::Error> {
    let width = 800;
    let height = 600;

    let font_path = Path::new("./assets/fonts/Arial.fnt");
    let font = font::Font::load_fnt_font(font_path).unwrap();
    let mut window = gls::window::SdlGlWindow::new("Button", width, height, font).unwrap();

    let gl = &window.gl().clone();

    window.set_background_color(na::Vector4::new(0.9, 0.9, 0.9, 1.0));

    window.setup_blend();

    let mut container = setup_gui(&gl, width as f32, height as f32);
    //let mut container = setup_gui_old(&gl);

    let mut state: i32 = 1;


    let sb:ScreenBox = Default::default();
    let text_coords = BoxCoords {x:0.5, y:0.5};

    while !window.should_quit() {

        window
            .text_renderer()
            .render_text(&gl, &format!("State = {}", state), text_coords, Some(sb), 1.0);


        window.update(Some((&mut container, &mut state)));

        //window.update::<()>(None);
    }

    Ok(())
}

fn setup_gui_old(gl: &gl::Gl) -> gls::components::container::ComponentContainer<i32> {
    let mut container = gls::components::container::ComponentContainer::new();

    container.add_component(base::button(&gl), button_handler_1);
    container
}


fn setup_gui(gl: &gl::Gl, width: f32, height: f32) -> gls::components::container::ComponentContainer<i32> {
    use gls::layout::row::*;
    use gls::layout::*;
    use gls::layout::element::*;
    use gls::layout::attributes::*;



    let mut root_row = Row::new(Size { width: LengthAttrib::No(Length::Fill), height: LengthAttrib::Max(Length::Px(100.0)) });


    let mut btn = base::button(gl);

    btn.update_content("Add".to_string());
    let btn_elm = ComponentElement::new(btn, button_handler_1, Size {
        width: LengthAttrib::No(Length::Fill),
        height: LengthAttrib::No(Length::Px(50.)),
    });

    root_row.add(Box::new(btn_elm));


    let mut btn2 = base::button(gl);
    btn2.update_content("Sub".to_string());

    let btn_elm_2 = ComponentElement::new(btn2, button_handler_2, Size {
        width: LengthAttrib::No(Length::Fill),
        height: LengthAttrib::No(Length::Px(50.)),
    });



    root_row.add(Box::new(btn_elm_2));
    let mut cont = gls::components::container::ComponentContainer::new();

    root_row.add_to_container(&mut cont, &RealizedSize { x: 0.0, y: 0.0, width, height });

    cont

}


fn button_handler_1(event: gls::components::base::ComponentEvent, comp: &mut gls::components::base::Component, state: &mut i32) {
    use gls::components::base::ComponentEvent;

    match event {
        ComponentEvent::Clicked => {
            //println!("ADD");
            *state += 1;
        }
        _ => {}
    }
}


fn button_handler_2(event: gls::components::base::ComponentEvent, comp: &mut gls::components::base::Component, state: &mut i32) {
    use gls::components::base::ComponentEvent;


    match event {
        ComponentEvent::Clicked => {
            //println!("SUB");

            *state -= 1;
        }
        _ => {}
    }
}
