pub use crate::components::base::*;

pub struct Button {
    base: ComponentBase,
    content: String // Use another compontent for content
}


impl Button {

    pub fn new() -> Self {
        Self {
            base: ComponentBase::new(),
            content: "Test btn".to_string()
        }
    }

}

impl Component for Button {

    fn component_base(&self) -> &ComponentBase {
        &self.base
    }
    fn content(&self) -> &str {
        &self.content
    }
}
