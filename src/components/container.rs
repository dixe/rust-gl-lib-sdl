use crate::components::base::{ClickRes, Component, ComponentEvent};
use std::borrow::BorrowMut;


pub enum HandleRes {
    Consumed,
    Unused
}

pub type Handler<T> = fn(ComponentEvent, &mut dyn Component,  &mut T);

pub struct ComponentContainer<T> {
    next_id: usize,
    pub components: std::collections::HashMap<usize, (Box<dyn Component>, Handler<T>)>,
    component_events: std::collections::VecDeque<InternalComponentEvent>,

}


impl<T> ComponentContainer<T> {

    pub fn new() -> Self {
        Self {
            next_id: 1,
            components: std::collections::HashMap::new(),
            component_events: std::collections::VecDeque::new()
        }
    }

    pub fn add_component(&mut self, component: Box<dyn Component>, handler: Handler<T>)  -> usize {
        let id = self.next_id;
        self.components.insert(id, (component, handler));
        self.next_id = self.next_id + 1;
        id
    }


    pub fn handle_events(&mut self, state: &mut T) {

        let mut popped_event = self.component_events.pop_front();
        while let Some(event) = popped_event {
            let c =self.components.get_mut(&event.id);


            if let Some(data) = c {
                let comp = (*data.0).borrow_mut();
                data.1(event.event, comp, state);

            }

            popped_event = self.component_events.pop_front();
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
                        for (key, (comp, _)) in &self.components {

                            match comp.clicked(x as f32, y as f32) {
                                ClickRes::Click(_level) => {
                                    res = HandleRes::Consumed;
                                    self.component_events.push_back(InternalComponentEvent{
                                        id: *key,
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
