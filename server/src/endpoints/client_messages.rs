use bevy::prelude::*;
use bevy_quinnet::server::QuinnetServer;
use shared::protocol::{ClientMessage, ServerMessage};

use crate::{ConnectedUsers, Lobbies};

pub(super) fn handle_client_messages(
    mut server: ResMut<QuinnetServer>,
    mut connected_users: ResMut<ConnectedUsers>,
    lobbies: Res<Lobbies>,
) {
    let endpoint = server.endpoint_mut();

    for client_id in endpoint.clients() {
        while let Some((_channel_id, message)) =
            endpoint.try_receive_message_from::<ClientMessage>(client_id)
        {
            match message {
                ClientMessage::Join { name } => {
                    if connected_users.names.contains_key(&client_id) {
                        warn!(
                            "Received a Join from an already connected client: {}",
                            client_id
                        );
                        continue;
                    }
                    if connected_users.names.values().any(|n| n == &name) {
                        warn!("Username '{}' is already taken", name);
                        continue;
                    }

                    info!("{} connected", name);
                    connected_users.names.insert(client_id, name.clone());

                    endpoint
                        .send_message(
                            client_id,
                            ServerMessage::InitClient {
                                client_id: client_id,
                                lobbies: lobbies.to_vec(),
                            },
                        )
                        .unwrap();
                }
                ClientMessage::Disconnect {} => {
                    endpoint.disconnect_client(client_id).unwrap();

                    #[cfg(debug_assertions)]
                    if let Some(username) = connected_users.names.get(&client_id) {
                        info!("{} disconnected", username);
                    } else {
                        warn!(
                            "Received a Disconnect from an unknown or disconnected client: {}",
                            client_id
                        );
                    }

                    connected_users.names.remove(&client_id);
                }
                ClientMessage::CreateLobby {} => {
                    info!(
                        "{} created a new lobby",
                        connected_users.names.get(&client_id).unwrap()
                    );
                }
            }
        }
    }
}
