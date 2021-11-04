use super::*;
use crate::components::container::*;
use crate::layout::attributes::{self, Length, Attributes, Attribute};
use gl_lib::text_rendering::{ text_renderer::TextRenderer };


pub trait Element<Message> {

    fn attributes(&self) -> &Attributes;


    fn attributes_mut(&mut self) -> &mut Attributes;


    fn add_to_container(&self, container: &mut ComponentContainer<Message>, available_space: &RealizedSize, text_renderer: &TextRenderer);


    fn final_height(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32;


    fn final_width(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32;


    fn width(self, w: Length) -> Self where Self: Sized {
        self.add_attribute(Attribute::Width(w))
    }

    fn height(self, h: Length) -> Self where Self: Sized {
        self.add_attribute(Attribute::Height(h))
    }

    fn padding(self, p: f32) -> Self where Self: Sized {
        self.add_attribute(Attribute::Padding(p))
    }

    fn spacing(self, s: f32) -> Self where Self: Sized {
        self.add_attribute(Attribute::Spacing(s))
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

            WidthContraint(constraint) => {
                cur.width_contraint = constraint;
            },

            HeightContraint(constraint) => {
                cur.height_contraint = constraint;
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
