use super::*;
use crate::components::base::*;
use crate::components::container::*;
use crate::layout::attributes::{self, Length, Attributes, Attribute, LengthAttrib};

pub trait Element<T> {

    fn attributes(&self) -> &Attributes;

    fn attributes_mut(&mut self) -> &mut Attributes;

    fn add_to_container(&self, container: &mut ComponentContainer<T>, available_space: &RealizedSize);

    fn width(mut self, w: Length) -> Self where Self: Sized {
        let mut cur = self.attributes_mut();
        cur.width = LengthAttrib::No(w);
        self
    }

    fn height(mut self, h: Length) -> Self where Self: Sized {
        self.add_attribute(Attribute::Height(LengthAttrib::No(h)))
    }

    fn padding(mut self, p: f32) -> Self where Self: Sized {
        self.add_attribute(Attribute::Padding(p))
    }

    fn add_attribute(mut self, attribute: Attribute) -> Self where Self: Sized {
        use Attribute::*;

        let mut cur = self.attributes_mut();
        match attribute {
            Width(la) => {
                cur.width = la
            },
            Height(la) => {
                cur.height = la
            },
            Padding(p) => {

                let padding = attributes::Padding {
                    left: p,
                    right: p,
                    top: p,
                    bottom: p
                };
                cur.padding = padding
            }
            PaddingXY(x,y) => {
                let padding = attributes::Padding {
                    left: x,
                    right: x,
                    top: y,
                    bottom: y,
                };
                cur.padding = padding
            },
            PaddingEach(padding) => {
                cur.padding = padding
            },

            Spacing(s) => {
                let spacing = attributes::Spacing {
                    x: s,
                    y: s
                };
                cur.spacing = spacing
            },

            SpacingXY(x, y) => {
                let spacing = attributes::Spacing {
                    x,
                    y
                };
                cur.spacing = spacing
            },
        };
        self
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
        new_comp.base.coords.y = available_space.y;

        let attributes = self.attributes();
        match attributes.width {
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
                    Length::Px(_px) =>{ unimplemented!() },
                    _ => unimplemented!(),
                }
            },
            _ => unimplemented!(),
        };

        match attributes.height {
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
                    Length::Px(_px) =>{ unimplemented!() },
                    _ => unimplemented!(),
                }
            },
            _ => unimplemented!(),
        };

        container.add_component(new_comp,  self.handler);
    }
}
