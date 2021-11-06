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

        // loop over children width and calc abos space used. That is px(u32) and FitContent
        let mut abs_width = 0.;
        let mut fill_count = 0;
        for c in &self.children {
            match c.attributes().width {
                Px(px) => { abs_width += px as f32; },
                FitContent => { abs_width += c.final_width(available_space, text_renderer); },
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

        let mut next_x = available_space.x + padding.left;

        let content_width = available_space.width - padding.left - padding.right;
        let content_height = available_space.height - padding.bottom - padding.top;

        let dynamic_width = f32::max(0.0, content_width - abs_width) - (self.children.len() -1 ) as f32 * spacing.x;

        let mut child_spaces = Vec::new();
        for c in &self.children {
            let mut child_space = *available_space;
            child_space.width = 0.0;
            child_space.height = content_height;


            child_space.x = next_x;
            child_space.y = available_space.y + padding.top;


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



        let mut center_elements = 0;
        let mut center_elements_width = 0.0;

        let mut center_elements_left = None;
        let mut center_elements_right = 0.0;


        for i in 0..self.children.len() {
            let c = &self.children[i];
            let cs = &mut child_spaces[i];


            match c.attributes().align.x {
                AlignmentX::Center => {
                    match center_elements_left {
                        None => {
                            center_elements_left = Some(cs.x);
                        },
                        _ => {}// Already set we a previous element
                    }

                    center_elements_right = cs.x + cs.width;
                    center_elements += 1;

                },
                AlignmentX::Right => { break }, // when we first align to the right, centering does nothing after
                _ => {}
            }
        }

        let mut center_elements_width = match center_elements_left {
            None => None,
            Some(left) => Some(center_elements_right - left)
        };

        let mut unused_x = content_width - (next_x - spacing.x);
        let mut x_offset = 0.0;

        for i in 0..self.children.len() {
            let c = &self.children[i];
            let cs = &mut child_spaces[i];


            match c.attributes().align.x {
                AlignmentX::Left => {}, //default is left, do nothing},
                AlignmentX::Center => {
                    match center_elements_width {
                        None => {},
                        Some(offset) => {
                            let desired_x = content_width/2.0 - offset/2.0 - center_elements_left.unwrap();
                            let new_offset = f32::max(0.0, desired_x);
                            x_offset += new_offset;
                            unused_x -= new_offset;
                            center_elements_width = None;
                        }
                    }
                },
                AlignmentX::Right => {
                    // take all remaning space to the right and offset by that
                    x_offset += f32::max(0.0, unused_x);
                    unused_x = 0.0;
                },

            }


            cs.x += x_offset;
        }

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
