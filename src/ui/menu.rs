use crate::prelude::{ui::*, utils::file::saves_exists, *};
use crate::save_manager::game::FILE_GAME_SAVE;
use bevy_tweening::lens::UiPositionLens;
use bevy_tweening::{Animator, Delay, Tween};
use std::time::Duration;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuStates::MainMenu), setup_menu)
            .add_systems(OnExit(MenuStates::MainMenu), cleanup_menu);
    }
}

#[derive(Component)]
struct Menu;

// TODO: унифицировать создание меню и кнопок для него
fn setup_menu(mut commands: Commands) {
    info!("menu");
    let mut is_continue_button = false;
    let mut height = 40.0;
    if saves_exists(FILE_GAME_SAVE) {
        println!("Loading game");
        height += 10.0;
        is_continue_button = true;
    }

    commands
        .spawn((
            Name::new("Main menu"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::End,
                ..default()
            },
            GlobalZIndex(11),
            Menu,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Name::new("Main menu box"),
                    Node {
                        width: Val::Percent(30.0),
                        height: Val::Percent(height),
                        margin: UiRect::bottom(Val::Percent(5.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceAround,
                        ..default()
                    },
                ))
                .with_children(|parent_box| {
                    let mut buttons = vec![("Play", ButtonLabel::StartGame)];
                    if is_continue_button {
                        buttons.push(("Continue", ButtonLabel::ContinueGame));
                    }
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

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn();
    }
}
