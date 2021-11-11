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

    fn calculate_child_spaces(&self, update_info: &UpdateInfo) -> Vec<RealizedSize>;


    fn children(&self) -> &Vec::<Node<Message>>;


    fn add_children_to_container(&self, container: &mut ComponentContainer<Message>, available_space: &RealizedSize, text_renderer: &TextRenderer) {

        // loop over children width and calc abs space used. That is px(u32) and FitContent

        let attribs = self.attributes();
        let padding = attribs.padding;
        let spacing = attribs.spacing;

        let content_space = &RealizedSize {
            x: available_space.x + padding.left,
            y: available_space.y + padding.top,
            width: self.final_width(available_space, text_renderer, OnFill::Expand) - padding.right - padding.left,
            height: self.final_height(available_space, text_renderer, OnFill::Expand) - padding.bottom - padding.top,
        };

        let height_info = self.children_height_info(content_space, text_renderer);

        let width_info = self.children_width_info(content_space, text_renderer);

        let x_spacing = (width_info.child_count.max(1) - 1) as f32 * spacing.x;
        let y_spacing = (height_info.child_count.max(1) - 1) as f32 * spacing.y;

        let dynamic_width = f32::max(0.0, content_space.width - width_info.abs_length) - x_spacing;
        let dynamic_height = f32::max(0.0, content_space.height - height_info.abs_length) - y_spacing;

        let mut child_spaces = self.calculate_child_spaces(&mut UpdateInfo {
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



        let mut children_width = 0.0;
        let mut children_height = 0.0;


        for cs in &child_spaces {

            children_width += cs.width;
            children_height += cs.height;
        }


        children_width += x_spacing;
        children_height += y_spacing;


        /*
        println!("\n\n");
        println!("OLD nexStart = {:?}",  child_spaces_info.next);
        println!("NEW nexStart = {:?}",  NextStart {x: next_x, y: next_y});
        println!("\n\n");
         */
        align_child_spaces(self.children(), &mut child_spaces, &UsedSpace { w: children_width, h: children_height }, content_space);


        for i in 0..self.children().len() {
            self.children()[i].add_to_container(container, &child_spaces[i], text_renderer);
        }
    }
}

fn align_child_spaces<'a, Message>(children: &Vec<Node<'a, Message>>, child_spaces: &mut Vec<RealizedSize>, used_space: &UsedSpace , content_space: &RealizedSize) {


    let unused_x = f32::max(0.0, content_space.width - used_space.w) ;
    let unused_y = f32::max(0.0, content_space.height - used_space.h);


    match children[0].attributes().align.y {

        AlignmentY::Bottom => {
            println!("");
            println!("space {:?}", content_space);
            println!("{:?}", used_space);
            println!("(Unused_x, unused_y) ({:?},{})", unused_x, unused_y);

            println!("child_size {:?}", child_spaces[0]);
            println!("");
        },
        _ => {}\
    };


    //println!("unysed {:#?}", child_spaces_info);
    align_child_spaces_x(children, child_spaces, content_space.width, unused_x);
    align_child_spaces_y(children, child_spaces, content_space.height, unused_y);

    match children[0].attributes().align.y {

        AlignmentY::Bottom => {
            println!("ALIGNED BOTTOM SPACE");
            println!("child_size {:?}", child_spaces[0]);
            println!("");
        },
        _ => {}
    };
}


fn align_child_spaces_x<'a, Message>(children: &Vec::<Node<'a, Message>>, child_spaces: &mut Vec::<RealizedSize>, content_width: f32, mut unused_x: f32) {
    let mut center_elements_left = None;
    let mut center_elements_right = 0.0;


    for i in 0..children.len() {
        let c = &children[i];
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
            },

            AlignmentX::Right => { break }, // when we first align to the right, centering does nothing after
            _ => {}
        }
    }

    let mut center_elements_width = match center_elements_left {
        None => None,
        Some(left) => Some(center_elements_right - left)
    };


    let mut x_offset = 0.0;

    for i in 0..children.len() {
        let c = &children[i];
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

}


fn align_child_spaces_y<'a, Message>(children: &Vec::<Node<'a, Message>>, child_spaces: &mut Vec::<RealizedSize>, content_height: f32, mut unused_y: f32) {
    let mut center_elements_top = None;
    let mut center_elements_bottom = 0.0;


    for i in 0..children.len() {
        let c = &children[i];
        let cs = &mut child_spaces[i];


        match c.attributes().align.y {
            AlignmentY::Center => {
                match center_elements_top {
                    None => {
                        center_elements_top = Some(cs.y);
                    },
                    _ => {}// Already set we a previous element
                }

                center_elements_bottom = cs.y + cs.height;
            },

            AlignmentY::Bottom => { break }, // when we first align to the bottom, centering does nothing after
            _ => {}
        }
    }

    let mut center_elements_height = match center_elements_top {
        None => None,
        Some(top) => Some(center_elements_bottom - top)
    };




    let mut y_offset = 0.0;

    for i in 0..children.len() {
        let c = &children[i];
        let cs = &mut child_spaces[i];


        match c.attributes().align.y {
            AlignmentY::Top => {},
            AlignmentY::Center => {
                match center_elements_height {
                    None => {},
                    Some(offset) => {

                        let desired_y = content_height/2.0 - offset/2.0 - center_elements_top.unwrap();
                        let new_offset = f32::max(0.0, desired_y);
                        y_offset += new_offset;
                        unused_y -= new_offset;
                        center_elements_height = None;
                    }
                }
            },
            AlignmentY::Bottom => {

                // take all remaning space to the bottom and offset by that
                y_offset += f32::max(0.0, unused_y);
                unused_y = 0.0;
            },

        }

        //println!("y_offset {:?}", y_offset);

        cs.y += y_offset;
    }

}




#[derive(Debug, Clone, Copy)]
struct UsedSpace {
    pub w: f32,
    pub h: f32,
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
