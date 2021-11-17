use gl_lib_sdl::{
    gl_lib::na,
};
mod component;
pub use self::component::*;

mod layout_element;
pub use self::layout_element::*;



pub type Point = na::Vector2::<usize>;



#[derive(Copy, Clone, Debug)]
pub enum Tile {
    Hidden,
    UnCovered,
    Numbered(u8),
    Bomb,
}
