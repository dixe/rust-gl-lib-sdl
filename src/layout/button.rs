use super::*;
use crate::components::base::*;
use crate::components::button as comp_btn;
use crate::components::container::*;
use crate::layout::attributes::{Length, Attributes};
use crate::layout::element::*;
use gl_lib::text_rendering::{ text_renderer::TextRenderer };
use gl_lib::gl;
use crate::layout::node::*;


#[derive(Clone, Debug)]
pub struct Button<Message> {
    btn: comp_btn::Button<Message>,
    attributes: Attributes,
    on_click_msg: Option<Message>
}

impl<Message> Button<Message> where Message: Clone {
    pub fn new(gl: &gl::Gl, content: &str, msg: Option<Message>) -> Self {

        let btn = comp_btn::Button::new(gl, content, msg.clone());

        Self {
            btn,
            attributes: Default::default(),
            on_click_msg: msg
        }
    }
}


impl<Message> Element<Message> for Button<Message> where Message: Clone {

    fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }

    fn final_height(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32 {

        match self.attributes().height {
            Length::Px(px) => {
                px as f32
            },
            Length::FitContent => {
                text_renderer.render_box(&self.btn.content, 1.0).pixel_h
            },
            _ => available_space.height,
        }
    }

    fn final_width(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32 {

        let w = match self.attributes().width {

            Length::Px(px) => {
                px as f32
            },
            Length::FitContent => {
                text_renderer.render_box(&self.btn.content, 1.0).pixel_w

            },
            _ => available_space.width,
        };

        w
    }

    fn add_to_container(&self, container: &mut ComponentContainer<Message>, available_space: &RealizedSize, text_renderer: &TextRenderer) {

        // Update out base component to have the correct size

        let mut new_comp: Component<Message> = self.btn.clone().into();


        new_comp.base.coords.x = available_space.x;
        new_comp.base.coords.y = available_space.y;

        let attributes = self.attributes();



        let max_h = attributes.width_contraint.max(available_space.width);

        let final_width = self.final_width(available_space, text_renderer);
        new_comp.base.set_width(final_width, max_h);


        let max_h = attributes.height_contraint.max(available_space.height);

        let final_height = self.final_height(available_space, text_renderer);
        new_comp.base.set_height(final_height, max_h);

        container.add_component(new_comp.into());
    }
}


impl<'a, Message> From<Button<Message>> for Node<'a, Message>
where
    Message: Clone + 'a {

    fn from(button: Button<Message>) -> Node<'a, Message> {
        Node {
            element: Box::new(button)
        }
    }

}
