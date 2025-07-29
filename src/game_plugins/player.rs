use crate::game_plugins::ball::*;
use crate::input::player::PlayerAction;
use crate::prelude::*;
pub struct PlayerPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameStates::LoadingGame),
            (spawn_ball, spawn_player).chain(),
        )
        .add_systems(OnExit(GameStates::LoadingGame), restore_sprites)
        .add_systems(
            Update,
            (
                move_player,
                apply_movement_damping,
                ball_reset_check,
                enable_interaction_after_initial_animation,
            )
                .chain()
                .run_if(in_state(GameStates::Playing))
                .run_if(in_state(MenuStates::Disable)),
        )
        // physic update
        .add_systems(
            PhysicsSchedule,
            (collide_ball, collide_player_with_ball)
                .chain()
                .run_if(in_state(GameStates::Playing))
                .run_if(in_state(MenuStates::Disable))
                .in_set(NarrowPhaseSet::Update),
        )
        .add_systems(OnExit(MenuStates::Disable), pause_physics)
        .add_systems(OnEnter(MenuStates::Disable), resume_physics)
        .add_systems(
            PhysicsSchedule,
            kinematic_controller_collisions.in_set(NarrowPhaseSet::Last),
        );
    }
}

fn spawn_player(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameStates>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let width = window.width();
    let height = window.height();
    info!("width {width}, height {height}");
    let size = Vec2::new(32.0, 256.0);
    commands.spawn((
        Name::new("player one"),
        PlayerVisual {
            texture_kind: TextureKind::Player,
            color: Color::WHITE,
            size,
        },
        Transform::from_translation(Vec3::new(width / 2.0 - width + size.x, 0., 1.)),
        Player::One,
        PlayerAction::default_player_one(),
    ));

    commands.spawn((
        Name::new("player two"),
        PlayerVisual {
            texture_kind: TextureKind::Player,
            color: Color::WHITE,
            size,
        },
        Transform::from_translation(Vec3::new(width / 2.0 - size.x, 0., 1.)),
        Player::Two,
        PlayerAction::default_player_two(),
    ));

    // spawn box on screen
    commands.spawn((
        Name::new("up paddle"),
        Collider::rectangle(width, 10.0),
        RigidBody::Static,
        StateScoped(GameStates::Playing),
        Transform::from_xyz(0.0, height / 2.0, 1.0),
        Restitution::new(0.6),
        Wall,
    ));
    commands.spawn((
        Name::new("down paddle"),
        Collider::rectangle(width, 10.0),
        RigidBody::Static,
        StateScoped(GameStates::Playing),
        Transform::from_xyz(0.0, height / 2.0 - height, 1.0),
        Restitution::new(0.6),
        Wall,
    ));
    next_state.set(GameStates::Playing);
}

fn pause_physics(mut time: ResMut<Time<Physics>>) {
    time.pause();
}

fn resume_physics(mut time: ResMut<Time<Physics>>) {
    time.unpause();
}

fn restore_sprites(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    query: Query<(Entity, &PlayerVisual), Without<Sprite>>,
) {
    for (entity, visual) in query.iter() {
        let texture_handle = match visual.texture_kind {
            TextureKind::Player => textures.bevy.clone(),
            TextureKind::Enemy => {
                commands.entity(entity).insert((Sprite {
                    image: textures.bevy.clone(),
                    color: visual.color,
                    custom_size: Some(visual.size),
                    ..default()
                },));
                return;
            }
        };
        println!("restore sprites {entity:?}");
        commands.entity(entity).insert((
            Sprite {
                image: texture_handle,
                custom_size: Some(visual.size),
                ..Default::default()
            },
            Restitution::new(0.6),
            LockedAxes::ROTATION_LOCKED,
            MovementDampingFactor(0.9),
            MaxSlopeAngle(PI * 0.45), // TODO: default value
            RigidBody::Kinematic,
            CharacterController,
            MaxLinearSpeed(700.0),
            Collider::rectangle(visual.size.x, visual.size.y),
            StateScoped(GameStates::Playing),
        ));
    }
}

fn apply_movement_damping(
    mut query: Query<(&MovementDampingFactor, &mut LinearVelocity), With<Player>>,
) {
    for (damping_factor, mut linear_velocity) in &mut query {
        linear_velocity.y *= damping_factor.0;
    }
}

fn move_player(
    time: Res<Time>,
    mut move_event: EventReader<MoveEvent>,
    mut controllers: Query<(&mut LinearVelocity, &MaxLinearSpeed), With<Player>>,
) {
    for event in move_event.read() {
        if let Ok((mut linear_velocity, max_speed)) = controllers.get_mut(event.entity) {
            if event.move_intent == Vec2::ZERO {
                linear_velocity.y = 0.0;
                return;
            }
            let delta_time = time.delta_secs_f64().adjust_precision();
            let acceleration = 4000.0;
            let desired = event.move_intent.y * max_speed.0;
            let delta = desired - linear_velocity.y;
            linear_velocity.y += delta * acceleration * delta_time;
            linear_velocity.y = linear_velocity.y.clamp(-max_speed.0, max_speed.0);
        }
    }
}

