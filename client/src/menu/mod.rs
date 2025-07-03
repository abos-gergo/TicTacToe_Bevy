use crate::{
    ScreenState, User,
    menu::{
        enter_name::EnterNameMenuPlugin, main_menu::MainMenuPlugin, settings::SettingsMenuPlugin,
    },
};
use bevy::prelude::*;

mod enter_name;
mod main_menu;
mod settings;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ScreenState::Menu), on_enter_menu);
        app.add_systems(OnExit(ScreenState::Menu), on_exit_menu);
        app.init_state::<MenuState>();
        app.add_plugins(MainMenuPlugin);
        app.add_plugins(SettingsMenuPlugin);
        app.add_plugins(EnterNameMenuPlugin);
    }
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum MenuState {
    #[default]
    None,
    Main,
    Settings,
    EnterName,
}

fn on_enter_menu(mut menu_state: ResMut<NextState<MenuState>>, user: Res<User>) {
    info!(user.name);
    if user.name.is_some() {
        menu_state.set(MenuState::Main);
    } else {
        menu_state.set(MenuState::EnterName);
    }
}

fn on_exit_menu(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::None);
}
