use crate::prelude::*;

pub fn spawn_ball(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn((
        Name::new("ball"),
        Ball,
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
        CollisionEventsEnabled,
        LinearVelocity(Vec2::new(500.0, 300.0)),
        MaxLinearSpeed(1000.0),
        StateScoped(GameStates::Playing),
    ));
}

pub fn collide_ball(
    mut collision_events: EventReader<CollisionStarted>,
    mut ball_query: Query<(&mut LinearVelocity, &MaxLinearSpeed), With<Ball>>,
    wall_query: Query<Entity, With<Wall>>,
) {
    for CollisionStarted(collider1, collider2) in collision_events.read() {
        info!("Collision started! {collider1:?} collide with {collider2:?}");
        let wall_hit = wall_query.contains(*collider1) || wall_query.contains(*collider2);

        if wall_hit {
            if let Ok((mut vel, &max_speed)) = ball_query.get_mut(*collider1) {
                adjust_vel(&mut vel.0, max_speed.0);
            }
            if let Ok((mut vel, &max_speed)) = ball_query.get_mut(*collider2) {
                adjust_vel(&mut vel.0, max_speed.0);
            }
        }
    }
}

fn adjust_vel(vel: &mut Vector, max_speed: f32) {
    let speed = vel.xy().length();
    let new_speed = (speed * 1.40).min(max_speed);
    let direction = vel.xy().normalize_or_zero();
    let velocity = direction.extend(0.0) * new_speed;
    vel.x = velocity.x;
    vel.y = velocity.y;
}
