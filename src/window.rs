use gl_lib::{self, na, gl, gl::viewport};
use gl_lib::text_rendering::{text_renderer, font};
use failure;
use deltatime;
use crate::components::base::{ClickRes, Component, ComponentEvent};


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
    components: Vec::<Box<dyn Component>>,
    text_renderer: text_renderer::TextRenderer,
    component_events: std::collections::VecDeque<InternalComponentEvent>
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
            components: Vec::new(),
            text_renderer,
            component_events: std::collections::VecDeque::new(),
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

    /// Render components, Swap gl window, update internal delta time and handle sdl_events
    pub fn gl_swap_window_and_update(&mut self) {
        self.render_components();
        self.window.gl_swap_window();
        self.deltatime.update();
        self.handle_events();
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

    fn render_components(&mut self) {
        for comp in &self.components {
            comp.render(&self.gl, &mut self.text_renderer, self.viewport.w, self.viewport.h);
        }
    }

    pub fn add_component(&mut self, comp: Box<dyn Component> ) {
        self.components.push(comp);
    }

    pub fn poll_component_events(&mut self) -> ComponentEventIterator {
        ComponentEventIterator {
            events: &mut self.component_events,
            components: &mut self.components,
        }

    }


    fn handle_events(&mut self) {

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
                Event::MouseButtonDown {mouse_btn, x, y, ..} => {
                    match mouse_btn {
                        sdl2::mouse::MouseButton::Left => {
                            // TODO: Iter through all that clicks and only store the one furthest up
                            // TODO: To avoid layered items triggering underneath
                            for (i,comp) in self.components.iter().enumerate() {
                                match comp.clicked(x as f32, y as f32) {
                                    ClickRes::Click(level) => {
                                        self.component_events.push_back(InternalComponentEvent {
                                            id: i,
                                            event: ComponentEvent::Clicked
                                        });

                                    },
                                    ClickRes::NoClick => {}
                                }
                            }

                        }
                        _ => {}

                    }
                },
                _ => {}

            };
            (self.event_handler)(event);
        }
    }
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ComponentEventIterator<'a> {

    events: &'a mut std::collections::VecDeque<InternalComponentEvent>,
    components: &'a mut Vec::<Box<dyn Component>>,


}



impl<'a> Iterator for ComponentEventIterator<'a> {

    type Item = ComponentWithEvent<'a>;

    fn next<'b>(&mut self) -> Option<Self::Item> {

        let internal_event = match self.events.pop_front() {
            None => return None,
            Some(e) => e,
        };

        let component = &'a mut self.components[internal_event.id];
        let cwe = ComponentWithEvent {
            component,
            event: internal_event.event,
        };

        Some(cwe)

    }

}


pub struct ComponentWithEvent<'a> {
    component: &'a mut Box<dyn Component>,
    event: ComponentEvent
}

#[derive(Debug,Clone,Copy)]
struct InternalComponentEvent {
    id: usize,
    event: ComponentEvent
}


fn empty_handler( _:sdl2::event::Event)  {

}
