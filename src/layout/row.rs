use crate::components::container::*;
use crate::layout::attributes::{*, Length::*};
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

    fn content_height(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32 {

        let mut abs_height = 0.;

        for c in &self.children {
            abs_height = f32::max(abs_height, c.content_height(available_space, text_renderer))
        }

        abs_height
    }


    fn content_width(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32 {
        let mut abs_width = 0.;
        for c in &self.children {
            abs_width += c.content_width(available_space, text_renderer);
        }

        abs_width
    }

    fn add_to_container(&self, container: &mut ComponentContainer<Message>, available_space: &RealizedSize, text_renderer: &TextRenderer) {

        if self.children.len() == 0 {
            return;
        }


        // loop over children width and calc abs space used. That is px(u32) and FitContent
        let mut abs_width = 0.;
        let mut fill_count = 0;

        let attribs = self.attributes();
        let padding = attribs.padding;
        let spacing = attribs.spacing;


        let content_space = &RealizedSize {
            x: available_space.x + padding.left,
            y: available_space.y + padding.top,
            width: self.final_width(available_space, text_renderer) - padding.right - padding.left,
            height: self.final_height(available_space, text_renderer) - padding.bottom - padding.top,
        };


        for c in &self.children {
            match c.attributes().width {
                Px(px) => { abs_width += px as f32; },
                FitContent => { abs_width += c.final_width(content_space, text_renderer); },
                Fill => { fill_count += 1; }
                FillPortion(x) => { fill_count += x; }
            }
        }



        let mut next_x = available_space.x;

        let dynamic_width = f32::max(0.0, content_space.width - abs_width) - (self.children.len() -1 ) as f32 * spacing.x;

        let mut child_spaces = Vec::new();
        for c in &self.children {
            let mut child_space = *content_space;
            child_space.x = next_x;

            match c.attributes().width {
                Px(px) => {
                    child_space.width = px as f32;
                },
                FitContent => {
                    child_space.width = c.final_width(available_space, text_renderer);
                },
                Fill => {
                    child_space.width = dynamic_width / fill_count as f32 ;
                },
                FillPortion(p) => {
                    child_space.width = (dynamic_width / fill_count as f32) * p as f32;
                },
            }

            next_x += child_space.width + spacing.x;

            child_spaces.push(child_space);

        }

        next_x -= spacing.x;

        // TODO: Make this generic for childspaces on element. To work on both X and Y
        let unused_x = f32::max(0.0, content_space.width - (next_x - spacing.x));
        let unused_y = 0.0;

        align_child_spaces(&self.children, &mut child_spaces, content_space.width, unused_x, unused_y);

        for i in 0..self.children.len() {
            self.children[i].add_to_container(container, &child_spaces[i], text_renderer);
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
