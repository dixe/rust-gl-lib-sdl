use gl_lib::text_rendering::{text_renderer::TextRenderer};
use gl_lib::{gl};


#[derive(Debug,Clone,Copy)]
pub enum ComponentEvent {
    Clicked
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
            pos_x: 0.0,
            pos_y: 0.0,
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
pub enum ClickRes {
    NoClick,
    Click(Level)
}

pub trait Component {

    fn component_base(&self) -> &ComponentBase;

    fn content(&self) -> &str;

    fn update_content(&mut self, content: String);

    fn render(&self, gl: &gl::Gl, tr: &mut TextRenderer, w: i32, h: i32) {

        let cb = self.component_base();

        let screen_coords = window_to_screen_coords(cb.pos_x, cb.pos_y , w, h);
        tr.render_text(gl, self.content(), screen_coords.x, screen_coords.y, 1.0 );
    }

    // TODO: Maybe have this as an method/function to be implemented
    // And just have helper functions defined on Component
    // Maybe we want circles at some point
    fn clicked(&self, x: f32, y: f32) -> ClickRes {

        let cb = self.component_base();
        if x >= cb.pos_x && x <= cb.pos_x + cb.width && y >= cb.pos_y && y <= cb.pos_y + cb.height {
            return ClickRes::Click(cb.level)
        }

        ClickRes::NoClick
    }
}


#[derive(Debug, Copy, Clone)]
struct ScreenCoord {
    x: f32, y: f32
}

fn window_to_screen_coords(x: f32, y: f32, w: i32, h: i32) -> ScreenCoord {
    ScreenCoord {x : x *2.0/ (w as f32)  - 1.0, y: -y *2.0 / (h as f32) + 1.0 }
}
