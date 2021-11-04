use gl_lib::{gl};
use crate::layout::node::Node;
use crate::window;

pub trait State<Message> {

    fn handle_message(&mut self, message: &Message, windows_acces: &window::WindowComponentAccess);

    fn view(&self, gl: &gl::Gl) -> Node<Message>;


}
