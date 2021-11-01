use gl_lib::{self, na, gl, gl::viewport, objects::square, ScreenBox};
use gl_lib::text_rendering::{text_renderer, font};
use failure;
use deltatime;
use sdl2;
use crate::components::container::ComponentContainer;
use crate::state::State;
use crate::layout::RealizedSize;
use crate::layout::element::Element;


/// Struct given to component handlers to change things about the window.
pub struct WindowComponentAccess {
    video_subsystem: sdl2::VideoSubsystem,
}

impl WindowComponentAccess {

    /// Set the vsyn interval, see https://docs.rs/sdl2/0.34.5/sdl2/struct.VideoSubsystem.html#method.gl_set_swap_interval for more info on parameters
    /// Disable B vsync by calling with 0
    pub fn set_swap_interval<S: Into<sdl2::video::SwapInterval>>(& self, interval: S) {
        self.video_subsystem.gl_set_swap_interval(0);
    }


    pub fn enable_vsync(&self) {
        self.video_subsystem.gl_set_swap_interval(sdl2::video::SwapInterval::VSync);
    }
}

pub struct SdlGlWindow<Message> {
    sdl: sdl2::Sdl,
    gl: gl::Gl,
    window: sdl2::video::Window,
    viewport: viewport::Viewport,
    _gl_context: sdl2::video::GLContext,
    deltatime: deltatime::Deltatime,
    event_pump: sdl2::EventPump,
    quit: bool,
    event_handler: Box<dyn Fn(sdl2::event::Event)>,
    text_renderer: text_renderer::TextRenderer,
    render_square: square::Square,
    window_component_access: WindowComponentAccess,
    container: ComponentContainer<Message>,
    container_dirty: bool


}




impl<Message> SdlGlWindow<Message> where Message: Clone {

    pub fn new(window_text: &str, width: u32, height: u32, font: font::Font ) -> Result<Self, failure::Error> {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4,5);


        let viewport = viewport::Viewport::for_window(width as i32, height as i32);

        let window = video_subsystem
            .window(window_text, width, height)
            .opengl()
            .resizable()
            .build()?;


        let gl_context = window.gl_create_context().unwrap();
        let gl = gl::Gl::load_with(|s|{
            video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
        });

        let event_pump = sdl.event_pump().unwrap();

        viewport.set_used(&gl);

        let event_handler = Box::new(empty_handler);

        let text_renderer = text_renderer::TextRenderer::new(&gl, font);


        let render_square = square::Square::new(&gl);

        Ok(Self {
            gl,
            sdl,
            viewport,
            window,
            _gl_context: gl_context,
            deltatime: Default::default(),
            event_pump,
            quit: false,
            event_handler,
            text_renderer,
            render_square,
            window_component_access: WindowComponentAccess {
                video_subsystem
            },
            container: ComponentContainer::new(),
            container_dirty: true
        })

    }


    pub fn gl(&self) -> &gl::Gl {
        &self.gl
    }

    pub fn window(&self) -> &sdl2::video::Window {
        &self.window
    }

    pub fn sdl(&self) -> &sdl2::Sdl {
        &self.sdl
    }

    /// Return time last frame took to render in seconds.
    pub fn deltatime(&self) -> f32 {
        self.deltatime.time()
    }



    pub fn window_access(&self) -> &WindowComponentAccess {
        &self.window_component_access
    }

    /// Render components, Swap gl window, update internal delta time and handle sdl_events.
    /// Finish with clearing color_buffer_bit and depth_buffer_bit
    pub fn update(&mut self, mut state: &mut State<Message>) {

        if self.container_dirty {

            let mut cont = ComponentContainer::new();
            let size = (&self.viewport).into();
            state.view(&self.gl).add_to_container(&mut cont, &size, &self.text_renderer);
            self.container = cont;
            self.container_dirty = false;
        }


        self.render_components();

        self.window.gl_swap_window();
        self.deltatime.update();
        self.handle_events();

        // handle state update

        let mut popped_msg = self.container.messages.pop_front();
        while let Some(msg) = popped_msg {
            state.handle_message(&msg);
            self.container_dirty = true;
            popped_msg = self.container.messages.pop_front();
        }

        unsafe {
            self
                .gl
                .Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }


    pub fn should_quit(&self) -> bool {
        self.quit
    }

    pub fn set_background_color(&self, color: na::Vector4::<f32>) {
        unsafe {
            self.gl.ClearColor(color.x, color.y, color.z, color.w);
        }
    }


    pub fn render_text(&mut self, text: &str) {
        let sb = ScreenBox::full_screen(self.viewport.w as f32, self.viewport.h as f32);
        self.text_renderer.render_text(&self.gl, text, Default::default(), sb, 1.0);

    }

    pub fn setup_blend(&mut self) {
        self.text_renderer.setup_blend(&self.gl);
    }


    fn render_components(&mut self) {
        for comp in self.container.components.values() {
            comp.render(&self.gl, &mut self.text_renderer, &self.render_square, self.viewport.w as f32, self.viewport.h as f32);
        }

    }



    fn handle_events (&mut self) {

        use sdl2::event::Event;
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => self.quit = true,
                Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w,h),
                    ..
                } => {
                    self.viewport.update_size(w, h);
                    self.viewport.set_used(&self.gl);
                    self.container_dirty = true;
                },
                _ => {}
            };

            self.container.handle_sdl_event(event.clone(), &self.window_component_access);

            (self.event_handler)(event);
        }
    }

}


fn empty_handler( _:sdl2::event::Event)  {

}
