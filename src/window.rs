use gl_lib::{self, na, gl, gl::viewport, ScreenBox};
use gl_lib::text_rendering::{text_renderer, font};
use failure;
use deltatime;
use crate::components::container::ComponentContainer;


pub struct SdlGlWindow {
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
}




impl SdlGlWindow {

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



    /// Render components, Swap gl window, update internal delta time and handle sdl_events.
    /// Finish with clearing color_buffer_bit and depth_buffer_bit
    pub fn update<T>(&mut self, mut container: Option<(&mut ComponentContainer<T>, &mut T)>) {

        if let Some(cont) = container.as_mut() {
            self.render_components(cont.0);

        };

        self.window.gl_swap_window();
        self.deltatime.update();
        self.handle_events(container);

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

    pub fn text_renderer(&mut self) -> &mut text_renderer::TextRenderer {
        &mut self.text_renderer
    }


    pub fn setup_blend(&mut self) {
        self.text_renderer.setup_blend(&self.gl);
    }


    fn render_components<T>(&mut self, container: &mut ComponentContainer<T>) {
        let base_screen_box: ScreenBox = Default::default();
        for (comp, _) in container.components.values() {
            comp.render(&self.gl, &mut self.text_renderer, base_screen_box);
        }
    }



    fn handle_events<T>(&mut self, mut container: Option<(&mut ComponentContainer<T>, &mut T)>) {

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
                },
                _ => {}
            };


            if let Some(ref mut cont) = container {
                cont.0.handle_sdl_event(event.clone(), cont.1, self.viewport.w, self.viewport.h);
            }
            // TODO: Consider passing events consummed by components
            (self.event_handler)(event);
        }
    }

}


fn empty_handler( _:sdl2::event::Event)  {

}
