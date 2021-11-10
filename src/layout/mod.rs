use gl_lib::{gl::viewport};

pub mod row;

pub mod column;

pub mod attributes;

pub mod button;

pub mod element;

pub mod node;

pub mod container;

#[derive(Debug,Clone, Copy)]
pub struct RealizedSize {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32
}


impl From<&viewport::Viewport> for RealizedSize {

    fn from(viewport: &viewport::Viewport) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: viewport.w as f32,
            height: viewport.h as f32
        }

    }

}
