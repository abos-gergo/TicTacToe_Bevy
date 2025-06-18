use bevy::prelude::*;
use crate::ScreenState;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ScreenState::Game), on_enter_game);
    }
}

fn on_enter_game() {
    info!("State Game entered");
}
