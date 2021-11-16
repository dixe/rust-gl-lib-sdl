use gl_lib_sdl::{
    components::base,
    gl_lib::{
        gl,
        na,
        objects::square,
        ScreenCoords,
        ScreenBox,
        shader::Shader,
        text_rendering::{ text_renderer::TextRenderer },
    }
};


#[derive(Debug)]
pub struct GameComponent {
    pub shader: Shader,
    pub base: base::ComponentBase,
    columns: i32,
    rows: i32
}


impl GameComponent {

    pub fn new(gl: &gl::Gl) -> Box<Self> {
        let shader = default_shader(gl).unwrap();

        Box::new(Self {
            shader,
            columns: 9,
            rows: 9,
            base: Default::default()
        })
    }
}


impl<Message> base::ComponentTrait<Message> for GameComponent where Message: Clone  {

    fn base(&self) -> &base::ComponentBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut base::ComponentBase {
        &mut self.base
    }

    fn set_base(&mut self, base: base::ComponentBase) {
        self.base = base;
    }

    fn render(&self, gl: &gl::Gl, tr: &mut TextRenderer, render_square: &square::Square, screen_w: f32, screen_h: f32) {

        self.shader.set_used();

        let transform = self.base.unit_square_transform_matrix(screen_w as f32, screen_h as f32);

        self.shader.set_mat4(gl, "transform", transform);

        self.shader.set_f32(gl, "height", self.base.height);

        self.shader.set_f32(gl, "width", self.base.width);

        self.shader.set_f32(gl, "radius", 0.0); //0.1 / (f32::max(self.base.width / screen_w, self.base.height / screen_h)));

        render_square.render(&gl);
    }

    fn update_content(&mut self, _: String) {

    }

    fn on_event(&self, event: base::ComponentEvent) -> Option<Message> {
        match event {
            base::ComponentEvent::Clicked(vec2) => {
                let offset = na::Vector2::new(self.base.x as i32, self.base.y as i32);
                let relative = vec2 - offset;

                let x = ((relative.y as f32 / self.base.height ) * self.rows as f32) as i32;
                let y = ((relative.x as f32 / self.base.width ) * self.columns as f32) as i32;

                //println!("(row, column) = ({},{})", x, y);

                None

            },
            _ => None
        }
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
    vec2 Pos;
} OUT;

void main()
{
    vec4 pos = transform * vec4(aPos.x, aPos.y, aPos.z, 1.0);
    OUT.FragPos = aPos.xy;
    OUT.Pos = aPos.xy;
    gl_Position = pos;

}";

    let frag_source = r"
#version 330 core


in VS_OUTPUT {
    vec2 FragPos;
    vec2 Pos;
} IN;

out vec4 FragColor;

uniform float width;
uniform float height;


float grid(vec2 fragCoord, float space_x, float space_y, float gridWidth)
{
    vec2 p  = fragCoord - vec2(.5);
    vec2 size = vec2(gridWidth - .5);


    vec2 a1 = vec2(0.);
    a1.x = mod(p.x - size.x, space_x);
    a1.y = mod(p.y - size.y, space_y);

    vec2 a2 = vec2(0.);
    a2.x = mod(p.x + size.x, space_x);
    a2.y = mod(p.y + size.y, space_y);


    vec2 a = a2 - a1;

    float g = min(a.x, a.y);
    return clamp(g, 0., 1.0);
}

float border(vec2 fragCoord, float gridWidth)
{
    if(fragCoord.x < gridWidth * 2.0 || fragCoord.x > (width - gridWidth * 2.0) ||
    fragCoord.y < gridWidth * 2.0 || fragCoord.y > (height - gridWidth * 2.0))
    {
        return 0.0;
    }
    return 1.0;
}

void main()
{

    // maybe look at https://www.shadertoy.com/view/WtdSDs

    // Square is defined with corners in 0.5 and -0.5 on both x and y axis.
    // multiply by 2 to get -1.0...1.0 range
    float u = IN.FragPos.x + 0.5;
    float v = IN.FragPos.y + 0.5;

    vec2 fragCoord = vec2(u * width, v* height);

    if(fragCoord.y  > 100.0)
    {
//       discard;
    }

    //vec3 col = vec3(IN.FragPos.x, IN.FragPos.y, 0.0);
    vec3 col = vec3(u, v, 0.0);
    float space_x = width / 9.;
    float space_y = height / 9.;
    float grid_width = 2.0;
    col *= border(fragCoord, grid_width) * grid(fragCoord, space_x, space_y, grid_width);

    float alpha = 1.0;
    FragColor = vec4(col, alpha);

}";

    Shader::new(gl, vert_source, frag_source)
}
