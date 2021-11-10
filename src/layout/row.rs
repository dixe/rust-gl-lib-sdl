use crate::components::container::*;
use crate::layout::attributes::{*, Length::*};
use crate::layout::element::*;
use super::*;
use crate::layout::node::*;
use crate::layout::container::*;
use gl_lib::text_rendering::{ text_renderer::TextRenderer };
use std::marker;


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

        self.add_children_to_container(container, available_space, text_renderer);
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


impl<'a, Message> Container<Message> for Row<'a, Message>
where Message: 'a {

    fn children_width_info(&self, content_space: &RealizedSize, text_renderer: &TextRenderer) -> ChildrenAbsAndFill<Width> {

        let mut abs_width = 0.;
        let mut fill_count = 0;
        for c in &self.children {
            match c.attributes().width {
                Px(px) => { abs_width += px as f32; },
                FitContent => { abs_width += c.final_width(content_space, text_renderer); },
                Fill => { fill_count += 1; }
                FillPortion(x) => { fill_count += x; }
            }
        }
        ChildrenAbsAndFill::<Width> {
            abs_length: abs_width,
            fill_count: fill_count,
            child_count: self.children.len() as u32,
            _marker: marker::PhantomData::<Width>,
        }
    }


    fn calculate_child_spaces(&self, update_info: &mut UpdateInfo) -> ChildSpaceInfo {

        let text_renderer = update_info.text_renderer;
        let spacing = update_info.spacing;
        let fill_count = update_info.width_info.fill_count;
        let content_space = update_info.content_space;
        let mut next_x = update_info.next.x;
        let dynamic_width = update_info.dynamic_width;

        let mut child_spaces = Vec::new();
        for child in &self.children {

            let mut child_space = *content_space;
            child_space.x = next_x;

            match child.attributes().width {
                Px(px) => {
                    child_space.width = px as f32;
                },
                FitContent => {
                    child_space.width = child.final_width(&child_space, text_renderer);
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


        ChildSpaceInfo {
            child_spaces,
            next: NextStart {
                x: next_x,
                y: update_info.next.y
            }
        }

    }

    fn children(&self) -> &Vec::<Node<Message>>  {
        &self.children
    }

}
