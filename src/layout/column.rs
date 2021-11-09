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

    fn content_height(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32 {
        let mut abs_height = 0.;
        for c in &self.children {
            abs_height += c.content_height(available_space, text_renderer);
        }

        let spacing = self.attributes().spacing;

        abs_height + self.children.len() as f32 * spacing.y
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
        let mut abs_height = 0.;
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
            match c.attributes().height {
                Px(px) => { abs_height += px as f32; },
                FitContent => { abs_height += c.content_height(content_space, text_renderer); },
                Fill => { fill_count += 1; }
                FillPortion(x) => { fill_count += x; }
            }
        }







        let mut next_y = content_space.y;

        let dynamic_height = f32::max(0.0, content_space.height - abs_height) - (self.children.len() - 1) as f32 * spacing.y;
        let mut child_spaces = Vec::new();
        for c in &self.children {
            let mut child_space = *content_space;
            child_space.y = next_y;

            match c.attributes().height {
                Px(px) => {
                    child_space.height = px as f32;
                },
                FitContent => {
                    child_space.height = c.final_height(content_space, text_renderer);
                },
                Fill => {
                    child_space.height = dynamic_height / fill_count as f32 ;
                },
                _ => unimplemented!(),
            }

            next_y += child_space.height + spacing.y;
            child_spaces.push(child_space);
        }

        // TODO: Make this generic for childspaces on element. To work on both X and Y
        let unused_y = f32::max(0.0, content_space.height - (next_y - spacing.y));
        let unused_x = 0.0;

        for i in 0..self.children.len() {
            self.children[i].add_to_container(container, &child_spaces[i], text_renderer);
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
