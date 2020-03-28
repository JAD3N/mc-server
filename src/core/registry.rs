// use crate::world::level::DimensionType;
// use lazy_static;

// enum RegistryItem {
//     DimensionType(DimensionType),
// }

// impl Into<RegistryItem> for DimensionType {
//     fn into(self) -> RegistryItem {
//         RegistryItem::DimensionType(self)
//     }
// }

// fn register_item<T: Into<RegistryItem>>(key: &str, value: T) {

// }

// #[macro_export]
// macro_rules! register {
//     ($key:expr, $value:expr) => {
//         lazy_static
//         register_item($key, $value);
//     };
// }

// fn test() {
// register_item("test", DimensionType::OVERWORLD);
// }