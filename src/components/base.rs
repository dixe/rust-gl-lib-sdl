use gl_lib::text_rendering::{ text_renderer::TextRenderer };
use gl_lib::{gl, na, objects::square, shader,  na::Translation3, BoxCoords, ScreenBox, ScreenCoords};
use crate::components::button::Button;


#[derive(Debug,Clone,Copy)]
pub enum ComponentEvent {
    Clicked,
    Hover
}

pub type EventHandler = Box<dyn Fn(ComponentEvent)>;

pub type Level = u32;


#[derive(Debug,Clone,Copy)]
pub struct ComponentBase {
    width: f32,
    height: f32,
    pub coords: BoxCoords,
    pub level: Level,
}

impl ComponentBase {
    pub fn new() -> Self {
        Self {
            width: 0.1,
            height: 0.05,

            coords: BoxCoords {
                x: 0.2,
                y: 0.1
            },
            level: 0 ,
        }
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }
}

#[derive(Debug,Clone,Copy)]
pub enum OnTop {
    No,
    OnTop(Level)
}


pub struct Component {
    base: ComponentBase,
    comp_type: ComponentType
}

pub enum ComponentType {
    Btn(Button)
}


impl Component {
    pub fn on_top(&self, x: f32, y: f32) -> OnTop {

        println!("mouse({:?},{})", x * 2.0 -1.0, 2.0*-y + 1.0);


        if x >= self.base.coords.x && x <= self.base.coords.x + self.base.width && y >= self.base.coords.y && y <= self.base.coords.y + self.base.height {
            return OnTop::OnTop(self.base.level)
        }

        OnTop::No
    }

    pub fn render(&self, gl: &gl::Gl, tr: &mut TextRenderer, parent_screen_box: ScreenBox) {

        match &self.comp_type {
            ComponentType::Btn(btn) => {
                btn.shader.set_used();

                let transform = self.unit_square_transform_matrix(parent_screen_box);

                btn.shader.set_mat4(gl, "transform", transform);
                btn.square.render(&gl);


                let button_screen_box = parent_screen_box.create_child(self.base.coords, self.base.width(), self.base.height());

                self.render_text(gl, tr, &btn.content, button_screen_box);
            }
        }
    }


    // TODO: maybe move out as a seperate function that just takes component base instead of self
    fn unit_square_transform_matrix(&self, base_box: ScreenBox) -> na::Matrix4::<f32> {

        //let screen_coords_top_left = self.window_to_screen_coords(self.base.coords.x, self.base.coords.y , w, h);

        let x_scale = self.base.width * 2.0;
        let y_scale = self.base.height * 2.0;

        let mut model = na::Matrix4::<f32>::identity();



        // Scale to size
        model[0] = x_scale;
        model[5] = y_scale;

        // move to position

        // current position is top_left = -0.5* x_scale, 0.5 * y_scale bottom_right = 0.5 * x_scale, -0.5 * y_scale

        //println!("{:#?}", self.base);

        // get screen coords / 2 + 1 by multiply with base_box

        let x_move = (base_box.left() * 2.0 - 1.0) + self.base.coords.x  * 2.0 + x_scale * 0.5;
        let y_move = base_box.top() * 2.0 + 1.0 - self.base.coords.y * 2.0 - y_scale * 0.5;
        let trans = Translation3::new(x_move, y_move, 0.0);


        model = trans.to_homogeneous() * model;

        model
    }


    pub fn update_content(&mut self, content: String) {
        match &mut self.comp_type {
            ComponentType::Btn(btn) => {
                btn.content = content;
            }
        }
    }

    fn render_text(&self, gl: &gl::Gl, tr: &mut TextRenderer, content: &str, screen_box: ScreenBox) {
        let coords = BoxCoords {x: 0.0, y:0.0};
        tr.render_text(gl, content, coords, Some(screen_box), 1.0 );
    }
}



pub fn button(gl: &gl::Gl) -> Component {
    let shader = square::Square::default_shader(gl).unwrap();

    Component {
        base: ComponentBase::new(),
        comp_type: ComponentType::Btn(Button::new(gl))
    }
}


/*
pub trait Component {

fn component_base(&self) -> &ComponentBase;

fn content(&self) -> &str;

fn update_content(&mut self, content: String);

fn render(&self, gl: &gl::Gl, tr: &mut TextRenderer, parent_screen_box: ScreenBox);

fn render_text(&self, gl: &gl::Gl, tr: &mut TextRenderer, screen_box: ScreenBox) {
let coords = BoxCoords {x: 0.0, y:0.0};
tr.render_text(gl, self.content(), coords, Some(screen_box), 1.0 );
}

    // TODO: Maybe have this as an method/function to be implemented
    // And just have helper functions defined on Component
    // Maybe we want circles at some point
    fn on_top(&self, x: f32, y: f32) -> OnTop {

    println!("mouse({:?},{})", x * 2.0 -1.0, 2.0*-y + 1.0);
    let cb = self.component_base();

    if x >= cb.coords.x && x <= cb.coords.x + cb.width && y >= cb.coords.y && y <= cb.coords.y + cb.height {
    return OnTop::OnTop(cb.level)
}

    OnTop::No
}


    fn unit_square_transform_matrix(&self, base_box: ScreenBox) -> na::Matrix4::<f32> {

    let cb = self.component_base();

    //let screen_coords_top_left = self.window_to_screen_coords(cb.coords.x, cb.coords.y , w, h);

    let x_scale = cb.width * 2.0;
    let y_scale = cb.height * 2.0;

    let mut model = na::Matrix4::<f32>::identity();



    // Scale to size
    model[0] = x_scale;
    model[5] = y_scale;

    // move to position

    // current position is top_left = -0.5* x_scale, 0.5 * y_scale bottom_right = 0.5 * x_scale, -0.5 * y_scale

    //println!("{:#?}", cb);

    // get screen coords / 2 + 1 by multiply with base_box

    let x_move = (base_box.left() * 2.0 - 1.0) + cb.coords.x  * 2.0 + x_scale * 0.5;
    let y_move = base_box.top() * 2.0 + 1.0 - cb.coords.y * 2.0 - y_scale * 0.5;
    let trans = Translation3::new(x_move, y_move, 0.0);


    model = trans.to_homogeneous() * model;

    model
}
}
     */
