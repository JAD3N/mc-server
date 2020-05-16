#[macro_export]
macro_rules! get_block {
    ($k:expr) => {
        {
            let blocks = $crate::BLOCKS.read().unwrap();
            let blocks_ref = blocks.as_ref().unwrap();

            match blocks_ref.get_value($k) {
                Some(block) => Some(block.clone()),
                None => None,
            }
        }
    };
}

#[macro_export]
macro_rules! get_server {
    () => {
        unsafe {
            $crate::server::SERVER.as_ref()
                .unwrap()
                .clone()
        }
    };
}