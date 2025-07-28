use crate::prelude::{ui::*, *};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuStates::Setting), setup_settings)
            .add_systems(
                Update,
                (
                    (audio_settings, update_global_volume_label)
                        .run_if(in_state(SettingsStates::Audio)),
                    control_settings.run_if(in_state(SettingsStates::Controls)),
                    other_settings.run_if(in_state(SettingsStates::Other)),
                )
                    .run_if(in_state(MenuStates::Setting)),
            )
            .add_systems(OnExit(MenuStates::Setting), cleanup_settings);
    }
}

#[derive(Component)]
struct Setting;

#[derive(Component)]
struct SettingArea;

fn setup_settings(mut commands: Commands) {
    info!("settings");
    commands
        .spawn((
            Name::new("Main Settings menu"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                ..default()
            },
            GlobalZIndex(11),
            Setting,
        ))
        .with_child((
            Name::new("Header Settings menu"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(95.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            children![
                // header
                (
                    Node {
                        width: Val::Percent(90.0),
                        height: Val::Percent(10.0),
                        border: UiRect::all(Val::Px(2.0)),
                        padding: UiRect::all(Val::Percent(1.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Start,
                        ..default()
                    },
                    BorderRadius::all(Val::Px(30.)),
                    BackgroundColor(BACKGROUND_SETTING_COLOR),
                    children![
                        small_button("Audio", ButtonLabel::Audio),
                        Node {
                            width: Val::Percent(1.0),
                            ..default()
                        },
                        small_button("Controls", ButtonLabel::Controls),
                        Node {
                            width: Val::Percent(1.0),
                            ..default()
                        },
                        small_button("Others", ButtonLabel::Other),
                    ],
                ),
                // menu settings
                (
                    Name::new("Setting menu"),
                    Node {
                        width: Val::Percent(90.0),
                        height: Val::Percent(78.0),
                        border: UiRect::all(Val::Px(2.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BorderColor(Color::hsla(44., 0.0, 1.0, 0.6)),
                    BorderRadius::all(Val::Px(30.)),
                    BackgroundColor(BACKGROUND_SETTING_COLOR),
                    SettingArea,
                ),
                // actions buttons
                (
                    Name::new("Bottom Settings button"),
                    Node {
                        width: Val::Percent(90.0),
                        height: Val::Percent(10.0),
                        border: UiRect::all(Val::Px(2.0)),
                        padding: UiRect::all(Val::Percent(1.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Start,
                        ..default()
                    },
                    BorderRadius::all(Val::Px(30.)),
                    BackgroundColor(BACKGROUND_SETTING_COLOR),
                    children![small_button("Back", ButtonLabel::Back),],
                ),
            ],
        ));
}

#[derive(Component)]
struct AudioSetting;
fn audio_settings(
    mut commands: Commands,
    menu: Query<Entity, With<SettingArea>>,
    audio_setting: Query<Entity, With<AudioSetting>>,
) {
    if !audio_setting.is_empty() {
        return;
    }
    info!("audio_settings");

    let entity = menu.single().unwrap();
    commands.entity(entity).with_child((
        (
            Name::new("Settings Grid"),
            Node {
                display: Display::Grid,
                row_gap: Val::Px(10.0),
                column_gap: Val::Px(30.0),
                grid_template_columns: RepeatedGridTrack::px(2, 400.0),
                ..default()
            },
            children![
                (
                    label("Master Volume"),
                    Node {
                        justify_self: JustifySelf::End,
                        ..default()
                    }
                ),
                (
                    Name::new("Global Volume Widget"),
                    Node {
                        justify_self: JustifySelf::Start,
                        ..default()
                    },
                    children![
                        small_button("-", ButtonLabel::DownAudio),
                        (
                            Name::new("Current Volume"),
                            Node {
                                padding: UiRect::horizontal(Val::Px(10.0)),
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            children![(label(""), GlobalVolumeLabel)],
                        ),
                        small_button("+", ButtonLabel::UpAudio)
                    ],
                ),
            ],
        ),
        StateScoped(SettingsStates::Audio),
        AudioSetting,
    ));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct GlobalVolumeLabel;

fn update_global_volume_label(
    global_volume: Res<GlobalVolume>,
    mut label: Single<&mut Text, With<GlobalVolumeLabel>>,
) {
    let percent = 100.0 * global_volume.0;
    label.0 = format!("{percent:3.0}%");
}

#[derive(Component)]
struct ControlSetting;
fn control_settings(
    mut commands: Commands,
    menu: Query<Entity, With<SettingArea>>,
    control_setting: Query<Entity, With<ControlSetting>>,
) {
    if !control_setting.is_empty() {
        return;
    }
    info!("controls_settings");

    let entity = menu.single().unwrap();
    commands.entity(entity).with_child((
        label("Controls menu"),
        StateScoped(SettingsStates::Controls),
        ControlSetting,
    ));
}

#[derive(Component)]
struct OtherSetting;
fn other_settings(
    mut commands: Commands,
    menu: Query<Entity, With<SettingArea>>,
    other_setting: Query<Entity, With<OtherSetting>>,
) {
    if !other_setting.is_empty() {
        return;
    }
    info!("other_settings");

    let entity = menu.single().unwrap();
    commands.entity(entity).with_child((
        label("Other menu"),
        StateScoped(SettingsStates::Other),
        OtherSetting,
    ));
}

fn cleanup_settings(mut commands: Commands, menu: Query<Entity, With<Setting>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn();
    }
}
