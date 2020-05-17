// use crate::world::level::Level;
// use super::Server;
// use tokio::task;
// use futures::future;
// use std::sync::{Arc, RwLock, RwLockWriteGuard};

// pub async fn tick() {

// }

// pub async fn tick_levels(levels: Vec<Arc<RwLock<Level>>>) {
//     let handles = vec![];

//     for level in levels {
//         let level = level.write().unwrap();
//         let handle = tick_level();

//         handles.push(task::spawn(handle));
//     }

//     future::join_all(handles).await;
// }

// pub async fn tick_level() {
//     // do base level ticking?
// }