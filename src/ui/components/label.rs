use crate::prelude::{ui::*, *};

pub fn label(txt: impl Into<String>) -> impl Bundle {
    let text = txt.into();
    (
        Name::new("label_text"),
        Text::new(text),
        TextFont {
            font_size: 40.0,
            ..default()
        },
        TextColor(BUTTON_TEXT_COLOR),
    )
}
