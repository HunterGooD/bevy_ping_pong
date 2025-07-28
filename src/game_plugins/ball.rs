use crate::prelude::*;

pub fn spawn_ball(mut commands: Commands, textures: Res<TextureAssets>,) {
    commands.spawn((
        Name::new("ball"),
        Sprite {
            image: textures.git_hub.clone(),
            custom_size: Some(Vec2::splat(100.0)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(400., 0., 1.)),
        RigidBody::Dynamic,
        Collider::circle(50.),
        Restitution::new(1.0),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        SweptCcd::LINEAR,
        MaxLinearSpeed(700.0),
        StateScoped(GameStates::Playing),
    ));
}