use bevy::prelude::*;

use crate::{
    menu::Menu,
    screens::Screen,
    theme::{button, ui_root},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main);
}

pub fn spawn_main(mut commands: Commands) {
    commands.spawn((
        ui_root("Main Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Main),
        children![button("Find Match", find_match), button("Exit", exit_app)],
    ));
}

pub fn find_match(_: Trigger<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}

fn exit_app(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
