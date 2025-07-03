use bevy_quinnet::shared::ClientId;
use serde::{Deserialize, Serialize};

use crate::components::Lobby;

// Messages from clients
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    Join { name: String },
    Disconnect {},
    CreateLobby {},
}

// Messages from the server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    InitClient {
        client_id: ClientId,
        lobbies: Vec<Lobby>,
    },
}
