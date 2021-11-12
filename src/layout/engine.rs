use crate::layout::attributes::{self, LengthConstraint, Alignment, Padding, Spacing};
use super::*;
use crate::layout::node::*;
use gl_lib::text_rendering::{ text_renderer::TextRenderer };
use crate::layout::element::Element;
use std::fmt;


#[derive(Debug)]
pub struct RenderedElement<'a, Message> where Message: fmt::Debug {
    node: Node<'a, Message>,
    realized_size: RealizedSize
}

pub fn render_tree<'a, Message>(root: Node<'a, Message>, output_size: Size, text_renderer: &TextRenderer) -> Vec::<RenderedElement<'a, Message>> where Message: 'a + fmt::Debug {

    let sized_tree = get_sized_tree(root, output_size, text_renderer);

    let mut elements = Vec::new();
    map_size_to_rendered(sized_tree, &mut elements);

    println!("All Rendered elements: {:?}", elements.len());
    elements

}

fn map_size_to_rendered<'a, Message>(mut sized_node: NodeWithSize<'a, Message>, elements: &mut Vec::<RenderedElement<'a, Message>>) where Message: fmt::Debug {

    while let Some(child) = sized_node.children.pop() {
        map_size_to_rendered(child, elements);
    }

    elements.push(RenderedElement {
        node: sized_node.node,
        realized_size: sized_node.layout.into()
    });

}


fn get_sized_tree<'a, Message>(mut node: Node<'a, Message>, available_space: Size, text_renderer: &TextRenderer) -> NodeWithSize<'a, Message> where Message: 'a + fmt::Debug {


    let attribs = *node.attributes();
    let padding = attribs.padding;
    let spacing = attribs.spacing;

    let content_size = Size {
        w: final_width(&node, &attribs, available_space, text_renderer, OnFill::Expand) - padding.right - padding.left,
        h: final_height(&node, &attribs, available_space, text_renderer, OnFill::Expand) - padding.bottom - padding.top,
    };

    let engine_w = match attribs.width {
        attributes::Length::Px(px) => EngineLength::Px(px),
        attributes::Length::Fill => EngineLength::Fill,
        attributes::Length::FillPortion(p) => EngineLength::FillPortion(p),
        attributes::Length::FitContent => EngineLength::Px(node.content_width(&content_size.into(), text_renderer) as u32),
    };

    let engine_h = match attribs.height {
        attributes::Length::Px(px) => EngineLength::Px(px),
        attributes::Length::Fill => EngineLength::Fill,
        attributes::Length::FillPortion(p) => EngineLength::FillPortion(p),
        attributes::Length::FitContent => EngineLength::Px(node.content_height(&content_size.into(), text_renderer) as u32),
    };

    let mut children = Vec::new();




    while pop_children( &mut node, &mut children, content_size, text_renderer) {
    }

    let layout = LayoutElement::new(engine_w, engine_h, attribs, content_size);

    NodeWithSize { node, layout, children }


}


fn pop_children<'a, Message>(node: &'a mut Node<'a, Message>, children: &mut Vec::<NodeWithSize<'a, Message>>, content_size: Size, text_renderer: &TextRenderer) -> bool  where Message: 'a + fmt::Debug {
    let child = node.pop_children_front();

    match child {
        None => false,
        Some(c) => {
            children.push(get_sized_tree(c, content_size, text_renderer));
            true
        }
    }
}

#[derive(Debug)]
struct NodeWithSize<'a, Message> where Message: fmt::Debug {
    node: Node<'a, Message>,
    layout: LayoutElement,
    children: Vec<NodeWithSize<'a, Message>>
}


impl From<LayoutElement> for RealizedSize {
    fn from(layout: LayoutElement) -> Self {
        Self {
            x: layout.position.x,
            y: layout.position.y,
            width: layout.content_size.w,
            height: layout.content_size.h
        }

    }
}


#[derive(Debug, Clone, Copy)]
pub enum OnFill {
    Expand,
    Shrink
}

fn final_height<'a, Message>(node: &Node<'a, Message>, attributes: &attributes::Attributes, available_space: Size, text_renderer: &TextRenderer, on_fill: OnFill) -> f32 where Message: fmt::Debug {

    let h = match attributes.height {
        attributes::Length::Px(px) => {
            px as f32
        },
        attributes::Length::FitContent => node.content_height(&(available_space.into()), text_renderer),
        _ => match on_fill {
            OnFill::Expand => available_space.h,
            OnFill::Shrink => node.content_height(&(available_space.into()), text_renderer),
        }
    };

    let min = attributes.height_constraint.min();
    let max = attributes.height_constraint.max(available_space.h);

    f32::min(max, f32::max(min, h))

}

fn final_width<'a, Message>(node: &Node<'a, Message>, attributes: &attributes::Attributes, available_space: Size, text_renderer: &TextRenderer, on_fill: OnFill) -> f32 where Message: fmt::Debug {

    let w = match attributes.width {
        attributes::Length::Px(px) => {
            px as f32
        },
        attributes::Length::FitContent => node.content_width(&(available_space.into()), text_renderer),
        _ => match on_fill {
            OnFill::Expand => available_space.w,
            OnFill::Shrink => node.content_width(&(available_space.into()), text_renderer),
        }
    };

    let min = attributes.width_constraint.min();
    let max = attributes.width_constraint.max(available_space.w);

    f32::min(max, f32::max(min, w))

}





/// Almost the same as a regular length, except that Fit Content has been calculated to a px value
#[derive(Debug, Clone, Copy)]
enum EngineLength {
    /// Length equal to given number of pixels
    Px(u32),

    /// Fill all the avialable space
    /// Is equivalent to FillPortion 1
    Fill,

    /// Fill a portion
    /// If 1 child has FillPortion 1 and another has FillPortion 3
    /// Than the first child will have 1/4 of the space and the other child the
    /// remainin 3/4
    FillPortion(u32),
}


#[derive(Debug, Clone, Copy)]
struct EngineAttributes {
    width: EngineLength,
    height: EngineLength,
    width_constraint: LengthConstraint,
    height_constraint: LengthConstraint,
    align: Alignment,
    padding: Padding,
    spacing: Spacing
}




#[derive(Debug, Clone, Copy)]
struct LayoutElement {
    attributes: EngineAttributes,
    content_size: Size,
    position: Point,
}

impl LayoutElement {

    fn new(width:EngineLength, height: EngineLength, attributes: attributes::Attributes, content_size: Size) -> Self {
        Self {
            attributes: EngineAttributes {
                width,
                height,
                width_constraint: attributes.width_constraint,
                height_constraint: attributes.height_constraint,
                align: attributes.align,
                padding: attributes.padding,
                spacing: attributes.spacing,
            },
            content_size,
            position: Point::default(),
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32
}


#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub w: f32,
    pub h: f32
}
