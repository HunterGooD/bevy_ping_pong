use crate::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    // Movement
    #[actionlike(DualAxis)]
    Move,
    SpeedBoost,
}

impl PlayerAction {
    pub fn default_player_one() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map.insert_dual_axis(Self::Move, GamepadStick::LEFT);
        input_map.insert(Self::SpeedBoost, GamepadButton::North);

        input_map.insert_dual_axis(Self::Move, VirtualDPad::wasd());
        input_map.insert(Self::SpeedBoost, KeyCode::ShiftLeft);

        input_map
    }

    pub fn default_player_two() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map.insert_dual_axis(Self::Move, GamepadStick::LEFT);
        input_map.insert(Self::SpeedBoost, GamepadButton::North);

        input_map.insert_dual_axis(Self::Move, VirtualDPad::arrow_keys());
        input_map.insert(Self::SpeedBoost, KeyCode::ShiftRight);

        input_map
    }
}

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(
                Update,
                player_input_intent
                    .run_if(in_state(GameStates::Playing))
                    .run_if(in_state(MenuStates::Disable)),
            );
    }
}

fn player_input_intent(
    mut move_event: EventWriter<MoveEvent>,
    mut boost_event: EventWriter<SpeedBoostEvent>,
    query: Query<(Entity, &ActionState<PlayerAction>), With<Player>>,
) {
    for (entity, action_state) in query.iter() {
        let axis = action_state.clamped_axis_pair(&PlayerAction::Move).xy();
        move_event.write(MoveEvent {
            entity,
            move_intent: axis,
        });
        if action_state.just_pressed(&PlayerAction::SpeedBoost) {
            boost_event.write(SpeedBoostEvent {
                entity,
                speed_boost: 200.0,
            });
        }
        if action_state.just_released(&PlayerAction::SpeedBoost) {
            boost_event.write(SpeedBoostEvent {
                entity,
                speed_boost: -200.0,
            });
        }
    }
}
