// TODO: Spawn Camera and other actions with her
use crate::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameStates::Loading), setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Name::new("Main camera"), Camera2d, Msaa::Off));
}
