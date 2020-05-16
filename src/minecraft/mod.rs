mod blocks;
mod items;
mod sounds;

pub fn init() {
    blocks::init();
    items::init();
    sounds::init();

    info!("init -> Minecraft module initialized.");
}