use crate::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    // Movement
    #[actionlike(DualAxis)]
    Move,
    UseItem,
}

impl PlayerAction {
    /// Define the default bindings to the input
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map.insert_dual_axis(Self::Move, GamepadStick::LEFT);
        input_map.insert(Self::UseItem, GamepadButton::North);

        input_map.insert_dual_axis(Self::Move, VirtualDPad::wasd());
        input_map.insert(Self::UseItem, KeyCode::KeyE);

        input_map
    }
}

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Startup, setup_player_input)
            .add_systems(
                Update,
                player_input_intent
                    .run_if(in_state(GameStates::Playing))
                    .run_if(in_state(MenuStates::Disable)),
            );
    }
}

fn setup_player_input(mut commands: Commands) {
    commands.spawn((
        Name::new("player input"),
        PlayerInput,
        PlayerAction::default_input_map(),
    ));
}

fn player_input_intent(
    mut movement_intent: ResMut<MovementIntent>,
    query: Query<&ActionState<PlayerAction>, With<PlayerInput>>,
) {
    let action_state = query.single().expect("Player actions not found");
    let axis = action_state.clamped_axis_pair(&PlayerAction::Move).xy();
    movement_intent.0 = axis;

    if action_state.just_pressed(&PlayerAction::UseItem) {
        println!("Used an Item!");
    }
}
