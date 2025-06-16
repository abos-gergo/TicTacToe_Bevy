use crate::screens::Screen;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), game);
}

fn game() {
    info!("Game started");
}
