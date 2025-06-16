use bevy::prelude::*;

mod menu;
pub mod screens;
pub mod theme;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((screens::plugin, menu::plugin, theme::palette::plugin))
        .run();
}
