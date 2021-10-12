use super::*;
use crate::components::base::*;
use crate::components::container::*;
use crate::layout::attributes::*;

pub trait Element<T> {

    fn size(&self) -> Size;

    fn add_to_container(&self, container: &mut ComponentContainer<T>, available_space: &RealizedSize);
}

pub struct ComponentElement<T> {
    comp: Component,
    handler: Handler<T>,
    size: Size
}





impl<T> ComponentElement<T> {
    pub fn new(comp: Component, handler: Handler<T>, size: Size) -> Self {
        Self {
            comp, handler, size
        }
    }
}


impl<T> Element<T> for ComponentElement<T> {

    fn size(&self) -> Size {
        self.size
    }


    fn add_to_container(&self, container: &mut ComponentContainer<T>, available_space: &RealizedSize) {

        // Update out base component to have the correct size

        let mut new_comp = self.comp.clone();


        new_comp.base.coords.x = available_space.x;

        match self.size().width {
            LengthAttrib::No(l) =>
                match l {
                    Length::Px(px) => {
                        new_comp.base.set_width(px, available_space.width)
                    },
                    Length::Fill => {new_comp.base.set_width(available_space.width, available_space.width)},
                    _ => unimplemented!(),

                },
            LengthAttrib::Max(l) => {
                match l {
                    Length::Px(px) =>{panic!() },
                    _ => unimplemented!(),
                }
            },
            _ => unimplemented!(),
        };

        match self.size().height {
            LengthAttrib::No(l) =>
                match l {
                    Length::Px(px) => {
                        new_comp.base.set_height(px, available_space.height)
                    },
                    Length::Fill => {new_comp.base.set_height(available_space.height, available_space.height)},
                    _ => unimplemented!(),

                },
            LengthAttrib::Max(l) => {
                match l {
                    Length::Px(px) =>{panic!()},
                    _ => unimplemented!(),
                }
            },
            _ => unimplemented!(),
        };

        println!("Adding comp {:#?}\n\n", new_comp);

        container.add_component(new_comp,  self.handler);
    }

}
