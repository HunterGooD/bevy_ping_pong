#![allow(clippy::type_complexity)]
mod audio;
mod effects;
mod entities;
mod event_managers;
mod game_plugins;
mod input;
mod loading;
mod prelude;
mod save_manager;
mod ui;
mod utils;

use crate::prelude::{ui::*, *};
use bevy_tweening::TweeningPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity(Vector::ZERO)).add_plugins((
            EntityPlugin,
            LoadingPlugin,
            PhysicsPlugins::default().with_length_unit(20.0),
            TweeningPlugin,
            EffectsPlugin,
            SettingSaveManagerPlugin,
            GameSaveManagerPlugin,
            UIPlugin,
            InGamePlugins,
            InputPlugin,
            InternalAudioPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                PhysicsDebugPlugin::default(),
                FrameTimeDiagnosticsPlugin::default(),
                LogDiagnosticsPlugin::default(),
                EguiPlugin::default(),
                WorldInspectorPlugin::new(),
            ));
        }
    }
}
