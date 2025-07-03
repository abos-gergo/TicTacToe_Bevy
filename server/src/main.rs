use std::collections::HashMap;

use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*};
use bevy_quinnet::{
    server::{QuinnetServer, ServerEndpointConfiguration, certificate::CertificateRetrievalMode},
    shared::{ClientId, channels::ChannelsConfiguration},
};

use shared::components::Lobby;

use crate::endpoints::EndpointsPlugin;

mod endpoints;

#[derive(Resource, Debug, Clone, Default)]
struct ConnectedUsers {
    names: HashMap<ClientId, String>,
}

#[derive(Resource, Debug, Clone, Default, Deref, DerefMut)]
struct Lobbies(Vec<Lobby>);

fn main() {
    App::new()
        .add_plugins((ScheduleRunnerPlugin::default(), LogPlugin::default()))
        .add_plugins(EndpointsPlugin)
        .add_systems(Startup, start_listening)
        .insert_resource(Lobbies::default())
        .insert_resource(ConnectedUsers::default())
        .run();
}

fn start_listening(mut server: ResMut<QuinnetServer>) {
    server
        .start_endpoint(
            ServerEndpointConfiguration::from_string("0.0.0.0:6000").unwrap(),
            CertificateRetrievalMode::GenerateSelfSigned {
                server_hostname: "TODO: REPLACE_TO_CERT".to_string(),
            },
            ChannelsConfiguration::default(),
        )
        .unwrap();
}
