// use crate::actions::{set_movement_actions, Actions};
use crate::loading::AudioAssets;
use crate::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_systems(OnEnter(GameStates::Playing), start_audio)
            .add_systems(
                Update,
                control_flying_sound
                    // .after(set_movement_actions) //TODO: new input move function
                    .run_if(in_state(GameStates::Playing).and(in_state(MenuStates::Disable))),
            );
    }
}

#[derive(Resource)]
pub struct FlyingAudio(pub Handle<AudioInstance>);

fn start_audio(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    global_volume: Res<GlobalVolume>,
    audio: Res<Audio>,
) {
    audio.pause();
    let handle = audio
        .play(audio_assets.flying.clone())
        .looped()
        .with_volume(Volume::from(global_volume.0))
        .handle();
    commands.insert_resource(FlyingAudio(handle));
}

fn control_flying_sound(
    intent: Res<MovementIntent>,
    audio: Res<FlyingAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    let is_playable_music = intent.0 != Vec2::ZERO;
    if let Some(instance) = audio_instances.get_mut(&audio.0) {
        match instance.state() {
            PlaybackState::Paused { .. } => {
                if is_playable_music {
                    instance.resume(AudioTween::default());
                }
            }
            PlaybackState::Playing { .. } => {
                if !is_playable_music {
                    instance.pause(AudioTween::default());
                }
            }
            _ => {}
        }
    }
}
