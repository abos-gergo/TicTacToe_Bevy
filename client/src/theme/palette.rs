use bevy::prelude::*;

pub const LABEL_TEXT: Color = Color::srgb(0.867, 0.827, 0.412);
pub const HEADER_TEXT: Color = Color::srgb(0.988, 0.984, 0.800);
pub const BUTTON_TEXT: Color = Color::srgb(0.925, 0.925, 0.925);
pub const BUTTON_BACKGROUND: Color = Color::srgb(0.275, 0.400, 0.750);
pub const BUTTON_HOVERED_BACKGROUND: Color = Color::srgb(0.384, 0.600, 0.820);
pub const BUTTON_PRESSED_BACKGROUND: Color = Color::srgb(0.239, 0.286, 0.600);

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

fn apply_interaction_palette(
    mut palette_query: Query<
        (&Interaction, &InteractionPalette, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, palette, mut background) in &mut palette_query {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        }
        .into();
    }
}
pub fn plugin(app: &mut App) {
    app.add_systems(Update, apply_interaction_palette);
}
