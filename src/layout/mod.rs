pub mod row;

pub mod column;

pub mod attributes;

pub mod element;

#[derive(Debug,Clone, Copy)]
pub struct RealizedSize {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32
}
