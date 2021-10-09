use super::*;
use crate::components::base::*;
use crate::components::container::*;
use crate::components::attributes::*;

pub trait Element<T> {


    fn size(&self) -> Size;

    fn add_to_container(self, container: &mut ComponentContainer<T>, available_space: &RealizedSize);


}




pub struct ComponentElement<T> {
    comp: Box<dyn Component>,
    handler: Handler<T>,
    size: Size
}





impl<T> ComponentElement<T> {
    pub fn new(comp: Box<dyn Component>, handler: Handler<T>, size: Size) -> Self {
        Self {
            comp, handler, size
        }
    }
}


impl<T> Element<T> for ComponentElement<T> {

    fn size(&self) -> Size {
        self.size
    }


    fn add_to_container(self, container: &mut ComponentContainer<T>, available_space: &RealizedSize) {

        // Update out base component to have the correct size
        container.add_component(self.comp,  self.handler);
    }

}
