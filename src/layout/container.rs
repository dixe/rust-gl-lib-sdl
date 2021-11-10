use crate::components::container::*;
use crate::layout::attributes::{*};
use crate::layout::element::*;
use super::*;
use crate::layout::node::*;
use gl_lib::text_rendering::{ text_renderer::TextRenderer };
use std::marker;


pub trait Container<Message>: Element<Message> {


    fn children_height_info(&self, content_space: &RealizedSize, _text_renderer: &TextRenderer) -> ChildrenAbsAndFill<Height> {
        ChildrenAbsAndFill::<Height> {
            abs_length: content_space.height,
            fill_count: 0,
            child_count: 0,
            _marker: marker::PhantomData::<Height>,
        }
    }

    fn children_width_info(&self, content_space: &RealizedSize, _text_renderer: &TextRenderer) -> ChildrenAbsAndFill<Width> {
        ChildrenAbsAndFill::<Width> {
            abs_length: content_space.width,
            fill_count: 0,
            child_count: 0,
            _marker: marker::PhantomData::<Width>,
        }
    }

    fn calculate_child_spaces(&self, update_info: &mut UpdateInfo) -> ChildSpaceInfo;


    fn children(&self) -> &Vec::<Node<Message>>;


    fn add_children_to_container(&self, container: &mut ComponentContainer<Message>, available_space: &RealizedSize, text_renderer: &TextRenderer) {

        // loop over children width and calc abs space used. That is px(u32) and FitContent

        let attribs = self.attributes();
        let padding = attribs.padding;
        let spacing = attribs.spacing;


        let content_space = &RealizedSize {
            x: available_space.x + padding.left,
            y: available_space.y + padding.top,
            width: self.final_width(available_space, text_renderer) - padding.right - padding.left,
            height: self.final_height(available_space, text_renderer) - padding.bottom - padding.top,
        };

        println!("{:?}",content_space);

        let height_info = self.children_height_info(content_space, text_renderer);

        let width_info = self.children_width_info(content_space, text_renderer);


        let dynamic_width = f32::max(0.0, content_space.width - width_info.abs_length) - (u32::max(1, width_info.child_count) - 1) as f32 * spacing.x;
        let dynamic_height = f32::max(0.0, content_space.height - height_info.abs_length) - (u32::max(1, height_info.child_count) - 1) as f32 * spacing.y;

        let mut child_spaces_info = self.calculate_child_spaces(&mut UpdateInfo {
            height_info: &height_info,
            width_info: &width_info,
            content_space: &content_space,
            spacing: &spacing,
            next:  NextStart {
                x: content_space.x,
                y: content_space.y,
            },
            text_renderer,
            dynamic_width,
            dynamic_height
        });


        let unused_x = f32::max(0.0, content_space.width + content_space.x - (child_spaces_info.next.x - spacing.x));
        let unused_y = f32::max(0.0, content_space.height + content_space.y - (child_spaces_info.next.y - spacing.y));

        align_child_spaces(self.children(), &mut child_spaces_info.child_spaces, content_space.width, unused_x, unused_y);


        for i in 0..self.children().len() {
            self.children()[i].add_to_container(container, &child_spaces_info.child_spaces[i], text_renderer);
        }
    }
}


pub struct Width {}
pub struct Height {}

#[derive(Debug, Clone, Copy)]
pub struct ChildrenAbsAndFill<T> {
    pub abs_length: f32,
    pub fill_count: u32,
    pub child_count: u32,
    pub _marker: marker::PhantomData<T>,
}


#[derive(Debug, Clone, Copy)]
pub struct NextStart {
    pub x: f32,
    pub y: f32
}

#[derive(Debug, Clone)]
pub struct ChildSpaceInfo {
    pub next: NextStart,
    pub child_spaces: Vec<RealizedSize>,
}


pub struct UpdateInfo<'a> {
    pub content_space: &'a RealizedSize,
    pub width_info: &'a ChildrenAbsAndFill<Width>,
    pub height_info: &'a ChildrenAbsAndFill<Height>,
    pub text_renderer: &'a TextRenderer,
    pub next: NextStart,
    pub spacing: &'a Spacing,

    pub dynamic_width: f32,
    pub dynamic_height: f32
}
