pub trait Traitable<T: Copy + PartialEq> {
    fn traits(&self) -> &Vec<T>;
    fn mut_traits(&mut self) -> &mut Vec<T>;

    fn has(&self, t: T) -> bool {
        self.traits().contains(&t)
    }

    fn add(&mut self, t: T) -> &mut Self {
        if !self.has(t) {
            self.mut_traits().push(t);
        }

        self
    }

    fn remove(&mut self, t: T) -> &mut Self {
        if self.has(t) {
            let traits = self.mut_traits();

            for i in 0..traits.len() {
                if traits[i] == t {
                    traits.remove(i);
                }
            }
        }

        self
    }
}

#[macro_export]
macro_rules! traitable {
    ($traits:ty, $name:ident) => {
        traitable!($traits, $name { });

        impl $name {
            pub fn new() -> $name {
                $name { traits: vec![] }
            }
        }
    };
    ($traits:ty, $name:ident { $( $field:ident: $ty:ty ),* $(,)* }) => {
        use crate::util::Traitable;

        #[derive(Clone)]
        pub struct $name {
            traits: Vec<$traits>,
            $( $field: $ty ),*
        }

        impl Traitable<$traits> for $name {
            fn traits(&self) -> &Vec<$traits> {
                &self.traits
            }

            fn mut_traits(&mut self) -> &mut Vec<$traits> {
                &mut self.traits
            }
        }
    };
}