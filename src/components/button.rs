pub use crate::components::base::*;
use gl_lib::text_rendering::{text_renderer::TextRenderer};
use gl_lib::{gl, objects::square, shader, ScreenBox};


pub struct Button {
    pub content: String, // Maybe use another compontent for content
    pub shader: shader::Shader,
    pub square: square::Square
}


impl Button {

    pub fn new(gl: &gl::Gl) -> Self {

        let shader = square::Square::default_shader(gl).unwrap();

        Self {
            content: "Test btn".to_string(),
            square: square::Square::new(gl),
            shader,
        }
    }

}

/*
impl Component for Button {

fn component_base(&self) -> &ComponentBase {
&self.base
    }

    fn content(&self) -> &str {
        &self.content
    }



}
*/
