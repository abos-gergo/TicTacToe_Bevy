use bevy::{ecs::system::command::insert_resource, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_pkv::PkvStore;
use bevy_quinnet::shared::ClientId;
use serde::{Deserialize, Serialize};

use crate::{
    game::GamePlugin, menu::MenuPlugin, network::NetworkPlugin, theme::palette::UIThemePlugin,
};

mod game;
mod menu;
mod network;
mod theme;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(User::default())
        .insert_resource(PkvStore::new("TodoOrgName", "TicTacToe"))
        .init_state::<ScreenState>()
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(UIThemePlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(NetworkPlugin)
        .add_systems(PreStartup, load_user)
        .add_systems(Startup, startup)
        .run();
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
#[states(scoped_entities)]
pub enum ScreenState {
    #[default]
    Menu,
    Game,
    Loading,
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Resource, Default)]
struct User {
    name: Option<String>,
    client_id: Option<ClientId>,
}

#[derive(Serialize, Deserialize, Debug)]
struct StoredUser {
    name: String,
}

fn load_user(store: Res<PkvStore>, mut user: ResMut<User>) {
    match store.get::<StoredUser>("user") {
        Ok(stored_user) => {
            user.name = Some(stored_user.name);
        }
        Err(get_error) => {
            eprint!("{:?}", get_error);
        }
    }
    info!(user.name);
}