/// Kinematic bodies do not get pushed by collisions by default,
/// so it needs to be done manually.
///
/// This system handles collision response for kinematic character controllers
/// by pushing them along their contact normals by the current penetration depth,
/// and applying velocity corrections in order to snap to slopes, slide along walls,
/// and predict collisions using speculative contacts.
#[allow(clippy::type_complexity)]
fn kinematic_controller_collisions(
    collisions: Collisions,
    bodies: Query<&RigidBody>,
    collider_rbs: Query<&ColliderOf, Without<Sensor>>,
    mut character_controllers: Query<
        (&mut Position, &mut LinearVelocity, Option<&MaxSlopeAngle>),
        (With<RigidBody>, With<CharacterController>),
    >,
    time: Res<Time>,
) {
    // Iterate through collisions and move the kinematic body to resolve penetration
    for contacts in collisions.iter() {
        // Get the rigid body entities of the colliders (colliders could be children)
        let Ok([&ColliderOf { body: rb1 }, &ColliderOf { body: rb2 }]) =
            collider_rbs.get_many([contacts.collider1, contacts.collider2])
        else {
            continue;
        };

        // Get the body of the character controller and whether it is the first
        // or second entity in the collision.
        let is_first: bool;

        let character_rb: RigidBody;
        let is_other_dynamic: bool;

        let (mut position, mut linear_velocity, max_slope_angle) =
            if let Ok(character) = character_controllers.get_mut(rb1) {
                is_first = true;
                character_rb = *bodies.get(rb1).unwrap();
                is_other_dynamic = bodies.get(rb2).is_ok_and(|rb| rb.is_dynamic());
                character
            } else if let Ok(character) = character_controllers.get_mut(rb2) {
                is_first = false;
                character_rb = *bodies.get(rb2).unwrap();
                is_other_dynamic = bodies.get(rb1).is_ok_and(|rb| rb.is_dynamic());
                character
            } else {
                continue;
            };

        // This system only handles collision response for kinematic character controllers.
        if !character_rb.is_kinematic() {
            continue;
        }

        // Iterate through contact manifolds and their contacts.
        // Each contact in a single manifold shares the same contact normal.
        for manifold in contacts.manifolds.iter() {
            let normal = if is_first {
                -manifold.normal
            } else {
                manifold.normal
            };

            let mut deepest_penetration: Scalar = Scalar::MIN;

            // Solve each penetrating contact in the manifold.
            for contact in manifold.points.iter() {
                if contact.penetration > 0.0 {
                    position.0 += normal * contact.penetration;
                }
                deepest_penetration = deepest_penetration.max(contact.penetration);
            }

            // For now, this system only handles velocity corrections for collisions against static geometry.
            if is_other_dynamic {
                continue;
            }

            // Determine if the slope is climbable or if it's too steep to walk on.
            let slope_angle = normal.angle_to(Vector::Y);
            let climbable = max_slope_angle.is_some_and(|angle| slope_angle.abs() <= angle.0);

            if deepest_penetration > 0.0 {
                // If the slope is climbable, snap the velocity so that the character
                // up and down the surface smoothly.
                if climbable {
                    // Points either left or right depending on which side the normal is leaning on.
                    // (This could be simplified for 2D, but this approach is dimension-agnostic)
                    let normal_direction_x =
                        normal.reject_from_normalized(Vector::Y).normalize_or_zero();

                    // The movement speed along the direction above.
                    let linear_velocity_x = linear_velocity.dot(normal_direction_x);

                    // Snap the Y speed based on the speed at which the character is moving
                    // up or down the slope, and how steep the slope is.
                    //
                    // A 2D visualization of the slope, the contact normal, and the velocity components:
                    //
                    //             ╱
                    //     normal ╱
                    // *         ╱
                    // │   *    ╱   velocity_x
                    // │       * - - - - - -
                    // │           *       | velocity_y
                    // │               *   |
                    // *───────────────────*

                    let max_y_speed = -linear_velocity_x * slope_angle.tan();
                    linear_velocity.y = linear_velocity.y.max(max_y_speed);
                } else {
                    // The character is intersecting an unclimbable object, like a wall.
                    // We want the character to slide along the surface, similarly to
                    // a collide-and-slide algorithm.

                    // Don't apply an impulse if the character is moving away from the surface.
                    if linear_velocity.dot(normal) > 0.0 {
                        continue;
                    }

                    // Slide along the surface, rejecting the velocity along the contact normal.
                    let impulse = linear_velocity.reject_from_normalized(normal);
                    linear_velocity.0 = impulse;
                }
            } else {
                // The character is not yet intersecting the other object,
                // but the narrow phase detected a speculative collision.
                //
                // We need to push back the part of the velocity
                // that would cause penetration within the next frame.

                let normal_speed = linear_velocity.dot(normal);

                // Don't apply an impulse if the character is moving away from the surface.
                if normal_speed > 0.0 {
                    continue;
                }

                // Compute the impulse to apply.
                let impulse_magnitude =
                    normal_speed - (deepest_penetration / time.delta_secs_f64().adjust_precision());
                let mut impulse = impulse_magnitude * normal;

                // Apply the impulse differently depending on the slope angle.
                if climbable {
                    // Avoid sliding down slopes.
                    linear_velocity.y -= impulse.y.min(0.0);
                } else {
                    // Avoid climbing up walls.
                    impulse.y = impulse.y.max(0.0);
                    linear_velocity.0 -= impulse;
                }
            }
        }
    }
}
