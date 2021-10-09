use gl_lib::text_rendering::{ text_renderer::TextRenderer };
use gl_lib::{ gl, na, na::Translation3, BoxCoords, ScreenBox, ScreenCoords };


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
