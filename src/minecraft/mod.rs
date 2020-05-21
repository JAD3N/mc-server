mod blocks;
mod items;
mod sounds;
mod protocol;

pub fn init() {
    blocks::init();
    items::init();
    sounds::init();
    protocol::init();

    info!("init -> Minecraft module initialized.");
}