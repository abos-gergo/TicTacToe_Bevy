use bevy::prelude::*;
use bevy_pkv::PkvStore;

use crate::{
    User,
    menu::MenuState,
    theme::{button, ui_root},
};

pub(super) struct EnterNameMenuPlugin;
impl Plugin for EnterNameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuState::EnterName), spawn_enter_name_menu);
    }
}

fn spawn_enter_name_menu(mut commands: Commands) {
    commands.spawn((
        ui_root("Enter Name Menu"),
        GlobalZIndex(2),
        StateScoped(MenuState::EnterName),
        children![button("Save", save_name),],
    ));
}

fn save_name(
    _: Trigger<Pointer<Click>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut user: ResMut<User>,
    mut pkv: ResMut<PkvStore>,
) {
    info!("Saving user name");
    user.name = Some("PlayerName".to_string()); // Replace with actual input handling
    pkv.set(
        "user",
        &crate::StoredUser {
            name: user.name.clone().unwrap_or_default(),
        },
    )
    .unwrap();
    menu_state.set(MenuState::Main);
}
