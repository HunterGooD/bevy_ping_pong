use bevy::prelude::*;

pub const BUTTON_TEXT_COLOR: Color = Color::linear_rgb(0.9, 0.9, 0.9);
pub const BACKGROUND_SETTING_COLOR: Color = Color::hsla(217., 0.0, 0.1, 0.5);

#[derive(Component)]
pub struct ButtonColors {
    pub(crate) normal: Color,
    pub(crate) hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::linear_rgb(0.15, 0.15, 0.15),
            hovered: Color::linear_rgb(0.25, 0.25, 0.25),
        }
    }
}
