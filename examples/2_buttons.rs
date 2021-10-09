use gl_lib_sdl as gls;
use gl_lib_sdl::{
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

    let mut container = setup_gui(&gl, width, height);

    let mut state = 1;


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

fn setup_gui_old(gl: &gl::Gl) -> gls::components::container::ComponentContainer<u32> {
    let mut container = gls::components::container::ComponentContainer::new();

    let button_1 = Box::new(gls::components::button::Button::new(gl));

    let button_2 = Box::new(gls::components::button::Button::new(gl));

    container.add_component(button_1, button_handler_1);

    container.add_component(button_2, button_handler_2);
    container
}


fn setup_gui(gl: &gl::Gl, width: u32, height: u32) -> gls::components::container::ComponentContainer<u32> {
    use gls::layout::row::*;
    use gls::layout::*;
    use gls::layout::element::*;
    use gls::components::attributes::*;
    use gls::components::button::*;


    let mut root_row = Row::new(Size { width: LengthAttrib::No(Length::Fill), height: LengthAttrib::Max(Length::Px(100)) });

    let btn_elm = ComponentElement::new(Box::new(Button::new(gl)), button_handler_1, Size {
        width: LengthAttrib::No(Length::Fill),
        height: LengthAttrib::No(Length::Px(200)),
    });


    root_row.add(Box::new(btn_elm));


    let mut cont = gls::components::container::ComponentContainer::new();

    root_row.add_to_container(&mut cont, RealizedSize {width, height});

    cont

}


fn button_handler_1(
    event: gls::components::base::ComponentEvent,
    comp: &mut dyn gls::components::base::Component,
    state: &mut u32,
) {
    use gls::components::base::ComponentEvent;
    match event {
        ComponentEvent::Clicked => {
            *state += 1;
            comp.update_content(format!("Btn state {}", state));
        }
        _ => {}
    }
}


fn button_handler_2(
    event: gls::components::base::ComponentEvent,
    comp: &mut dyn gls::components::base::Component,
    state: &mut u32,
) {
    use gls::components::base::ComponentEvent;
    match event {
        ComponentEvent::Clicked => {
            *state -= 1;
            comp.update_content(format!("Btn state {}", state));
        }
        _ => {}
    }
}
