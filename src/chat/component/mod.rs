/*a component has ->
    style
    siblings
    contents
    can be serialized to and from json
*/

use super::Style;
use std::rc::Rc;
use std::cell::RefCell;

pub trait Component {
    fn style(&self) -> &Rc<RefCell<Style>>;
    fn style_mut(&mut self) -> &mut Rc<RefCell<Style>>;

    fn siblings(&self) -> &Vec<Box<dyn Component>>;
    fn append(&mut self, component: Box<dyn Component>);

    fn contents(&self) -> &str {
        ""
    }
}

// pub struct TextComponent {
//     style: Rc<RefCell<Style>>,
//     siblings: Vec<Box<dyn Component>>,
//     text: String,
// }

// impl Component for TextComponent {
//     fn style(&self) -> &Rc<RefCell<Style>> {
//         &self.style
//     }

//     fn style_mut(&mut self) -> &mut Rc<RefCell<Style>> {
//         &mut self.style
//     }

//     fn siblings(&self) -> &Vec<Box<dyn Component>> {
//         &self.siblings
//     }

//     fn append(&mut self, component: Box<dyn Component>) {
//         // adjust child component style
//         let mut style = component.style().borrow_mut();
//         style.set_parent(Some(self.style.clone()));
//         drop(style);

//         // push to siblings
//         self.siblings.push(component);
//     }

//     fn contents(&self) -> &str {
//         self.get_text()
//     }
// }

#[macro_export]
macro_rules! base_component {
    ($name:ident { $( $field:ident: $ty:ty ),* $(,)* }) => {
        base_component!($name { $( $field: $ty ),* }, self => { "" });
    };
    ($name:ident { $( $field:ident: $ty:ty ),* $(,)* }, $self_:ident => $contents:block) => {
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

base_component!(TextComponent { text: String }, self => {
    self.text()
});

impl TextComponent {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text<T: Into<String>>(&mut self, text: T) {
        self.text = text.into();
    }
}