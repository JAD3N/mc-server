use super::Style;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Component {
    fn style(&self) -> &Rc<RefCell<Style>>;
    fn style_mut(&mut self) -> &mut Rc<RefCell<Style>>;

    fn siblings(&self) -> &Vec<Box<dyn Component>>;
    fn append(&mut self, component: Box<dyn Component>);

    fn contents(&self) -> &str {
        ""
    }
}

#[macro_export]
macro_rules! base_component {
    ($name:ident { $( $field:ident: $ty:ty ),* $(,)* }) => {
        base_component!($name { $( $field: $ty ),* }, self => { "" });
    };
    ($name:ident { $( $field:ident: $ty:ty ),* $(,)* }, $self_:ident => $contents:block) => {
        use crate::chat::{Style, component::Component};
        use std::cell::RefCell;
        use std::rc::Rc;

        pub struct $name {
            style: Rc<RefCell<Style>>,
            siblings: Vec<Box<dyn Component>>,
            $( $field: $ty ),*
        }

        impl Component for $name {
            fn style(&self) -> &Rc<RefCell<Style>> {
                &self.style
            }

            fn style_mut(&mut self) -> &mut Rc<RefCell<Style>> {
                &mut self.style
            }

            fn siblings(&self) -> &Vec<Box<dyn Component>> {
                &self.siblings
            }

            fn append(&mut self, component: Box<dyn Component>) {
                // adjust child component style
                let mut style = component.style().borrow_mut();
                style.set_parent(Some(self.style.clone()));
                drop(style);

                // push to siblings
                self.siblings.push(component);
            }

            fn contents(&$self_) -> &str $contents
        }
    };
}

mod text;
pub use text::*;