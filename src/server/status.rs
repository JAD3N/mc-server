use std::io::{self, Read, Write};
use std::fmt;
use crate::network::protocol::Protocol;
use crate::chat::component::BoxedComponent;
use crate::util::ToJsonValue;
use crate::auth::Profile;

pub struct ServerStatusVersion {
    pub name: String,
    pub protocol: i32,
}

impl ToJsonValue for ServerStatusVersion {
    fn to_json(&self) -> Option<serde_json::Value> {
        Some(json!({
            "name": &self.name,
            "protocol": self.protocol,
        }))
    }
}

pub struct ServerStatusPlayers {
    pub max_players: i32,
    pub num_players: i32,
    pub sample: Vec<Profile>,
}

impl ToJsonValue for ServerStatusPlayers {
    fn to_json(&self) -> Option<serde_json::Value> {
        let mut json = json!({
            "max": self.max_players,
            "online": self.num_players,
        });

        if self.sample.len() > 0 {
            let sample: Vec<serde_json::Value> = self.sample.iter().map(|profile| {
                let uuid = match profile.uuid() {
                    Some(uuid) => uuid.to_hyphenated().to_string(),
                    None => String::new(),
                };

                json!({
                    "id": uuid,
                    "name": profile.name(),
                })
            }).collect();

            json["sample"] = json!(sample);
        }

        Some(json)
    }
}

pub struct ServerStatus {
    pub description: Option<BoxedComponent>,
    pub players: Option<ServerStatusPlayers>,
    pub version: Option<ServerStatusVersion>,
    pub favicon: Option<String>,
}

impl ServerStatus {
    pub fn new() -> Self {
        Self {
            description: None,
            players: None,
            version: None,
            favicon: None,
        }
    }
}

impl ToJsonValue for ServerStatus {
    fn to_json(&self) -> Option<serde_json::Value> {
        let mut json = json!({});

        if let Some(description) = &self.description {
            json["description"] = description.to_json().unwrap();
        }

        if let Some(players) = &self.players {
            json["players"] = players.to_json().unwrap();
        }

        if let Some(version) = &self.version {
            json["version"] = version.to_json().unwrap();
        }

        if let Some(favicon) = &self.favicon {
            json["favicon"] = json!(favicon);
        }

        Some(json)
    }
}

impl fmt::Display for ServerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.to_json()
            .unwrap_or(serde_json::Value::Null)
            .to_string();

        write!(f, "{}", s)
    }
}

// impl Protocol for ServerStatus {
//     fn len(&self) -> usize {
//         Protocol::len(&self.to_string())
//     }

//     fn write<U: Write>(&self, dst: &mut U) -> io::Result<()> {
//         self.to_string().write(dst)
//     }
// }