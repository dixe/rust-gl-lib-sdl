pub use crate::components::base::*;
use gl_lib::text_rendering::{text_renderer::TextRenderer};
use gl_lib::{gl, objects::square, shader, ScreenBox};


pub struct Button {
    base: ComponentBase,
    content: String, // Maybe use another compontent for content
    shader: shader::Shader,
    square: square::Square
}


impl Button {

    pub fn new(gl: &gl::Gl, level: Level) -> Self {

        let shader = square::Square::default_shader(gl).unwrap();

        Self {
            base: ComponentBase::new(level),
            content: "Test btn".to_string(),
            square: square::Square::new(gl),
            shader,
        }
    }
}

impl Component for Button {

    fn component_base(&self) -> &ComponentBase {
        &self.base
    }

    fn content(&self) -> &str {
        &self.content
    }

    fn update_content(&mut self, content: String) {
        self.content = content;
    }

    fn render(&self, gl: &gl::Gl, tr: &mut TextRenderer, parent_screen_box: ScreenBox) {
        self.shader.set_used();

        let transform = self.unit_square_transform_matrix(parent_screen_box);

        self.shader.set_mat4(gl, "transform", transform);
        self.square.render(&gl);


        let button_screen_box = parent_screen_box.create_child(self.base.coords, self.base.width, self.base.height);

        self.render_text(gl, tr, button_screen_box);
    }
}
