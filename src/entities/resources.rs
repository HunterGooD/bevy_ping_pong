use crate::prelude::*;

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct PreviousMenuState(pub MenuStates);

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct GlobalVolume(pub f64);

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct MovementIntent(pub Vec2);
