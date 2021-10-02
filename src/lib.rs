use gl_lib::{na, gl, gl::viewport};
use failure;
use deltatime;

pub struct SdlGlWindow {
    sdl: sdl2::Sdl,
    gl: gl::Gl,
    window: sdl2::video::Window,
    _viewport: viewport::Viewport,
    _gl_context: sdl2::video::GLContext,
    deltatime: deltatime::Deltatime
}


impl SdlGlWindow {

    pub fn new(window_text: &str, width: u32, height: u32) -> Result<Self, failure::Error> {
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

        viewport.set_used(&gl);

        Ok(Self {
            gl,
            sdl,
            _viewport: viewport,
            window,
            _gl_context: gl_context,
            deltatime: Default::default(),
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

    /// Swap gl window and update internal delta time
    pub fn gl_swap_window_and_update(&mut self) {
        self.window.gl_swap_window();
        self.deltatime.update();
    }


    pub fn set_background_color(&self, color: na::Vector4::<f32>) {
        unsafe {
            self.gl.ClearColor(color.x, color.y, color.z, color.w);
        }
    }
}
