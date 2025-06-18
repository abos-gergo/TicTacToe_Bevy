use bevy::{
    app::{App, Plugin}, ecs::system::{ResMut}, state::{app::AppExtStates, state::{NextState, OnEnter, OnExit, States}}
};
use crate::{menu::{main_menu::MainMenuPlugin, settings::SettingsMenuPlugin}, ScreenState};

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
    }
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum MenuState {
    #[default]
    None,
    Main,
    Settings,
}

fn on_enter_menu(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn on_exit_menu(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::None);
}