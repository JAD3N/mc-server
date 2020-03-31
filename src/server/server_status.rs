use serde_json::json;

pub struct ServerStatusVersion {
    name: String,
    protocol: i32,
}

pub struct ServerStatusPlayers {
    max_players: i32,
    num_players: i32,
}

pub struct ServerStatus {
    version: ServerStatusVersion,
    description: String,
    favicon: Option<String>,
}

impl ServerStatus {
    pub fn new() -> ServerStatus {

    }

    pub fn max_players(&self) -> i32 {
        self.max_players
    }

    pub fn set_max_players(&mut self, max_players: i32) {
        self.max_players = max_players;
    }

    pub fn num_players(&self) -> i32 {
        self.num_players
    }

    pub fn set_num_players(&mut self, num_players: i32) {
        self.num_players = num_players;
    }
}