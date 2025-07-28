pub mod components;
pub mod events;
pub mod game_states;
pub mod resources;

use crate::prelude::events::SaveSettingEvent;
use crate::prelude::*;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app
            // register types
            .register_type::<GlobalVolume>()
            .register_type::<GameStates>()
            .register_type::<MenuStates>()
            .register_type::<SettingsStates>()
            .register_type::<PlayerInput>()
            .register_type::<UiInput>()
            .register_type::<GlobalInput>()
            .register_type::<PreviousMenuState>()
            .register_type::<MovementIntent>()
            .register_type::<Player>()
            .register_type::<PlayerVisual>()
            .register_type::<TextureKind>()
            .register_type::<MovementDampingFactor>()
            .register_type::<CharacterController>()
            .register_type::<MaxSlopeAngle>()
            // initial states
            .init_state::<GameStates>()
            .enable_state_scoped_entities::<GameStates>()
            .init_state::<MenuStates>()
            .enable_state_scoped_entities::<MenuStates>()
            .init_state::<SettingsStates>()
            .enable_state_scoped_entities::<SettingsStates>()
            // initial events
            .add_event::<SaveSettingEvent>()
            .add_event::<SaveGameEvent>()
            .add_event::<LoadGameEvent>()
            // initial resources
            .insert_resource(PreviousMenuState(MenuStates::default()))
            .insert_resource(GlobalVolume(0.3))
            .insert_resource(MovementIntent(Vec2::ZERO));
    }
}
