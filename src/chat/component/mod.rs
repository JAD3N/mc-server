mod text;

pub use text::*;

use super::Style;
use std::sync::{Arc, Mutex, MutexGuard};

pub enum Component {
    Text(TextComponent),
    Base(TextComponent),
}

pub trait BaseComponent: Into<Component> {
    // fn style(&self) -> Option<&Style>;
}

type ComponentRef = Arc<Mutex<Component>>;

pub struct ComponentContainer {
    component: ComponentRef,
    children: Vec<Self>,
    parent: Option<ComponentRef>,
}

impl ComponentContainer {
    pub fn new<C: Into<Component>>(component: C) -> ComponentContainer {
        ComponentContainer {
            component: Arc::new(Mutex::new(component.into())),
            children: vec![],
            parent: None,
        }
    }

    pub fn component(&self) -> MutexGuard<'_, Component> {
        self.component.lock().unwrap()
    }

    pub fn append(&mut self, mut container: ComponentContainer) {
        container.parent = Some(self.component.clone());
        self.children.push(container);
    }
}

pub fn test() {
    let c1 = TextComponent { text: String::from("") };
    let c2 = TextComponent { text: String::from("This is some sample text.") };

    let mut cc1 = ComponentContainer::new(c1);
    let cc2 = ComponentContainer::new(c2);

    cc1.append(cc2);

    let component = cc1.children.get(0).unwrap().component();

    if let Component::Text(ref component) = *component {
        println!("Test! {}", component.text);
    }
}