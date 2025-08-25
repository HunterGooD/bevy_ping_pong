use crate::prelude::{ui::*, *};
use bevy_tweening::lens::UiPositionLens;
use bevy_tweening::{Animator, Delay, Tween};
use std::time::Duration;

pub struct GameOverPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuStates::GameOver), setup_game_over_screen)
            .add_systems(OnExit(MenuStates::GameOver), cleanup_menu);
    }
}

#[derive(Component)]
struct GameOverMenu;

// TODO: унифицировать создание меню и кнопок для него
fn setup_game_over_screen(mut commands: Commands) {
    info!("game over");

    commands
        .spawn((
            Name::new("Game over"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::End,
                ..default()
            },
            GlobalZIndex(11),
            GameOverMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Name::new("Game Over box"),
                    Node {
                        width: Val::Percent(30.0),
                        height: Val::Percent(40.0),
                        margin: UiRect::bottom(Val::Percent(5.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceAround,
                        ..default()
                    },
                ))
                .with_children(|parent_box| {
                    let mut buttons = vec![("Play Again", ButtonLabel::StartGame)];
                    buttons.push(("Settings", ButtonLabel::Settings));
                    buttons.push(("Quit", ButtonLabel::Quit));

                    let mut start_time_ms = 0;
                    for (label_name, button_action) in buttons {
                        let tween_scale = Tween::new(
                            // CubicOut | CubicInOut
                            EaseFunction::CubicOut,
                            Duration::from_millis(500),
                            UiPositionLens {
                                start: UiRect {
                                    left: Val::Px(-700.0), // Начальная позиция за левым краем
                                    // top: Val::Px(target_position),
                                    ..default()
                                },
                                end: UiRect {
                                    left: Val::Px(0.0), // Конечная позиция
                                    // top: Val::Px(target_position),
                                    ..default()
                                },
                            },
                        );
                        let animator = if start_time_ms > 0 {
                            let delay = Delay::new(Duration::from_millis(start_time_ms));
                            Animator::new(delay.then(tween_scale))
                        } else {
                            Animator::new(tween_scale)
                        };
                        start_time_ms += 150;
                        parent_box.spawn((
                            default_button(
                                label_name,
                                button_action,
                                Some(Node {
                                    left: Val::Px(-700.0),
                                    width: Val::Percent(75.0),
                                    height: Val::Percent(20.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..Default::default()
                                }),
                            ),
                            animator,
                        ));
                    }
                });
        });
}

// #[derive(Component)]
// struct ChangeState(GameState);

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<GameOverMenu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn();
    }
}
