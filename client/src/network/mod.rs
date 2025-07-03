use bevy::prelude::*;
use bevy_quinnet::{
    client::{
        QuinnetClient, QuinnetClientPlugin, certificate::CertificateVerificationMode,
        connection::ClientEndpointConfiguration,
    },
    shared::channels::ChannelsConfiguration,
};
use shared::protocol::ClientMessage;

use crate::User;

pub struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(QuinnetClientPlugin::default())
            .insert_resource(NetworkStatus::default())
            .add_systems(Startup, start_connection)
            .add_systems(
                Update,
                register.run_if(|user: Res<User>, status: Res<NetworkStatus>| {
                    status.register_state == RegisterState::Unregistered && user.name.is_some()
                }),
            );
    }
}

//TODO: Refactor to state
#[derive(Resource, Debug, Clone, Default)]
struct NetworkStatus {
    pub register_state: RegisterState,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
enum RegisterState {
    /// The client name is not set.
    #[default]
    Unregistered,
    /// The client is in the process of registering, waiting for the server to respond.
    Registering,
    /// The client is registered and can send messages.
    Registered,
}

fn start_connection(mut client: ResMut<QuinnetClient>) {
    client
        .open_connection(
            ClientEndpointConfiguration::from_strings("127.0.0.1:6000", "0.0.0.0:0").unwrap(),
            CertificateVerificationMode::SkipVerification,
            ChannelsConfiguration::default(),
        )
        .unwrap();
}

fn register(mut client: ResMut<QuinnetClient>, user: Res<User>, mut status: ResMut<NetworkStatus>) {
    info!("Registering to server");
    client
        .connection_mut()
        .send_message(ClientMessage::Join {
            name: user.name.clone().unwrap(),
        })
        .unwrap();
    status.register_state = RegisterState::Registering;
}
