use crate::prelude::{ui::*, *};
use bevy::ecs::spawn::SpawnWith;

#[derive(Component, Clone, Copy)]
pub enum ButtonLabel {
    StartGame,
    ContinueGame,
    Settings,
    Quit,
    // Settings
    UpAudio,
    DownAudio,
    Audio,
    Controls,
    Other,
    Back,
    //Pause
    Continue,
    Save,
    ToMainMenu,
}

pub fn default_button(
    in_text: impl Into<String>,
    label: ButtonLabel,
    node_option: Option<Node>,
) -> impl Bundle {
    let button_colors = ButtonColors::default();
    let node = if let Some(node) = node_option {
        node
    } else {
        Node {
            width: Val::Percent(75.0),
            height: Val::Percent(20.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        }
    };
    let text = in_text.into();
    (
        Name::new(format!("default_button_{text}")),
        Button,
        label,
        BorderRadius::MAX,
        node,
        BackgroundColor(button_colors.normal),
        button_colors,
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent.spawn((
                Text::new(text),
                TextFont::from_font_size(40.),
                TextColor(BUTTON_TEXT_COLOR),
            ));
        })),
    )
}

pub fn small_button(in_text: impl Into<String>, label: ButtonLabel) -> impl Bundle {
    let button_colors = ButtonColors::default();
    let text = in_text.into();
    (
        Name::new(format!("small_button_{text}")),
        Button,
        label,
        BorderRadius::MAX,
        Node {
            width: Val::Px(160.0),
            height: Val::Px(40.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        BackgroundColor(button_colors.normal),
        button_colors,
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent.spawn((
                Text::new(text),
                TextFont::from_font_size(30.),
                TextColor(BUTTON_TEXT_COLOR),
            ));
        })),
    )
}
