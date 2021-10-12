pub use crate::components::base::*;
use gl_lib::text_rendering::{ text_renderer::TextRenderer };
use gl_lib::{gl, na, shader::Shader, objects::square, BoxCoords, ScreenBox};

#[derive(Debug,Clone)]
pub struct Button {
    pub content: String, // Maybe use another compontent for content
    pub shader: Shader,
    //pub square: square::Square
}


impl Button {

    pub fn new(gl: &gl::Gl) -> Self {

        let shader = default_shader(gl).unwrap();

        Self {
            content: "Test btn".to_string(),
            shader,
        }
    }

    pub fn render(&self, gl: &gl::Gl, tr: &mut TextRenderer, render_square: &square::Square, screen_w: f32, screen_h: f32, base: &ComponentBase) {

        self.shader.set_used();

        let transform = base.unit_square_transform_matrix(screen_w as f32, screen_h as f32);


        let hover = if base.hover { 0.6 } else { 1.0 };

        let c = ComponentBase::window_to_screen_coords( (base.coords.x + base.width) / 2.0, (base.coords.y + base.height) / 2.0, screen_w, screen_h);

        self.shader.set_mat4(gl, "transform", transform);

        self.shader.set_f32(gl, "hover", hover);

        self.shader.set_f32(gl, "h_half", (base.height / screen_h));

        self.shader.set_f32(gl, "w_half", (base.width / screen_w));

        render_square.render(&gl);


        let button_screen_box = ScreenBox::new(base.coords.x, base.coords.y, base.width, base.height, screen_w, screen_h);


        let coords = BoxCoords {x: 0.0, y: 0.0};
        tr.render_text(gl, &self.content, coords, Some(button_screen_box), 1.0 );
    }
}



/// Creates a basic default shader that takes a mat4 transformation uniform transform
pub fn default_shader(gl: &gl::Gl) -> Result<Shader, failure::Error> {

    // default program for square
    let vert_source = r"#version 330 core
layout (location = 0) in vec3 aPos;

uniform mat4 transform;

out VS_OUTPUT {
    vec2 FragPos;
} OUT;

void main()
{
    vec4 pos = transform * vec4(aPos.x, aPos.y, aPos.z, 1.0);
    OUT.FragPos = aPos.xy;
    gl_Position = pos;

}";

    let frag_source = r"
#version 330 core


in VS_OUTPUT {
    vec2 FragPos;
} IN;

out vec4 FragColor;

uniform float w_half;
uniform float h_half;

uniform float hover;

float roundedRectangle(vec2 uv, vec2 size, float radius, float thickness)
{
  float d = length(max(abs(uv), size) - size) - radius;
  return smoothstep(0.66, 0.33, d / thickness);
}



void main()
{

    // maybe look at https://www.shadertoy.com/view/WtdSDs

    // Square is defined with corners in 0.5 and -0.5 on both x and y axis.
    // multiply by 2 to get -1.0...1.0 range
    float u = IN.FragPos.x * 2.0;
    float v = IN.FragPos.y * 2.0;


    float aspect = w_half / h_half;

    vec2 uv = vec2(u * aspect, v);

    vec3 col = vec3(.8, 0.8, .8) * hover;




    float radius = 0.4;
    // size = aspect - radius, 1.0 - radius
    vec2 size = vec2(aspect - radius, 1.0 - radius);

    // higher is more blur, and also thicker corners
    float aa = 0.05;
    float dist = roundedRectangle(uv, size, radius, aa);
    col =  col * dist;

    FragColor = vec4(col, smoothstep(0.9, 1.0, dist));

}";

    Shader::new(gl, vert_source, frag_source)
}
