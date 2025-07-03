use bevy::prelude::*;
use bevy_quinnet::server::QuinnetServerPlugin;
use client_messages::handle_client_messages;

mod client_messages;

pub struct EndpointsPlugin;
impl Plugin for EndpointsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(QuinnetServerPlugin::default())
            .add_systems(Update, handle_client_messages);
    }
}
