use gl_lib_sdl::{
    gl_lib::{gl, na, shader::Shader},
    gl_lib::objects::*,
};


pub struct State {
    square: square::Square,
    square_shader: Shader,
    gl: gl::Gl,

}


impl State {

    pub fn new(gl: &gl::Gl) -> Self {
        Self {
            square: square::Square::new(gl),
            square_shader: square_shader(gl).unwrap(),
            gl: gl.clone()
        }
    }

    pub fn render(&self) {
        self.square_shader.set_used();
        self.square_shader.set_mat4(&self.gl, "transform", na::Matrix4::identity());
        self.square.render(&self.gl);
    }


    pub fn handle_events(&mut self, event: sdl2::event::Event) {
        println!("{:?}", event);
    }

}

/// Creates a shader for rendering a square (two triangle)
fn square_shader(gl: &gl::Gl) -> Result<Shader, failure::Error> {

    // default program for square
    let vert_source = std::include_str!("square_shader.vert");
    let frag_source = std::include_str!("square_shader.frag");

    Shader::new(gl, vert_source, frag_source)
}
