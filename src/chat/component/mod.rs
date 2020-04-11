mod text;
pub use text::*;

use super::Style;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Component {
    fn style(&self) -> &Rc<RefCell<Style>>;
    fn style_mut(&mut self) -> &mut Rc<RefCell<Style>>;

    fn siblings(&self) -> &Vec<Box<dyn Component>>;
    fn siblings_mut(&mut self) -> &mut Vec<Box<dyn Component>>;

    fn append(&mut self, component: Box<dyn Component>) {
        // adjust child component style
        let mut style = component.style().borrow_mut();
        style.set_parent(Some(self.style().clone()));
        drop(style);

        // push to siblings
        self.siblings_mut().push(component);
    }

    fn contents(&self) -> &str {
        ""
    }
}