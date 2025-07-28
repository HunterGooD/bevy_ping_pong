use crate::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum GlobalAction {
    // Movement
    Pause,
}

impl GlobalAction {
    /// Define the default bindings to the input
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map.insert(Self::Pause, GamepadButton::Start);

        input_map.insert(Self::Pause, KeyCode::Escape);

        input_map
    }
}

pub struct GlobalInputPlugin;

impl Plugin for GlobalInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui_input)
            .add_systems(
                Update,
                process_global_input
                    .run_if(in_state(GameStates::Playing).and(in_state(MenuStates::Disable))),
            )
            .add_plugins(InputManagerPlugin::<GlobalAction>::default());
    }
}

fn setup_ui_input(mut commands: Commands) {
    commands.spawn((
        Name::new("global input"),
        GlobalInput,
        GlobalAction::default_input_map(),
    ));
}

fn process_global_input(
    state: Res<State<MenuStates>>,
    mut movement_intent: ResMut<MovementIntent>,
    mut next_state: ResMut<NextState<MenuStates>>,
    query: Query<&ActionState<GlobalAction>, With<GlobalInput>>,
) {
    let action_state = query.single().expect("Global actions not found");

    if action_state.just_pressed(&GlobalAction::Pause) && state.get() == &MenuStates::Disable {
        movement_intent.0 = Vec2::ZERO;
        next_state.set(MenuStates::PauseMenu)
    }
}
