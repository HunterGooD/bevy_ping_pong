use crate::effects::explosion::get_explosion_bundle;
use crate::prelude::*;
use bevy_tweening::lens::TransformScaleLens;
use bevy_tweening::{Animator, Tween, TweenCompleted};
use std::time::Duration;

const END_ANIMATION: u64 = 8;

pub fn spawn_ball(mut commands: Commands, textures: Res<TextureAssets>) {
    let ball = get_ball(textures.git_hub.clone());
    commands.spawn(ball);
}

fn get_ball(ball: Handle<Image>) -> impl Bundle {
    let tween_scale = Tween::new(
        // CubicOut | CubicInOut
        EaseFunction::BounceOut,
        Duration::from_millis(300),
        TransformScaleLens {
            start: Vec3::splat(0.01),
            end: Vec3::ONE,
        },
    );
    (
        Name::new("ball"),
        Ball,
        Sprite {
            image: ball,
            custom_size: Some(Vec2::splat(100.0)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(400., 0., 0.1)),
        RigidBody::Dynamic,
        Collider::circle(50.),
        Restitution::new(1.0),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        SweptCcd::LINEAR,
        CollisionEventsEnabled,
        Animator::new(tween_scale),
        // LinearVelocity(Vec2::new(500.0, 300.0)),
        MaxLinearSpeed(1500.0),
        StateScoped(GameStates::Playing),
    )
}

pub fn ball_reset_check(
    mut commands: Commands,
    effects: Res<EffectAssets>,
    mut query: Query<
        (
            Entity,
            &mut LinearVelocity,
            &mut Animator<Transform>,
            &Transform,
        ),
        (With<Ball>, Without<InTweening>),
    >,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let width = window.width();
    let right_end = width / 2.0;
    let left_end = width / 2.0 - width;
    if let Ok((entity, mut velocity, mut animator, ball_position)) = query.single_mut() {
        if ball_position.translation.x < left_end || ball_position.translation.x > right_end {
            let tween_scale = Tween::new(
                // CubicOut | CubicInOut
                EaseFunction::BounceIn,
                Duration::from_millis(300),
                TransformScaleLens {
                    start: Vec3::ONE,
                    end: Vec3::splat(0.1),
                },
            )
            .with_completed_event(END_ANIMATION);
            info!("Start animation: ");
            // stop ball and start animation destroy
            velocity.0.x = 0.0;
            velocity.0.y = 0.0;
            let explosion = get_explosion_bundle(effects.explosion.clone());
            commands.spawn((
                explosion,
                Transform::from_xyz(
                    ball_position.translation.x.clamp(left_end, right_end),
                    ball_position.translation.y,
                    1.0,
                ),
            ));
            commands.entity(entity).insert(InTweening);
            animator.set_tweenable(tween_scale);
        }
    }
}

pub fn enable_interaction_after_initial_animation(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut reader: EventReader<TweenCompleted>,
) {
    for event in reader.read() {
        if event.user_data == END_ANIMATION {
            info!("Enabling interaction");
            commands.entity(event.entity).despawn();
            let ball = get_ball(textures.git_hub.clone());
            commands.spawn(ball);
        }
    }
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
    let new_speed = (speed * 2.0).min(max_speed);
    let direction = vel.xy().normalize_or_zero();
    let velocity = direction.extend(0.0) * new_speed;
    vel.x = velocity.x;
    vel.y = velocity.y;
}
