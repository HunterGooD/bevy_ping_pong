use crate::prelude::*;
use bevy_enoki::prelude::*;

pub struct TestFirstEffect;

impl Plugin for TestFirstEffect {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStates::Menu), setup_effect);
    }
}

fn setup_effect(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        ParticleSpawner::default(),
        ParticleEffectHandle(asset_server.load("effects/test_effect.ron")),
    ));
}
