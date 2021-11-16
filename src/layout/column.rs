use crate::layout::attributes::{*};
use crate::layout::element::*;
use crate::layout::node::*;
use super::*;
use gl_lib::text_rendering::{ text_renderer::TextRenderer };
use std::collections::VecDeque;
use std::fmt;

#[derive(Debug)]
pub struct Column<Message> where Message: fmt::Debug {
    children: VecDeque::<Node<Message>>,
    attributes: Attributes
}


impl<Message> Column<Message> where Message: fmt::Debug {

    pub fn new() -> Self {

        Column {
            children: VecDeque::new(),
            attributes: Default::default(),
        }
    }

    pub fn add<E>(mut self, el: E) -> Self
    where
        E: Into<Node<Message>> {
        self.children.push_back(el.into());
        self
    }
}


impl<Message> Element<Message> for Column<Message> where Message: fmt::Debug {

    fn name(&self) -> String {
        "Column".to_string()
    }

    fn height_children(&self) -> i32 {
        self.children.len() as i32
    }


    fn distribution_dir(&self) -> Direction {
        Direction::Y
    }


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

        let h = abs_height + child_spacing_count as f32 * spacing.y;

        h
    }


    fn content_width(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32 {
        let mut abs_width = 0.;
        for c in &self.children {
            abs_width += c.final_width(available_space, text_renderer, OnFill::Shrink);
        }

        abs_width
    }

    fn pop_children_front(&mut self) -> Option<Node<Message>> {
        self.children.pop_front()
    }
}

impl<Message: 'static> From<Column<Message>> for Node<Message>
where
    Message: fmt::Debug  {

    fn from(column: Column<Message>) -> Node<Message> {
        Box::new(column)
    }
}

/*
impl<Message> Container<Message> for Column<Message>
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

            child_space.width = child.final_width(&child_space, text_renderer, OnFill::Expand);
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

        child_spaces
    }

    fn children(&self) -> &Vec::<Node<Message>>  {
        &self.children.clone().into()
    }


}
*/
