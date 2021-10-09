use crate::components::base::*;
use crate::components::container::*;
use crate::components::attributes::*;
use gl_lib::text_rendering::{text_renderer::TextRenderer};
use gl_lib::{gl, objects::square, shader, ScreenBox};
use crate::layout::element::*;
use super::*;

pub struct Row<T> {
    children: Vec::<Box<dyn Element<T>>>,
    size: Size
}



impl<T> Row<T> {

    pub fn new(size: Size) -> Self {

        Row {
            children: Vec::new(),
            size
        }
    }

    pub fn add(&mut self, el: Box<dyn Element<T>>) {
        self.children.push(el);
    }




}


impl<T> Element<T> for Row<T> {

    fn size(&self) -> Size {
        self.size
    }

    fn add_to_container(mut self, container: &mut ComponentContainer<T>, available_space: &RealizedSize) {


        while let Some(c) = self.children.pop() {
            c.add_to_container(container, &available_space);
        }
        /*

        // dependent on the
        // TODO: Make this live on Element and not row
        let final_width = match self.size().width {
        LengthAttrib::No(l) =>
        match l {
        Length::Px(px) => px,
        Length::Fill => {},
        _ => unimplemented!(),

    },
        LengthAttrib::Max(l) => {

        let r : u32 = match l {
        Length::Px(px) => px.min(available_space.width),
        _ => unimplemented!(),
    };
        r
    },
        _ => unimplemented!(),


    };

         */

    }

}

/*
fn get_length() -> Length {

let final_width = match self.size().width {
LengthAttrib::No(l) =>
match l {
Length::Px(px) => px,
Length::Fill => {},
_ => unimplemented!(),

            },
        LengthAttrib::Max(l) => {

            let r : u32 = match l {
                Length::Px(px) => px.min(available_space.width),
                _ => unimplemented!(),
            };
            r
        },
        _ => unimplemented!(),


    }
}
*/
