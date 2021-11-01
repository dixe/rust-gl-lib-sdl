use gl_lib::{gl};
use crate::layout::node::Node;


pub trait State<Message> {

    fn handle_message(&mut self, message: &Message);

    fn view(&self, gl: &gl::Gl) -> Node<Message>;


}
