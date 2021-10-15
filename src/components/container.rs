use crate::components::base::{OnTop, Component, ComponentEvent};
use crate::window;


pub enum HandleRes {
    Consumed,
    Unused
}

pub type Handler<T> = fn(ComponentEvent, &mut Component,  &mut T, &window::WindowComponentAccess);
type ComponentEvents = std::collections::VecDeque<InternalComponentEvent>;

pub type Components<T> = std::collections::HashMap<usize, (Component, Handler<T>)>;

pub struct ComponentContainer<T> {
    next_id: usize,
    pub components: Components<T>,
    component_events: ComponentEvents,
}


impl<T> ComponentContainer<T> {

    pub fn new() -> Self {

        Self {
            next_id: 1,
            components: std::collections::HashMap::new(),
            component_events: std::collections::VecDeque::new(),
        }
    }

    pub fn add_component(&mut self, component: Component, handler: Handler<T>)  -> usize {
        let id = self.next_id;
        self.components.insert(id, (component, handler));
        self.next_id += 1;
        id
    }


    fn handle_events(&mut self, state: &mut T, window_access: &window::WindowComponentAccess) {

        let mut popped_event = self.component_events.pop_front();
        while let Some(event) = popped_event {
            let c = self.components.get_mut(&event.id);


            if let Some(data) = c {
                let comp = &mut data.0;

                let _ = match event.event {
                    ComponentEvent::Hover => {
                        comp.base.hover = true;
                    },
                    ComponentEvent::HoverEnd => {
                        comp.base.hover = false;
                    },
                    _ => {},
                };

                data.1(event.event, comp, state, window_access);

            }

            popped_event = self.component_events.pop_front();
        }
    }


    pub fn handle_sdl_event(&mut self, event: sdl2::event::Event, state: &mut T, window_access: &window::WindowComponentAccess) -> HandleRes {
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


        self.handle_events(state, window_access);
        res
    }
}

fn hover_no_match(key: usize, component: &Component, component_events: &mut ComponentEvents) {

    if component.base.hover {
        component_events.push_back(InternalComponentEvent{
            id: key,
            event: ComponentEvent::HoverEnd
        });
    }
}

type NoMatchFn = fn (key: usize, component: &Component, component_events: &mut ComponentEvents);

fn push_component_event<T>(event: ComponentEvent, event_x: f32, event_y: f32, components: &Components<T>, component_events: &mut ComponentEvents, no_match: Option<NoMatchFn>) -> HandleRes {

    let mut res = HandleRes::Unused;
    // TODO: Make this into a functions that takes the event to push
    // TODO: This is repeated and will get complicated
    for (key, (comp, _)) in components {

        match comp.on_top(event_x, event_y) {
            OnTop::OnTop(_level) => {
                res = HandleRes::Consumed;
                component_events.push_back(InternalComponentEvent{
                    id: *key,
                    event,
                });

            },
            OnTop::No => {
                if let Some(no_match_fn) = no_match {
                    no_match_fn(*key, comp,  component_events);
                }
            }
        }
    }

    res
}


#[derive(Debug,Clone,Copy)]
struct InternalComponentEvent {
    id: usize,
    event: ComponentEvent
}
