use std::fmt;
use crate::network::protocol::{ProtocolLength, ProtocolRead, ProtocolWrite, ProtocolError};
use crate::chat::component::ComponentContainer;
use crate::util::ToJsonValue;
use crate::auth::Profile;

#[derive(Clone)]
pub struct ServerStatusVersion {
    pub name: String,
    pub protocol: u32,
}

impl ToJsonValue for ServerStatusVersion {
    fn to_json(&self) -> Option<serde_json::Value> {
        Some(json!({
            "name": &self.name,
            "protocol": self.protocol,
        }))
    }
}

#[derive(Clone)]
pub struct ServerStatusPlayers {
    pub max_players: u32,
    pub num_players: u32,
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

#[derive(Clone)]
pub struct ServerStatus {
    pub description: ComponentContainer,
    pub players: ServerStatusPlayers,
    pub version: ServerStatusVersion,
    pub favicon: Option<String>,
}

impl ToJsonValue for ServerStatus {
    fn to_json(&self) -> Option<serde_json::Value> {
        let mut json = json!({});

        json["description"] = self.description.to_json().unwrap();
        json["players"] = self.players.to_json().unwrap();
        json["version"] = self.version.to_json().unwrap();

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

impl fmt::Debug for ServerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.to_json()
            .unwrap_or(serde_json::Value::Null)
            .to_string();

        write!(f, "{}", s)
    }
}

impl ProtocolLength for ServerStatus {
    fn len(&self) -> usize {
        ProtocolLength::len(&self.to_string())
    }
}

impl ProtocolRead for ServerStatus {}

impl ProtocolWrite for ServerStatus {
    fn write<U: bytes::BufMut>(&self, dst: &mut U) -> Result<(), ProtocolError> {
        ProtocolWrite::write(&self.to_string(), dst)
    }
}