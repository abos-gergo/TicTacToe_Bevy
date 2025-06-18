use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::{menu::MenuPlugin, game::GamePlugin, theme::palette::UIThemePlugin};

mod menu;
pub mod game;
pub mod theme;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin {enable_multipass_for_primary_context: true})
        .add_plugins(WorldInspectorPlugin::default())
        .init_state::<ScreenState>()
        .add_plugins(UIThemePlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(GamePlugin)
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