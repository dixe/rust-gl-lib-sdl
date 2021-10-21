use gl_lib::{gl};
use crate::layout::element::Element;


pub trait State<Message> {

    fn handle_message(&mut self, message: &Message);

    fn view(&self, gl: &gl::Gl) -> Box<Element<Message>>;


}
