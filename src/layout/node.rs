use crate::layout::element::*;

pub type Node<Message> = Box<dyn Element<Message>>;
