use crate::components::container::*;
use crate::layout::attributes::{*, Length::*};
use crate::layout::element::*;
use crate::layout::node::*;
use super::*;
use gl_lib::text_rendering::{ text_renderer::TextRenderer };


pub struct Column<'a, Message> {
    children: Vec::<Node<'a, Message>>,
    attributes: Attributes
}


impl<'a, Message> Column<'a, Message> {


    pub fn new() -> Self {

        Column {
            children: Vec::new(),
            attributes: Default::default(),
        }
    }

    pub fn add<E>(mut self, el: E) -> Self
    where
        E: Into<Node<'a, Message>> {
        self.children.push(el.into());
        self
    }
}


impl<'a, Message> Element<Message> for Column<'a, Message> {

    fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }

    fn final_height(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32 {
        let mut abs_height = 0.;
        for c in &self.children {
            abs_height += c.final_height(available_space, text_renderer);
        }

        abs_height
    }


    fn final_width(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32 {
        let mut abs_width = 0.;
        for c in &self.children {
            abs_width += c.final_width(available_space, text_renderer);
        }

        abs_width
    }

    fn add_to_container(&self, container: &mut ComponentContainer<Message>, available_space: &RealizedSize, text_renderer: &TextRenderer) {

        // loop over children width and calc abos space used. That is px(u32) and FitContent
        let mut abs_height = 0.;
        let mut fill_count = 0;
        for c in &self.children {
            println!("Child h={:?}", c.attributes().height );
            match c.attributes().height {
                Px(px) => { abs_height += px as f32; },
                FitContent => {
                    abs_height += c.final_height(available_space, text_renderer);
                },
                Fill => { fill_count += 1; }
                FillPortion(x) => { fill_count += x; }
            }
        }


        if self.children.len() == 0 {
            return;
        }

        let attribs = self.attributes();

        let padding = attribs.padding;
        let spacing = attribs.spacing;

        let mut next_y = available_space.y + padding.top;


        let content_width = available_space.width - padding.left - padding.right;
        let content_height = available_space.height - padding.bottom - padding.top;


        let dynamic_height = f32::max(0.0, content_height - abs_height) - (self.children.len() - 1) as f32 * spacing.y;

        for c in &self.children {
            let mut child_space = *available_space;
            child_space.width = content_width;
            child_space.height = 0.0;

            child_space.x = available_space.x + padding.left;
            child_space.y = next_y;
            match c.attributes().height {
                Px(px) => {
                    child_space.height = px as f32;
                },
                FitContent => {
                    child_space.height = c.final_height(available_space, text_renderer);
                },
                Fill => {
                    child_space.height = dynamic_height / fill_count as f32 ;
                },
                _ => unimplemented!(),
            }

            next_y += child_space.height + spacing.y;

            c.add_to_container(container, &child_space, text_renderer);
        }
    }
}

impl<'a, Message> From<Column<'a, Message>> for Node<'a, Message>
where
    Message: 'a {

    fn from(column: Column<'a, Message>) -> Node<'a, Message> {
        Node {
            element: Box::new(column)
        }
    }

}
