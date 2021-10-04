use gl_lib::text_rendering::{text_renderer::TextRenderer};
use gl_lib::{gl};

pub struct ComponentBase {
    width: LengthContraint,
    height: LengthContraint,
    pos_x: f32,
    pos_y: f32

}

impl ComponentBase {
    pub fn new() -> Self {
        Self {
            width: LengthContraint::No(Length::Px(30.0)),
            height: LengthContraint::No(Length::Px(30.0)),
            pos_x: 0.0,
            pos_y: 0.0
        }
    }
}

pub enum LengthContraint {
    No(Length),
    Max(Length),
    Min(Length),
    MinMax(Length, Length)
}

pub enum Length {
    Px(f32),
    Shrink,
    Fill,
    FillPortion(f32)
}



pub trait Component {

    fn component_base(&self) -> &ComponentBase;

    fn content(&self) -> &str;

    fn render(&self, gl: &gl::Gl, tr: &mut TextRenderer) {

        let cb = self.component_base();
        tr.render_text(gl, self.content(), cb.pos_x, cb.pos_y, 1.0 );




    }



}
