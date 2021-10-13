use super::*;
use crate::components::base::*;
use crate::components::container::*;
use crate::layout::attributes::*;

pub trait Element<T> {

    fn attributes(&self) -> &Attributes;

    fn attributes_mut(&mut self) -> &mut Attributes;

    fn add_to_container(&self, container: &mut ComponentContainer<T>, available_space: &RealizedSize);

    fn add_attribute(&mut self, attribute: Attribute) {
        use Attribute::*;

        let mut cur = self.attributes_mut();
        match attribute {
            Width(la) => {
                cur.size.width = la;
            },
            Height(la) => {
                cur.size.height = la;
            },
            Padding(p) => {
                cur.padding.top = p;
                cur.padding.left = p;
                cur.padding.right = p;
                cur.padding.bottom = p;
            }
            PaddingXY(x,y) => {
                cur.padding.left = x;
                cur.padding.right = x;
                cur.padding.top = y;
                cur.padding.bottom = y;
            },
            PaddingEach(padding) => {
                cur.padding = padding
            },
            Spacing(s) => {
                cur.spacing.x = s;
                cur.spacing.y = s;
            },
            SpacingXY(x, y) => {
                cur.spacing.x = x;
                cur.spacing.y = y;
            }

        };

    }
}

pub struct ComponentElement<T> {
    comp: Component,
    handler: Handler<T>,
    attributes: Attributes
}



impl<T> ComponentElement<T> {
    pub fn new(comp: Component, handler: Handler<T>) -> Self {
        Self {
            comp, handler, attributes: Default::default()
        }
    }
}


impl<T> Element<T> for ComponentElement<T> {


    fn attributes(&self) -> &Attributes {
        &self.attributes
    }


    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }


    fn add_to_container(&self, container: &mut ComponentContainer<T>, available_space: &RealizedSize) {

        // Update out base component to have the correct size

        let mut new_comp = self.comp.clone();


        new_comp.base.coords.x = available_space.x;

        let attributes = self.attributes();
        match attributes.size.width {
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

        match attributes.size.height {
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

        container.add_component(new_comp,  self.handler);
    }

}
