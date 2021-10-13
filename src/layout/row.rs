use crate::components::base::*;
use crate::components::container::*;
use crate::layout::attributes::{*, Length::*, LengthAttrib::*};
use gl_lib::text_rendering::{text_renderer::TextRenderer};
use gl_lib::{gl, objects::square, shader, ScreenBox};
use crate::layout::element::*;
use super::*;

pub struct Row<T> {
    children: Vec::<Box<dyn Element<T>>>,
    attributes: Attributes
}



impl<T> Row<T> {

    pub fn new() -> Self {

        Row {
            children: Vec::new(),
            attributes: Default::default(),
        }
    }

    pub fn add(&mut self, el: Box<dyn Element<T>>) {
        self.children.push(el);
    }
}


impl<T> Element<T> for Row<T> {

    fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }

    fn add_to_container(&self, container: &mut ComponentContainer<T>, available_space: &RealizedSize) {


        // loop over children width and calc abos space used. That is px(u32) and FitContent
        let mut abs_width = 0.;
        let mut fill_count = 0;
        for c in &self.children {
            match c.attributes().size.width {
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


        if self.children.len() == 0 {
            return;
        }

        let attributes = self.attributes();
        let mut next_x = available_space.x + attributes.padding.left;

        let content_width = available_space.width - attributes.padding.left - attributes.padding.right - attributes.spacing.x * (self.children.len() - 1) as f32;
        let content_height = available_space.height - attributes.padding.bottom - attributes.padding.top;


        for c in &self.children {
            let mut child_space = *available_space;
            child_space.width = content_width/ 2.0;
            child_space.x = next_x;
            next_x += child_space.width + attributes.spacing.x;
            c.add_to_container(container, &child_space);
        }
    }
}
