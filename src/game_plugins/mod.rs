pub mod camera;
pub mod player;
pub mod ball;

use crate::game_plugins::{camera::CameraPlugin, player::PlayerPlugin};
use crate::prelude::*;

pub struct InGamePlugins;

impl Plugin for InGamePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, CameraPlugin));
    }
}
