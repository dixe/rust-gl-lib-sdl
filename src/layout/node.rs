use crate::components::container::*;
use crate::layout::attributes::{*, Length::*, LengthAttrib::*};
use crate::layout::element::*;
use super::*;
use gl_lib::text_rendering::{ text_renderer::TextRenderer };


pub struct Node<'a, Message> {
    pub(crate) element: Box<dyn Element<Message> + 'a>,
}


impl<'a, Message> Element<Message> for Node<'a, Message> {
    fn attributes(&self) -> &Attributes {
        self.element.attributes()
    }

    fn attributes_mut(&mut self) -> &mut Attributes {
        self.element.attributes_mut()
    }


    fn add_to_container(&self, container: &mut ComponentContainer<Message>, available_space: &RealizedSize, text_renderer: &TextRenderer) {
        self.element.add_to_container(container, available_space, text_renderer);
    }


    fn final_height(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32 {
        self.element.final_height(available_space, text_renderer)
    }

    fn final_width(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32 {
        self.element.final_width(available_space, text_renderer)
    }

}
