pub(crate) use avian2d::{math::*, prelude::*};
#[cfg(debug_assertions)]
pub(crate) use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
pub(crate) use bevy::{prelude::*, window::PrimaryWindow};
#[cfg(debug_assertions)]
pub(crate) use bevy_inspector_egui::bevy_egui::EguiPlugin;
pub(crate) use leafwing_input_manager::prelude::*;
pub(crate) use moonshine_save::prelude::*;
pub(crate) use moonshine_util::prelude::*;
#[cfg(target_arch = "wasm32")]
pub(crate) use wasm_bindgen::prelude::*;
// TODO: unused
// #[cfg(debug_assertions)]
// pub(crate) use bevy_inspector_egui::prelude::*;
#[cfg(debug_assertions)]
pub(crate) use bevy_inspector_egui::quick::WorldInspectorPlugin;

// LOCAL IMPORTS
pub(crate) use crate::effects::EffectsPlugin;
pub(crate) use crate::entities::components::*;
pub(crate) use crate::entities::events::*;
/**
    maybe use this structure for detailed imports

    pub(crate) mod resources {
        pub(crate) use crate::entities::resources::*;
    }
    pub(crate) mod components {
        pub(crate) use crate::entities::components::*;
    }
    pub(crate) mod state {
        pub(crate) use crate::entities::game_states::*;
    }
**/
pub(crate) use crate::entities::game_states::*;
pub(crate) use crate::entities::resources::*;
pub(crate) use crate::entities::*;
pub(crate) use crate::save_manager::{game::*, settings::*};
pub(crate) mod utils {
    pub(crate) use crate::utils::*;
}
pub(crate) mod ui {
    pub(crate) use crate::ui::components::button::*;
    pub(crate) use crate::ui::components::label::*;
    pub(crate) use crate::ui::components::style::*;
    pub(crate) use crate::ui::*;
}
pub(crate) use crate::audio::*;
pub(crate) use crate::game_plugins::*;
pub(crate) use crate::input::*;
pub(crate) use crate::loading::*;
// pub(crate) use crate::event_managers::state_manager::*;
