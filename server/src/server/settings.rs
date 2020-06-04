use crate::util::Properties;
use crate::world::{level::LevelKind, Difficulty, GameMode};
use std::path::PathBuf;
use std::env;

#[derive(Debug)]
pub struct ServerSettings {
    allow_flight: bool,
    allow_nether: bool,
    broadcast_console_to_ops: bool,
    broadcast_rcon_to_ops: bool,
    difficulty: Difficulty,
    enable_command_block: bool,
    enable_query: bool,
    enable_rcon: bool,
    enforce_whitlist: bool,
    force_gamemode: bool,
    function_permission_level: u32,
    game_mode: GameMode,
    generate_structures: bool,
    generator_settings: String,
    hardcore: bool,
    level_name: String,
    level_seed: String,
    level_type: LevelKind,
    max_build_height: i32,
    max_players: u32,
    max_tick_time: i32,
    max_world_size: u32,
    motd: String,
    network_compression_threshold: i32,
    online_mode: bool,
    op_permission_level: u32,
    player_idle_timeout: u32,
    prevent_proxy_connections: bool,
    pvp: bool,
    query_port: u32,
    rcon_password: String,
    rcon_port: u32,
    resource_pack: String,
    resource_pack_sha1: String,
    server_ip: String,
    server_port: u32,
    spawn_animals: bool,
    spawn_monsters: bool,
    spawn_npcs: bool,
    spawn_protection: i32,
    use_native_transport: bool,
    view_distance: u32,
    white_list: bool,
}

impl ServerSettings {
    pub fn load() -> ServerSettings {
        let mut path = env::current_dir().unwrap();
        path.push("server.properties");
        Self::load_file(path)
    }

    pub fn load_file(path: PathBuf) -> ServerSettings {
        let mut properties = Properties::load(path);

        ServerSettings {
            allow_flight: properties.get_bool_default("allow-flight", false),
            allow_nether: properties.get_bool_default("allow-nether", true),
            broadcast_console_to_ops: properties.get_bool_default("broadcast-console-to-ops", true),
            broadcast_rcon_to_ops: properties.get_bool_default("broadcast-rcon-to-ops", true),
            difficulty: Self::get_difficulty(&mut properties, "difficulty", Difficulty::Easy),
            enable_command_block: properties.get_bool_default("enable-command-block", false),
            enable_query: properties.get_bool_default("enable-query", false),
            enable_rcon: properties.get_bool_default("enable-rcon", false),
            enforce_whitlist: properties.get_bool_default("enforce-whitelist", false),
            force_gamemode: properties.get_bool_default("force-gamemode", false),
            function_permission_level: properties.get_u32_default("function-permission-level", 2),
            game_mode: Self::get_game_mode(&mut properties, "gamemode", GameMode::Survival),
            generate_structures: properties.get_bool_default("generate-structures", true),
            generator_settings: properties.get_default("generator-settings", ""),
            hardcore: properties.get_bool_default("hardcore", false),
            level_name: properties.get_default("level-name", "world"),
            level_seed: properties.get_default("level-seed", ""),
            level_type: Self::get_level_kind(&mut properties, "level-type", LevelKind::Default),
            max_build_height: properties.get_i32_default("max-build-height", 256),
            max_players: properties.get_u32_default("max-players", 20),
            max_tick_time: properties.get_i32_default("max-tick-time", 60 * 1000),
            max_world_size: properties.get_u32_default("max-world-size", 29999984),
            motd: properties.get_default("motd", "A Rusty Minecraft Server"),
            network_compression_threshold: properties.get_i32_default("network-compression-threshold", 256),
            online_mode: properties.get_bool_default("online-mode", true),
            op_permission_level: properties.get_u32_default("op-permission-level", 4),
            player_idle_timeout: properties.get_u32_default("player-idle-timeout", 0),
            prevent_proxy_connections: properties.get_bool_default("prevent-proxy-connections", false),
            pvp: properties.get_bool_default("pvp", true),
            query_port: properties.get_u32_default("query.port", 25565),
            rcon_password: properties.get_default("rcon.password", ""),
            rcon_port: properties.get_u32_default("rcon.port", 25565),
            resource_pack: properties.get_default("resource-pack", ""),
            resource_pack_sha1: properties.get_default("resource-pack-sha1", ""),
            server_ip: properties.get_default("server-ip", ""),
            server_port: properties.get_u32_default("server-port", 25565),
            spawn_animals: properties.get_bool_default("spawn-animals", true),
            spawn_monsters: properties.get_bool_default("spawn-monsters", true),
            spawn_npcs: properties.get_bool_default("spawn-npcs", true),
            spawn_protection: properties.get_i32_default("spawn-protection", 16),
            use_native_transport: properties.get_bool_default("use-native-transport", true),
            view_distance: properties.get_u32_default("view-distance", 10),
            white_list: properties.get_bool_default("white-list", true),
        }
    }

    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }

    pub fn game_mode(&self) -> GameMode {
        self.game_mode
    }

    pub fn max_tick_time(&self) -> i32 {
        self.max_tick_time
    }

    pub fn addr(&self) -> String {
        // clean up ip
        let mut ip = self.server_ip
            .trim()
            .to_string();

        // check ip is valid
        if ip.len() == 0 {
            ip.push_str("0.0.0.0");
        }

        format!("{}:{}", ip, self.server_port)
    }

    pub fn motd(&self) -> &str {
        &self.motd
    }

    pub fn max_players(&self) -> u32 {
        self.max_players
    }
}

macro_rules! add_custom_fn {
    ($fn: ident, $type: ty) => {
        impl ServerSettings {
            fn $fn(properties: &mut Properties, key: &str, default: $type) -> $type {
                let value = properties.get(key);

                if value.is_some() {
                    let value = value.unwrap();
                    let mut custom_value = <$type>::from_name(&value);

                    // check if name failed to match
                    if custom_value.is_none() {
                        // try convert string to integer
                        if let Ok(id) = value.parse::<i32>() {
                            // check if id matched
                            custom_value = <$type>::from_id(id);
                        }
                    }

                    // only return if found
                    if custom_value.is_some() {
                        return custom_value.unwrap();
                    }
                }

                // apply default and override existing
                properties.set(key, default.name());
                default
            }
        }
    };
}

add_custom_fn!(get_difficulty, Difficulty);
add_custom_fn!(get_game_mode, GameMode);
add_custom_fn!(get_level_kind, LevelKind);
