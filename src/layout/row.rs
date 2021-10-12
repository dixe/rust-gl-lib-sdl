use crate::components::base::*;
use crate::components::container::*;
use crate::layout::attributes::{*, Length::*, LengthAttrib::*};
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

    fn add_to_container(&self, container: &mut ComponentContainer<T>, available_space: &RealizedSize) {


        // loop over children width and calc abos space used. That is px(u32) and FitContent
        let mut abs_width = 0.;
        let mut fill_count = 0;
        for c in &self.children {
            match c.size().width {
                No(l) => {
                    match l {
                        Px(px) => { abs_width += px; },
                        FitContent => unimplemented!(),
                        _ => { fill_count += 1; }

                    }
                },
                _ => unimplemented!()
            }
        }


        let mut next_x = available_space.x;
        println!("Row abs {:?}, fill count: {}", abs_width, fill_count);
        for c in &self.children {
            let mut child_space = *available_space;
            child_space.width /= 2.0;
            child_space.x = next_x;
            next_x += child_space.width;
            c.add_to_container(container, &child_space);
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
