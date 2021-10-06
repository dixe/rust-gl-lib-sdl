use crate::components::base::{ClickRes, Component, ComponentEvent};



pub enum HandleRes {
    Consumed,
    Unused
}

pub struct ComponentContainer {

    next_id: usize,
    pub components: std::collections::HashMap<usize, Box<dyn Component>>,
    component_events: std::collections::VecDeque<InternalComponentEvent>
}


impl ComponentContainer {

    pub fn new() -> Self {
        Self {
            next_id: 1,
            components: std::collections::HashMap::new(),
            component_events: std::collections::VecDeque::new()
        }
    }

    pub fn add_component(&mut self, component: Box<dyn Component>) -> usize {
        let id = self.next_id;
        self.components.insert(id, component);
        self.next_id = self.next_id + 1;
        id
    }


    pub fn handle_events(&mut self, event_handlers: std::collections::HashMap<usize, Box<dyn Fn(sdl2::event::Event)>>) {

        for event in &self.component_events {

            println!("{:?}", event);

        }
    }


    pub fn handle_sdl_event(&mut self, event: sdl2::event::Event) -> HandleRes {
        use sdl2::event::Event;

        let mut res = HandleRes::Unused;

        match event {
            Event::MouseButtonDown {mouse_btn, x, y, ..} => {
                match mouse_btn {
                    sdl2::mouse::MouseButton::Left => {
                        // TODO: Iter through all that clicks and only store the one furthest up
                        // TODO: To avoid layered items triggering underneath
                        for (i,(key, comp)) in self.components.iter().enumerate() {

                            match comp.clicked(x as f32, y as f32) {
                                ClickRes::Click(level) => {
                                    res = HandleRes::Consumed;
                                    self.component_events.push_back(InternalComponentEvent{
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

        res
    }
}

#[derive(Debug,Clone,Copy)]
struct InternalComponentEvent {
    id: usize,
    event: ComponentEvent
}
