use super::*;
use crate::components::container::*;
use crate::layout::attributes::{self, Length, LengthConstraint, Attributes, Attribute, AlignmentY, AlignmentX};
use gl_lib::text_rendering::{ text_renderer::TextRenderer };


pub trait Element<Message> {

    fn attributes(&self) -> &Attributes;


    fn attributes_mut(&mut self) -> &mut Attributes;


    fn add_to_container(&self, container: &mut ComponentContainer<Message>, available_space: &RealizedSize, text_renderer: &TextRenderer);

    fn content_height(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32;

    fn final_height(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32 {

        let h = match self.attributes().height {
            Length::Px(px) => {
                px as f32
            },
            Length::FitContent => {
                let ch = self.content_height(available_space, text_renderer);
                ch
            },
            _ => available_space.height,
        };



        let attribs = self.attributes();

        let min = attribs.height_constraint.min();
        let max = attribs.height_constraint.max(available_space.height);


        f32::min(max, f32::max(min, h))

    }


    fn content_width(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32;

    fn final_width(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32 {

        let w = match self.attributes().width {
            Length::Px(px) => {
                px as f32
            },
            Length::FitContent => {
                self.content_width(available_space, text_renderer)
            },
            _ => available_space.width,
        };


        let attribs = self.attributes();

        let min = attribs.width_constraint.min();
        let max = attribs.width_constraint.max(available_space.width);

        f32::min(max, f32::max(min, w))

    }

    fn width(self, w: Length) -> Self where Self: Sized {
        self.add_attribute(Attribute::Width(w))
    }

    fn height(self, h: Length) -> Self where Self: Sized {
        self.add_attribute(Attribute::Height(h))
    }


    fn max_width(self, w: LengthConstraint) -> Self where Self: Sized {
        self.add_attribute(Attribute::WidthConstraint(w))
    }

    fn max_height(self, h: LengthConstraint) -> Self where Self: Sized {
        self.add_attribute(Attribute::HeightConstraint(h))
    }

    fn padding(self, p: f32) -> Self where Self: Sized {
        self.add_attribute(Attribute::Padding(p))
    }

    fn spacing(self, s: f32) -> Self where Self: Sized {
        self.add_attribute(Attribute::Spacing(s))
    }


    fn align_center(self) -> Self where Self: Sized {
        self.add_attribute(Attribute::AlignmentX(AlignmentX::Center))
            .add_attribute(Attribute::AlignmentY(AlignmentY::Center))
    }

    fn align_center_x(self) -> Self where Self: Sized {
        self.add_attribute(Attribute::AlignmentX(AlignmentX::Center))
    }

    fn align_center_y(self) -> Self where Self: Sized {
        self.add_attribute(Attribute::AlignmentY(AlignmentY::Center))
    }

    fn align_top(self) -> Self where Self: Sized {
        self.add_attribute(Attribute::AlignmentY(AlignmentY::Top))
    }

    fn align_bottom(self) -> Self where Self: Sized {
        self.add_attribute(Attribute::AlignmentY(AlignmentY::Bottom))
    }

    fn align_left(self) -> Self where Self: Sized {
        self.add_attribute(Attribute::AlignmentX(AlignmentX::Left))
    }

    fn align_right(self) -> Self where Self: Sized {
        self.add_attribute(Attribute::AlignmentX(AlignmentX::Right))
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

            WidthConstraint(constraint) => {
                cur.width_constraint = constraint;
            },

            HeightConstraint(constraint) => {
                cur.height_constraint = constraint;
            },

            Spacing(s) => {
                let spacing = attributes::Spacing {
                    x: s,
                    y: s
                };
                cur.spacing = spacing;
            },

            SpacingXY(x, y) => {
                let spacing = attributes::Spacing {
                    x,
                    y
                };
                cur.spacing = spacing;
            },

            Alignment(align) => {
                cur.align = align;
            },
            AlignmentX(x) => {
                cur.align.x = x;
            },
            AlignmentY(y) => {
                cur.align.y = y;
            },

        };
        self
    }
}
