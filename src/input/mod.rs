pub mod global;
pub mod player;
pub mod ui;

use crate::input::{global::*, player::*, ui::*};
use crate::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerInputPlugin, UiInputPlugin, GlobalInputPlugin));
    }
}
