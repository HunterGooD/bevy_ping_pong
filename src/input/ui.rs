use crate::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum UiAction {
    // Movement
    Up,
    Down,
    Confirm,
    Back,
}

impl UiAction {
    /// Define the default bindings to the input
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map.insert(Self::Up, GamepadButton::DPadUp);
        input_map.insert(Self::Down, GamepadButton::DPadDown);
        input_map.insert(Self::Confirm, GamepadButton::South);
        input_map.insert(Self::Back, GamepadButton::East);

        input_map.insert(Self::Up, KeyCode::KeyW);
        input_map.insert(Self::Down, KeyCode::KeyS);
        input_map.insert(Self::Confirm, KeyCode::Space);
        input_map.insert(Self::Back, KeyCode::Escape);

        input_map
    }
}

pub struct UiInputPlugin;

impl Plugin for UiInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui_input)
            .add_systems(
                Update,
                process_ui_input.run_if(not(in_state(MenuStates::Disable))),
            )
            .add_plugins(InputManagerPlugin::<UiAction>::default());
    }
}

fn setup_ui_input(mut commands: Commands) {
    commands.spawn((
        Name::new("ui input"),
        UiInput,
        UiAction::default_input_map(),
    ));
}

fn process_ui_input(
    state: Res<State<MenuStates>>,
    previous_state: Res<PreviousMenuState>,
    mut app_exit: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<MenuStates>>,
    query: Query<&ActionState<UiAction>, With<UiInput>>,
) {
    let action_state = query.single().expect("Global actions not found");

    if action_state.just_pressed(&UiAction::Up) {
        info!("TODO: Up in ui")
    }

    if action_state.just_pressed(&UiAction::Down) {
        info!("TODO: Down in ui")
    }

    if action_state.just_pressed(&UiAction::Confirm) {
        info!("TODO: Confirm in ui")
    }

    if action_state.just_pressed(&UiAction::Back) {
        info!("Back in ui");
        match state.get() {
            MenuStates::PauseMenu => next_state.set(MenuStates::Disable),
            MenuStates::Setting => next_state.set(previous_state.0.clone()),
            MenuStates::MainMenu => {
                app_exit.write(AppExit::Success);
            }
            _ => (),
        }
    }
}
