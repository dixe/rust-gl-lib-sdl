use crate::components::container::*;
use crate::layout::attributes::{*, Length::*, LengthAttrib::*};
use crate::layout::element::*;
use super::*;
use crate::layout::node::*;
use gl_lib::text_rendering::{ text_renderer::TextRenderer };

pub struct Row<'a, Message> {
    children: Vec::<Node<'a, Message>>,
    attributes: Attributes
}


impl<'a, Message> Row<'a, Message> {


    pub fn new() -> Self {
        Self {
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


impl<'a, Message> Element<Message> for Row<'a, Message> {

    fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }

    fn final_height(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32 {

        let mut abs_height = 0.;

        for c in &self.children {
            abs_height = f32::max(abs_height, c.final_height(available_space, text_renderer))
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
        let mut abs_width = 0.;
        let mut fill_count = 0;
        for c in &self.children {
            match c.attributes().width {
                No(l) => {
                    match l {
                        Px(px) => { abs_width += px; },
                        FitContent => unimplemented!(),
                        _ => { fill_count += 1; }

                    }
                },
                _ => unimplemented!()
            }
        }


        if self.children.len() == 0 {
            return;
        }

        let attribs = self.attributes();

        let padding = attribs.padding;
        let spacing = attribs.spacing;

        let mut next_x = available_space.x + padding.left;

        let content_width = available_space.width - padding.left - padding.right;
        let content_height = available_space.height - padding.bottom - padding.top;

        let dynamic_width = f32::max(0.0, content_width - abs_width) - (self.children.len() -1 ) as f32 * spacing.y;

        for c in &self.children {
            let mut child_space = *available_space;
            child_space.width = 0.0;
            child_space.height = content_height;


            child_space.x = next_x;
            child_space.y = available_space.y + padding.top;


            match c.attributes().width {
                No(l) => {
                    match l {
                        Px(px) => {
                            child_space.width = px as f32;
                        },
                        FitContent => {
                            child_space.width = c.final_width(available_space, text_renderer);
                        },
                        Fill => {
                            child_space.width = dynamic_width / fill_count as f32 ;
                        },
                        _ => unimplemented!(),

                    }
                },
                _ => unimplemented!()
            }

            next_x += child_space.width + spacing.x;

            c.add_to_container(container, &child_space, text_renderer);
        }
    }
}



impl<'a, Message> From<Row<'a, Message>> for Node<'a, Message>
where
    Message: 'a {


    fn from(row: Row<'a, Message>) -> Node<'a, Message> {
        Node {
            element: Box::new(row)
        }
    }
}
