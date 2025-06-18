use bevy::prelude::*;

use crate::{
    menu::MenuState,
    theme::{button, ui_root}, ScreenState,
};

pub(super) struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuState::Main), spawn_main_menu);
    }
}

fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((
        ui_root("Main Menu"),
        GlobalZIndex(2),
        StateScoped(MenuState::Main),
        children![button("Find Match", find_match), button("Settings", enter_settings_menu), button("Exit", exit_app)],
    ));
}

fn find_match(_: Trigger<Pointer<Click>>, mut screen_state: ResMut<NextState<ScreenState>>) {
    screen_state.set(ScreenState::Game);
}

fn enter_settings_menu(_: Trigger<Pointer<Click>>, mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Settings)
}

fn exit_app(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
