use crate::layout::attributes::{*};
use crate::layout::element::*;
use super::*;
use crate::layout::node::*;
use gl_lib::text_rendering::{ text_renderer::TextRenderer };
use std::collections::VecDeque;
use std::fmt;

#[derive(Debug)]
pub struct Row<Message> where Message: fmt::Debug {
    children: VecDeque::<Node<Message>>,
    attributes: Attributes
}


impl<Message> Row<Message> where Message: fmt::Debug {


    pub fn new() -> Self {
        Self {
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


impl<Message> Element<Message> for Row<Message> where Message: fmt::Debug {

    fn name(&self) -> String {
        "row".to_string()
    }
    fn width_children(&self) -> i32 {
        self.children.len() as i32
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
            abs_height = f32::max(abs_height, c.final_height(available_space, text_renderer, OnFill::Shrink))
        }

        abs_height
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



impl<Message: 'static> From<Row<Message>> for Node<Message>
where
    Message: fmt::Debug  {


    fn from(row: Row<Message>) -> Node<Message> {
        Box::new(row)
    }
}


/*
impl<Message> Container<Message> for Row<Message>
where Message: 'a {

fn children_width_info(&self, content_space: &RealizedSize, text_renderer: &TextRenderer) -> ChildrenAbsAndFill<Width> {

let mut abs_width = 0.;
let mut fill_count = 0;
for c in &self.children {
match c.attributes().width {
Px(px) => { abs_width += px as f32; },
FitContent => { abs_width += c.final_width(content_space, text_renderer, OnFill::Expand); },
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


    fn calculate_child_spaces(&self, update_info: & UpdateInfo) -> Vec<RealizedSize> {

        let text_renderer = update_info.text_renderer;
        let spacing = update_info.spacing;
        let fill_count = update_info.width_info.fill_count;
        let content_space = update_info.content_space;
        let mut next_x = update_info.next.x;
        let dynamic_width = update_info.dynamic_width;

        let mut child_spaces = Vec::new();


        for child in &self.children {

            let mut child_space = *content_space;
            child_space.height = child.final_height(&child_space, text_renderer, OnFill::Expand);

            match child.attributes().width {
                Px(px) => {
                    child_space.width = px as f32;
                },
                FitContent => {
                    child_space.width = child.final_width(&child_space, text_renderer, OnFill::Expand);
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


        println!("ROW CHILD SPACES {:#?}", child_spaces);
        child_spaces
    }

    fn children(&self) -> &Vec::<Node<Message>>  {
        &self.children.into()
    }

}
*/
