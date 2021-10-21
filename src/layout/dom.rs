
type ComponentEvents = std::collections::VecDeque<InternalComponentEvent>;



#[derive(Debug,Clone,Copy)]
struct Internalevent {
    id: usize,
    event: Event
}


#[derive(Debug,Clone,Copy)]
pub enum Event {
    Clicked,
    Hover,
    HoverEnd,
}



pub struct Dom {
    pub root: Box::(Element),
    dom_events: std::collections::VecDeque<InternalComponentEvent>,
}


impl Dom {


    fn handle_events(&mut self, window_access: &window::WindowComponentAccess) {

        let mut popped_event = self.component_events.pop_front();
        while let Some(event) = popped_event {
            let c = self.components.get_mut(&event.id);


            if let Some(mut comp) = c {
                let _ = match event.event {
                    ComponentEvent::Hover => {
                        comp.base.hover = true;
                    },
                    ComponentEvent::HoverEnd => {
                        comp.base.hover = false;
                    },
                    _ => {},
                };

                // TODO: call the on_event function for the element
                println!("Pressed {:#?}", event);
                //data.1(event.event, comp, state, window_access);

            }

            popped_event = self.component_events.pop_front();
        }
    }



    pub fn handle_sdl_event(&mut self, event: sdl2::event::Event, window_access: &window::WindowComponentAccess) -> HandleRes {
        use sdl2::event::Event;

        let mut res = HandleRes::Unused;

        match event {
            Event::MouseButtonDown {mouse_btn, x, y, ..} => {
                match mouse_btn {
                    sdl2::mouse::MouseButton::Left => {
                        res = push_component_event(ComponentEvent::Clicked, x as f32, y as f32, &self.components, &mut self.component_events, None);
                    },
                    sdl2::mouse::MouseButton::Right => {

                    },
                    _ => {}

                }
            },
            Event::MouseMotion{x, y, .. }  => {
                res = push_component_event(ComponentEvent::Hover,  x as f32, y as f32, &self.components, &mut self.component_events, Some(hover_no_match));
            }
            _ => {}

        };


        self.handle_events(window_access);
        res
    }

}
