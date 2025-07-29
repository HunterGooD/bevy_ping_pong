use crate::prelude::*;
use bevy_enoki::prelude::*;

pub fn get_explosion_bundle(effect: Handle<Particle2dEffect>) -> impl Bundle {
    (
        Name::new("explosion"),
        ParticleSpawner::default(),
        OneShot::Despawn,
        ParticleEffectHandle(effect),
    )
}
