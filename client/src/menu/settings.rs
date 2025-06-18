use bevy::prelude::*;
use crate::{menu::MenuState, theme::{button, ui_root}};

pub(super) struct  SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(OnEnter(MenuState::Settings), spawn_settings_menu);
    }
}

fn spawn_settings_menu(mut commands: Commands) {
    commands.spawn((
        ui_root("Main Menu"),
        GlobalZIndex(2),
        StateScoped(MenuState::Settings),
        children![button("Placeholder", placeholder_fn), button("Back", enter_main_menu)],
    ));
}

fn placeholder_fn(_: Trigger<Pointer<Click>>) {

}

fn enter_main_menu(_: Trigger<Pointer<Click>>, mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main)
}