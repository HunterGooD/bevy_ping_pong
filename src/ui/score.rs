use crate::prelude::ui::{custom_label, header_label};
use crate::prelude::*;

pub fn setup_score(mut commands: Commands) {
    commands
        .spawn((
            Name::new("score"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            StateScoped(GameStates::Playing),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Name::new("left score"),
                    Node {
                        width: Val::Percent(45.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Start,
                        justify_content: JustifyContent::End,
                        ..Default::default()
                    },
                ))
                .with_child((ScoreCounter, header_label("0")));
            parent
                .spawn((
                    Name::new("center timer"),
                    Node {
                        width: Val::Percent(10.0),
                        height: Val::Percent(100.0),
                        padding: UiRect::top(Val::Px(35.0)),
                        // align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                ))
                .with_child((CountdownTimerText, custom_label("01:00", 30.0)));
            parent
                .spawn((
                    Name::new("right score"),
                    Node {
                        width: Val::Percent(45.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Start,
                        justify_content: JustifyContent::Start,
                        ..Default::default()
                    },
                ))
                .with_child((ScoreCounter, header_label("1")));
        });
}

pub fn update_score(
    scores: Res<Scores>,
    mut query: Query<(&mut Text, &Transform), With<ScoreCounter>>,
) {
    for (mut text, transform) in query.iter_mut() {
        if transform.translation.x > 0.0 {
            text.0 = format!("{}", scores.left_score);
        } else {
            text.0 = format!("{}", scores.right_score);
        }
    }
}

pub fn update_timer(time: Res<Time>, mut countdown_timer: ResMut<CountdownTimer>) {
    countdown_timer.timer.tick(time.delta());
}

pub fn update_timer_text(
    countdown_timer: Res<CountdownTimer>,
    mut query: Query<&mut Text, With<CountdownTimerText>>,
) {
    if countdown_timer.timer.finished() {
        return;
    }

    let remaining = countdown_timer.timer.remaining_secs();
    let minutes = (remaining / 60.0).floor() as u32;
    let seconds = (remaining % 60.0).floor() as u32;

    for mut text in &mut query {
        text.0 = format!("{minutes:02}:{seconds:02}");
    }
}
