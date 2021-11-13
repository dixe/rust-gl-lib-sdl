use gl_lib::text_rendering::{ text_renderer::TextRenderer };
use gl_lib::{gl, na, na::Translation3, objects::square, ScreenCoords };
use crate::components::button::Button;
use std::fmt::Debug;
use crate::layout;

#[derive(Debug,Clone,Copy)]
pub enum ComponentEvent {
    Clicked,
    Hover,
    HoverEnd,
}


pub type Level = u32;
#[derive(Debug, Default,  Clone, Copy)]
pub struct Coords {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ComponentBase {
    pub width: f32,
    pub height: f32,
    pub coords: Coords,
    pub level: Level,
    pub hover: bool,
}


impl ComponentBase {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            width,
            height,

            coords: Coords {
                x,
                y
            },
            hover: false,
            level: 0 ,
        }
    }

    pub fn set_width(&mut self, w_pixels: f32) {
        self.width = w_pixels;
    }


    pub fn set_height(&mut self, h_pixels: f32) {
        self.height = h_pixels;
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn unit_square_transform_matrix(&self, screen_w: f32, screen_h: f32) -> na::Matrix4::<f32> {

        let sc_top_left = Self::window_to_screen_coords(self.coords.x, self.coords.y, screen_w, screen_h);

        let x_scale = self.width / screen_w  * 2.0;
        let y_scale = self.height / screen_h * 2.0;

        let mut model = na::Matrix4::<f32>::identity();

        // Scale to size
        model[0] = x_scale;
        model[5] = y_scale;

        // move to position

        // current position is top_left = -0.5* x_scale, 0.5 * y_scale bottom_right = 0.5 * x_scale, -0.5 * y_scale

        //println!("{:#?}", self);

        // get screen coords / 2 + 1 by multiply with base_box

        let x_move = sc_top_left.x + x_scale * 0.5;
        let y_move = sc_top_left.y - y_scale * 0.5;

        /*
        let x_move = (base_box.left() * 2.0 - 1.0) + self.coords.x  * 2.0 + x_scale * 0.5;
        let y_move = base_box.top() * 2.0 + 1.0 - self.coords.y * 2.0 - y_scale * 0.5;

         */
        let trans = Translation3::new(x_move, y_move, 0.0);


        model = trans.to_homogeneous() * model;

        model
    }



    pub fn window_to_screen_coords(x: f32, y: f32, w: f32, h: f32) -> ScreenCoords {
        ScreenCoords {x : x *2.0/ w  - 1.0, y: -y *2.0 / h + 1.0 }
    }
}


impl From<layout::RealizedSize> for ComponentBase {
    fn from(rs: layout::RealizedSize) -> Self {
        Self::new(rs.x, rs.y, rs.width, rs.height)
    }
}


#[derive(Debug,Clone,Copy)]
pub enum OnTop {
    No,
    OnTop(Level)
}


#[derive(Debug, Clone)]
pub struct Component<Message> {
    pub base: ComponentBase,
    pub comp_type: ComponentType<Message>,
}


impl<Message> Component<Message> where Message : Clone {
    pub fn on_top(&self, x: f32, y: f32) -> OnTop {

        if x >= self.base.coords.x && x <= self.base.coords.x + self.base.width && y >= self.base.coords.y && y <= self.base.coords.y + self.base.height {
            return OnTop::OnTop(self.base.level)
        }
        OnTop::No
    }

    pub fn render(&self, gl: &gl::Gl, tr: &mut TextRenderer, render_square: &square::Square, screen_w: f32, screen_h: f32) {
        match &self.comp_type {
            ComponentType::Btn(btn) => {
                btn.render(gl, tr, render_square, screen_w, screen_h, &self.base);
            }
        }
    }

    pub fn update_content(&mut self, content: String) {
        match &mut self.comp_type {
            ComponentType::Btn(btn) => {
                btn.content = content
            }
        }
    }

    pub fn on_event(&self, event: ComponentEvent) -> Option<Message> {

        match &self.comp_type {
            ComponentType::Btn(btn) => {
                match event {
                    ComponentEvent::Clicked => {
                        btn.on_click_msg.clone()
                    },
                    _ => None
                }
            }
        }

    }

}

#[derive(Debug,Clone)]
pub enum ComponentType<Message> {
    Btn(Button<Message>)
}
