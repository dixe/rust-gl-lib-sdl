use crate::components::container::*;
use crate::layout::attributes::{*, Length::*, LengthAttrib::*};
use crate::layout::element::*;
use super::*;

pub struct Column<T> {
    children: Vec::<Box<dyn Element<T>>>,
    attributes: Attributes
}




impl<T> Column<T> {

    pub fn new() -> Self {

        Column {
            children: Vec::new(),
            attributes: Default::default(),
        }
    }

    pub fn add(mut self, mut el: Box<dyn Element<T>>) -> Self {
        self.children.push(el);
        self
    }
}


impl<T> Element<T> for Column<T> {

    fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }

    fn add_to_container(&self, container: &mut ComponentContainer<T>, available_space: &RealizedSize) {

        // loop over children width and calc abos space used. That is px(u32) and FitContent
        let mut abs_height = 0.;
        let mut fill_count = 0.0;
        for c in &self.children {
            println!("Child h={:?}", c.attributes().height );
            match c.attributes().height {
                No(l) => {
                    match l {
                        Px(px) => { abs_height += px; },
                        FitContent => unimplemented!(),
                        _ => { fill_count += 1.0; }

                    }
                },
                _ => unimplemented!()
            }
        }


        if self.children.len() == 0 {
            return;
        }

        let attribs = self.attributes();

        let padding = attribs.padding;
        let spacing = attribs.spacing;

        let mut next_y = available_space.y + padding.top;

        let content_height = available_space.width - padding.left - padding.right - spacing.x * (self.children.len() - 1) as f32;
        let content_height = available_space.height - padding.bottom - padding.top;


        let dynamic_height = f32::max(0.0, content_height - abs_height) - fill_count * spacing.y;


        for c in &self.children {
            let mut child_space = *available_space;
            child_space.width = content_height;

            child_space.x = available_space.x + padding.left;
            child_space.y = next_y;
            match c.attributes().height {
                No(l) => {
                    match l {
                        Px(px) => {
                            child_space.height = px as f32;
                        },
                        FitContent => unimplemented!(),
                        Fill => {
                            child_space.height = dynamic_height / fill_count as f32 ;
                        },
                        _ => unimplemented!(),

                    }
                },
                _ => unimplemented!()
            }


            next_y += child_space.height + spacing.y;

            c.add_to_container(container, &child_space);
        }
    }
}
