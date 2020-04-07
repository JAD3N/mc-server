#[macro_export]
macro_rules! component {
    ($name:ident { $( $field:ident: $ty:ty ),* $(,)* }) => {
        use crate::chat::Style;

        #[derive(Clone)]
        pub struct $name {
            style: Style,
            $( $field: $ty ),*
        }

        impl $name {
            pub fn style(&self) -> &Style {
                &self.style
            }

            pub fn style_mut(&mut self) -> &mut Style {
                &mut self.style
            }
        }
    };
}

mod text;
pub use text::*;

use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone)]
pub enum ComponentType {
    Text(TextComponent),
}

type ComponentRef = Arc<Mutex<ComponentType>>;

pub struct Component {
    component: ComponentRef,
    children: Vec<Self>,
    parent: Option<ComponentRef>,
}

impl Clone for Component {
    fn clone(&self) -> Self {
        let component = self.component().clone();
        let component = Arc::new(Mutex::new(component));

        let children = self.children.clone();

        let mut component = Component {
            component,
            children,
            parent: None,
        };

        for child_component in &mut component.children {
            child_component.parent = Some(component.component.clone());
        }

        component
    }
}

impl Component {
    pub fn new<C: Into<ComponentType>>(component: C) -> Component {
        Component {
            component: Arc::new(Mutex::new(component.into())),
            children: vec![],
            parent: None,
        }
    }

    pub fn component(&self) -> MutexGuard<'_, ComponentType> {
        self.component.lock().unwrap()
    }

    pub fn append(&mut self, mut container: Component) {
        container.parent = Some(self.component.clone());
        self.children.push(container);
    }
}