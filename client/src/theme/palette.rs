use bevy::prelude::*;

pub const LABEL_TEXT: Color = Color::srgb(0.95, 0.95, 0.90);
pub const HEADER_TEXT: Color = Color::srgb(0.20, 0.30, 0.25);
pub const BUTTON_TEXT: Color = Color::srgb(0.70, 0.75, 0.68);
pub const BUTTON_BACKGROUND: Color = Color::srgb(0.18, 0.35, 0.25);
pub const BUTTON_HOVERED_BACKGROUND: Color = Color::srgb(0.25, 0.45, 0.30);
pub const BUTTON_PRESSED_BACKGROUND: Color = Color::srgb(0.12, 0.25, 0.18);

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

pub struct UIThemePlugin;
impl Plugin for UIThemePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_interaction_palette);
    }
}
