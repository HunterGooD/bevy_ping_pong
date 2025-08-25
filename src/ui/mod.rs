use crate::prelude::{ui::*, *};
use crate::ui::score::{setup_score, update_score, update_timer, update_timer_text};
use crate::ui::{game_over::*, menu::*, pause::*, settings::*};
use bevy_kira_audio::prelude::*;

pub mod components;
pub mod game_over;
pub mod menu;
pub mod pause;
pub mod score;
pub mod settings;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PauseMenuPlugin, SettingsPlugin, MenuPlugin, GameOverPlugin))
            .add_systems(
                Update,
                button_processing.run_if(not(in_state(MenuStates::Disable))),
            )
            .add_systems(OnEnter(GameStates::Playing), setup_score)
            .add_systems(
                Update,
                (update_score, update_timer, update_timer_text)
                    .run_if(in_state(GameStates::Playing).and(in_state(MenuStates::Disable))),
            );
    }
}

// TODO: отсылать ивенты для смены состояний и прочих действий (удалить игнорирование линта)
// так как тестовая функция тут куча всего
#[allow(clippy::too_many_arguments)]
fn button_processing(
    audio: Res<Audio>,
    mut save_event: EventWriter<SaveSettingEvent>,
    mut save_game_event: EventWriter<SaveGameEvent>,
    mut load_game_event: EventWriter<LoadGameEvent>,
    mut global_volume: ResMut<GlobalVolume>,
    mut previous_state: ResMut<PreviousMenuState>,
    mut app_exit: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut settings_next_state: ResMut<NextState<SettingsStates>>,
    game_state: Res<State<MenuStates>>,
    mut next_state_menu: ResMut<NextState<MenuStates>>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            &ButtonLabel,
            // Option<&ChangeState>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (_, interaction, mut color, button_colors, button_label) in &mut interaction_query {
        match *interaction {
            // TODO: send event for change states
            Interaction::Pressed => match button_label {
                ButtonLabel::StartGame => {
                    next_state_menu.set(MenuStates::Disable);
                    next_state.set(GameStates::LoadingGame);
                }
                ButtonLabel::ContinueGame => {
                    next_state_menu.set(MenuStates::Disable);
                    next_state.set(GameStates::LoadingGame);
                    load_game_event.write_default();
                }
                ButtonLabel::Continue => {
                    next_state_menu.set(MenuStates::Disable);
                }
                ButtonLabel::Settings => {
                    let current_game_state = game_state.get();
                    previous_state.0 = current_game_state.clone();
                    next_state_menu.set(MenuStates::Setting);
                    info!("setting pressed");
                }
                ButtonLabel::Save => {
                    save_game_event.write(SaveGameEvent);
                }
                ButtonLabel::Audio => settings_next_state.set(SettingsStates::Audio),
                ButtonLabel::Controls => settings_next_state.set(SettingsStates::Game),
                ButtonLabel::Other => settings_next_state.set(SettingsStates::Other),
                ButtonLabel::Back => {
                    next_state_menu.set(previous_state.0.clone());
                }
                ButtonLabel::ToMainMenu => {
                    next_state.set(GameStates::Menu);
                    next_state_menu.set(MenuStates::MainMenu);
                }
                ButtonLabel::Quit => {
                    app_exit.write(AppExit::Success);
                }
                ButtonLabel::UpAudio => {
                    global_volume.0 += 0.1;
                    global_volume.0 = global_volume.0.clamp(0.0, 1.0);
                    audio.set_volume(Volume::from(global_volume.0));
                    save_event.write(SaveSettingEvent);
                }
                ButtonLabel::DownAudio => {
                    global_volume.0 -= 0.1;
                    global_volume.0 = global_volume.0.clamp(0.0, 1.0);
                    audio.set_volume(Volume::from(global_volume.0));
                    save_event.write(SaveSettingEvent);
                }
            },
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}
