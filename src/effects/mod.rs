pub mod test_effect;

use crate::effects::test_effect::*;
use crate::prelude::*;
use bevy_enoki::prelude::*;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EnokiPlugin, TestFirstEffect));
    }
}
