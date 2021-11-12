use crate::components::container::*;
use crate::layout::attributes::Attributes;
use crate::layout::element::*;
use super::*;
use gl_lib::text_rendering::{ text_renderer::TextRenderer };
use std::fmt;

#[derive(Debug)]
pub struct Node<Message> where Message: fmt::Debug {
    pub(crate) element: Box<dyn Element<Message>>,
}

impl<Message> Element<Message> for Node<Message> where Message: fmt::Debug {

    fn name(&self) -> &str {
        self.element.name()
    }

    fn attributes(&self) -> &Attributes {
        self.element.attributes()
    }

    fn attributes_mut(&mut self) -> &mut Attributes {
        self.element.attributes_mut()
    }


    fn add_to_container(&self, container: &mut ComponentContainer<Message>, available_space: &RealizedSize, text_renderer: &TextRenderer) {
        self.element.add_to_container(container, available_space, text_renderer);
    }


    fn content_height(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32 {
        self.element.content_height(available_space, text_renderer)
    }

    fn content_width(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32 {
        self.element.content_width(available_space, text_renderer)
    }

    fn pop_children_front(&mut self) -> Option<Node<Message>> {
        self.element.pop_children_front()
    }

}
