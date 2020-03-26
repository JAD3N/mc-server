// settings is generic

// use macros to setup setters and getters

// set_port()
/*
this.difficulty = (Difficulty)this.get("difficulty", dispatchNumberOrString(Difficulty::byId, Difficulty::byName), Difficulty::getKey, Difficulty.EASY);
      this.gamemode = (GameType)this.get("gamemode", dispatchNumberOrString(GameType::byId, GameType::byName), GameType::getName, GameType.SURVIVAL);
      this.levelName = this.get("level-name", "world");
      this.levelSeed = this.get("level-seed", "");
      this.levelType = (LevelType)this.get("level-type", LevelType::getLevelType, LevelType::getName, LevelType.NORMAL);
      this.generatorSettings = this.get("generator-settings", "");
      this.serverPort = this.get("server-port", 25565);
      this.maxBuildHeight = this.get("max-build-height", (var0) -> {
         return Mth.clamp((var0 + 8) / 16 * 16, 64, 256);
      }, 256);
      this.announcePlayerAchievements = this.getLegacyBoolean("announce-player-achievements");
      this.enableQuery = this.get("enable-query", false);
      this.queryPort = this.get("query.port", 25565);
      this.enableRcon = this.get("enable-rcon", false);
      this.rconPort = this.get("rcon.port", 25575);
      this.rconPassword = this.get("rcon.password", "");
      this.resourcePackHash = this.getLegacyString("resource-pack-hash");
      this.resourcePackSha1 = this.get("resource-pack-sha1", "");
      this.hardcore = this.get("hardcore", false);
      this.allowNether = this.get("allow-nether", true);
      this.spawnMonsters = this.get("spawn-monsters", true);
      if (this.get("snooper-enabled", true)) {
      }

      this.snooperEnabled = false;
      this.useNativeTransport = this.get("use-native-transport", true);
      this.enableCommandBlock = this.get("enable-command-block", false);
      this.spawnProtection = this.get("spawn-protection", 16);
      this.opPermissionLevel = this.get("op-permission-level", 4);
      this.functionPermissionLevel = this.get("function-permission-level", 2);
      this.maxTickTime = this.get("max-tick-time", TimeUnit.MINUTES.toMillis(1L));
      this.viewDistance = this.get("view-distance", 10);
      this.maxPlayers = this.get("max-players", 20);
      this.networkCompressionThreshold = this.get("network-compression-threshold", 256);
      this.broadcastRconToOps = this.get("broadcast-rcon-to-ops", true);
      this.broadcastConsoleToOps = this.get("broadcast-console-to-ops", true);
      this.maxWorldSize = this.get("max-world-size", (var0) -> {
         return Mth.clamp(var0, 1, 29999984);
      }, 29999984);
      this.playerIdleTimeout = this.getMutable("player-idle-timeout", 0);
      this.whiteList = this.getMutable("white-list", false);
*/
use std::path::PathBuf;
use crate::world::{Difficulty, GameMode};
use crate::util::Properties;

#[derive(Debug)]
pub struct Settings {
    // properties: Properties,
    difficulty: Difficulty,
    game_mode: GameMode,

    level_name: String,
    level_seed: String,
}

impl Settings {
    pub fn load(path: PathBuf) -> Settings {
        let mut properties = Properties::load(path);

        Settings {
            difficulty: Self::get_difficulty(&mut properties, "difficulty", Difficulty::Easy),
            game_mode: Self::get_game_mode(&mut properties, "gamemode", GameMode::Survival),

            level_name: properties.get_default("level-name", "world"),
            level_seed: properties.get_default("level-seed", ""),

            // properties,
        }
    }

    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }

    pub fn game_mode(&self) -> GameMode {
        self.game_mode
    }

    fn get_difficulty(properties: &mut Properties, key: &str, default: Difficulty) -> Difficulty {
        let value = properties.get(key);

        if value.is_some() {
            let value = value.unwrap();
            let mut difficulty = Difficulty::from_name(&value);

            // check if name failed to match
            if difficulty.is_none() {
                // try convert string to integer
                if let Ok(id) = value.parse::<i32>() {
                    // check if id matched
                    difficulty = Difficulty::from_id(id);
                }
            }

            // only return difficulty if found
            if difficulty.is_some() {
                return difficulty.unwrap();
            }
        }

        // apply default difficulty and override existing
        properties.set(key, default.name());
        default
    }

    fn get_game_mode(properties: &mut Properties, key: &str, default: GameMode) -> GameMode {
        let value = properties.get(key);

        if value.is_some() {
            let value = value.unwrap();
            let mut game_mode = GameMode::from_name(&value);

            // check if name failed to match
            if game_mode.is_none() {
                // try convert string to integer
                if let Ok(id) = value.parse::<i32>() {
                    // check if id matched
                    game_mode = GameMode::from_id(id);
                }
            }

            // only return game mode if found
            if game_mode.is_some() {
                return game_mode.unwrap();
            }
        }

        // apply default game mode and override existing
        properties.set(key, default.name());
        default
    }
}