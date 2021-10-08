use gl_lib::text_rendering::{ text_renderer::TextRenderer };
use gl_lib::{ gl, na, na::Translation3 };


#[derive(Debug,Clone,Copy)]
pub enum ComponentEvent {
    Clicked,
    Hover
}

pub type EventHandler = Box<dyn Fn(ComponentEvent)>;


#[derive(Debug,Clone,Copy)]
pub struct ComponentBase {
    width: f32,
    height: f32,
    pos_x: f32,
    pos_y: f32,
    level: Level,
}

impl ComponentBase {
    pub fn new(level: Level) -> Self {
        Self {
            width: 70.0,
            height: 30.0,
            // upper left
            pos_x: 10.0,
            pos_y: 100.0,
            level,
        }
    }
}

#[derive(Debug,Clone,Copy)]
pub enum LengthContraint {
    No(Length),
    Max(Length),
    Min(Length),
    MinMax(Length, Length)
}

#[derive(Debug,Clone,Copy)]
pub enum Length {
    Px(f32),
    Shrink,
    Fill,
    FillPortion(f32)
}


pub type Level = u32;

#[derive(Debug,Clone,Copy)]
pub enum OnTop {
    No,
    OnTop(Level)
}

pub trait Component {

    fn component_base(&self) -> &ComponentBase;

    fn content(&self) -> &str;

    fn update_content(&mut self, content: String);

    fn render(&self, gl: &gl::Gl, tr: &mut TextRenderer, w: i32, h: i32);

    fn render_text(&self, gl: &gl::Gl, tr: &mut TextRenderer, w: i32, h: i32) {

        let cb = self.component_base();

        let screen_coords = self.window_to_screen_coords(cb.pos_x, cb.pos_y , w, h);
        tr.render_text(gl, self.content(), screen_coords.x, screen_coords.y, 1.0 );
    }

    // TODO: Maybe have this as an method/function to be implemented
    // And just have helper functions defined on Component
    // Maybe we want circles at some point
    fn on_top(&self, x: f32, y: f32) -> OnTop {

        let cb = self.component_base();
        if x >= cb.pos_x && x <= cb.pos_x + cb.width && y >= cb.pos_y && y <= cb.pos_y + cb.height {
            return OnTop::OnTop(cb.level)
        }

        OnTop::No
    }

    fn window_to_screen_coords(&self, x: f32, y: f32, w: i32, h: i32) -> ScreenCoord {
        ScreenCoord {x : x *2.0/ (w as f32)  - 1.0, y: -y *2.0 / (h as f32) + 1.0 }
    }

    fn unit_square_transform_matrix(&self, w: i32, h: i32) -> na::Matrix4::<f32> {

        let cb = self.component_base();

        let screen_coords_top_left = self.window_to_screen_coords(cb.pos_x, cb.pos_y , w, h);

        let x_scale = (cb.width / w as f32) * 2.0;
        let y_scale = (cb.height / h as f32) * 2.0;

        let mut model = na::Matrix4::<f32>::identity();



        // Scale to size
        model[0] = x_scale;
        model[5] = y_scale;

        // move to position

        // current position is top_left = -0.5, 0.5 bottom_right = 0.5, -0.5

        let x_move = screen_coords_top_left.x + x_scale * 0.5;
        let y_move = screen_coords_top_left.y - y_scale * 0.5;

        let trans = Translation3::new(x_move, y_move, 0.0);


        model = trans.to_homogeneous() * model;

        model




    }

}



#[derive(Debug, Copy, Clone)]
pub struct ScreenCoord {
    x: f32,
    y: f32
}
