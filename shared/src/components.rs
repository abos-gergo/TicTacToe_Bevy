use bevy_quinnet::shared::ClientId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lobby {
    pub host_player_id: ClientId,
    pub join_player_id: Option<ClientId>,
}
