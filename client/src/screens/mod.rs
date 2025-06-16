use bevy::prelude::*;

pub mod gameplay;
pub mod menu;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.add_plugins((menu::plugin, gameplay::plugin));
    app.add_systems(Startup, startup);
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum Screen {
    #[default]
    Menu,
    Loading,
    Gameplay,
}
