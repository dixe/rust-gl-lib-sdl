use gl_lib::{gl, na};
use gl_lib::text_rendering::{font};
use gl_lib_sdl as gls;
use failure;
use std::path::Path;

fn main() -> Result<(), failure::Error> {


    let width = 800;
    let height = 600;

    let font_path = Path::new("./assets/fonts/Arial.fnt");
    let font = font::Font::load_fnt_font(font_path).unwrap();
    let mut window = gls::window::SdlGlWindow::new("Text window", width, height, font).unwrap();


    let gl = &window.gl().clone();


    window.set_background_color(na::Vector4::new(0.9, 0.9, 0.9, 1.0));

    window.setup_blend();


    loop {

        unsafe {
            window.gl().Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }


        let time_ms =  1.0 / window.deltatime();
        window.text_renderer().render_text(&gl, &format!("Fps = {}", time_ms), -1.0, 1.0, 1.0);
        window.text_renderer().render_text(&gl, &TEST_TEXT, -1.0, 0.7, 1.0);

        window.update::<()>(None);

    }

}


const TEST_TEXT: &str = r"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec eu hendrerit velit. Vestibulum congue dui id laoreet viverra. Suspendisse ornare, velit in facilisis feugiat, elit orci viverra leo, sit amet consectetur nunc enim non mi. Curabitur sed efficitur lacus. Duis eu viverra nunc. Duis cursus maximus turpis. Aenean a convallis nulla.

Fusce lorem mauris, scelerisque vitae scelerisque quis, posuere viverra mauris. Ut convallis nec ex non tincidunt. Duis sapien est, vulputate vitae mattis eu, egestas vitae dolor. Suspendisse nec orci quis sem pharetra rhoncus. Donec egestas euismod ultrices. Nam euismod sem lorem, ut accumsan turpis congue sit amet. Integer ut tortor sit amet leo sagittis convallis sed quis augue. Quisque non magna placerat, consequat mi ut, hendrerit neque. Nunc sit amet fringilla lacus, non tempus lorem. Sed gravida consectetur nulla, sed maximus neque scelerisque eget. Donec quis risus metus. Duis eget dui id mi consectetur consequat eget in ex. Lorem ipsum dolor sit amet, consectetur adipiscing elit.

Vivamus pulvinar sapien at aliquam egestas. Etiam quis nisl vel velit euismod blandit. Maecenas rhoncus erat gravida mi facilisis, tempor egestas turpis venenatis. Duis ac lacus nec lectus lobortis porta. Aliquam aliquam, ex in feugiat cursus, risus dui maximus felis, eget mattis mi erat et odio. Aliquam tincidunt at urna vel placerat. Vestibulum bibendum, quam eu viverra eleifend, sem elit venenatis est, vel volutpat ante leo id metus. Vivamus luctus ligula sit amet vulputate aliquet. Nam et tincidunt nulla, non ultrices massa. Curabitur venenatis orci vel urna sollicitudin, eu sagittis eros fringilla. Aenean sed libero quis massa tempor imperdiet vitae quis massa. Sed faucibus interdum elit, eget lobortis leo fermentum nec. Nulla facilisi. Fusce ac tincidunt nisl. Donec fringilla leo a vestibulum varius.

Fusce fringilla augue in erat faucibus aliquam. Aliquam erat volutpat. Nam euismod augue sagittis libero rhoncus, auctor aliquam eros rhoncus. Sed a lacus quis lectus volutpat euismod. Donec non consectetur augue. Suspendisse ultricies aliquam turpis, quis convallis justo vehicula et. Maecenas molestie, sem non aliquam tempor, massa sapien ultrices mi, et tristique lorem odio in mauris. In tempus id magna eget vulputate.

Donec molestie in mauris et euismod. In posuere mauris dolor, sed aliquet orci accumsan ut. Curabitur velit purus, rhoncus ut dui sit amet, sollicitudin suscipit ipsum. Cras venenatis felis sed ex blandit convallis. Vivamus vestibulum mollis justo id rhoncus. Pellentesque fringilla maximus aliquet. Praesent luctus odio sed lacus bibendum mattis.
";
