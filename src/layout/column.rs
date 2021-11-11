use crate::components::container::*;
use crate::layout::attributes::{*, Length::*};
use crate::layout::element::*;
use crate::layout::node::*;
use super::*;
use crate::layout::container::*;
use gl_lib::text_rendering::{ text_renderer::TextRenderer };
use std::marker;

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
            abs_height += c.final_height(available_space, text_renderer, OnFill::Shrink);
        }


        let child_spacing_count = self.children.len().max(1) - 1;
        let spacing = self.attributes().spacing;

        abs_height + child_spacing_count as f32 * spacing.y
    }


    fn content_width(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32 {
        let mut abs_width = 0.;
        for c in &self.children {
            abs_width += c.final_width(available_space, text_renderer, OnFill::Shrink);
        }

        abs_width
    }

    fn add_to_container(&self, container: &mut ComponentContainer<Message>, available_space: &RealizedSize, text_renderer: &TextRenderer) {
        if self.children.len() == 0 {
            return;
        }

        self.add_children_to_container(container, available_space, text_renderer);

        return;
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


impl<'a, Message> Container<Message> for Column<'a, Message>
where Message: 'a {

    fn children_height_info(&self, content_space: &RealizedSize, text_renderer: &TextRenderer) -> ChildrenAbsAndFill<Height> {

        let mut abs_height = 0.;
        let mut fill_count = 0;
        for c in &self.children {
            match c.attributes().height {
                Px(px) => { abs_height += px as f32; },
                FitContent => { abs_height += c.content_height(content_space, text_renderer); },
                Fill => { fill_count += 1; }
                FillPortion(x) => { fill_count += x; }
            }
        }
        ChildrenAbsAndFill::<Height> {
            abs_length: abs_height,
            fill_count: fill_count,
            child_count: self.children.len() as u32,
            _marker: marker::PhantomData::<Height>,
        }
    }


    fn calculate_child_spaces(&self, update_info: &UpdateInfo) -> Vec<RealizedSize> {

        let text_renderer = update_info.text_renderer;
        let spacing = update_info.spacing;
        let fill_count = update_info.height_info.fill_count;
        let content_space = update_info.content_space;
        let mut next_y = update_info.next.y;
        let dynamic_height = update_info.dynamic_height;


        let mut child_spaces = Vec::new();
        for child in &self.children {
            let mut child_space = *content_space;

            child_space.x = child.final_width(&child_space, text_renderer, OnFill::Expand);
            child_space.y = next_y;

            match child.attributes().height {
                Px(px) => {
                    child_space.height = px as f32;
                },
                FitContent => {
                    child_space.height = child.final_height(content_space, text_renderer, OnFill::Expand);
                },
                Fill => {
                    child_space.height = dynamic_height / fill_count as f32 ;
                },
                FillPortion(p) => {
                    child_space.height = (dynamic_height / fill_count as f32) * p as f32;
                },
            }

            next_y += child_space.height + spacing.y;
            child_spaces.push(child_space);
        }

        println!("Cols CHild Spaces {:#?}", child_spaces);

        child_spaces
    }

    fn children(&self) -> &Vec::<Node<Message>>  {
        &self.children
    }

}
