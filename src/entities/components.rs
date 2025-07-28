use crate::prelude::*;

#[derive(Component, Reflect, Default, Copy, Clone)]
#[reflect(Component)]
#[require(Save)]
pub struct Player;

#[derive(Component, Reflect, Default, Copy, Clone)]
pub struct MovementDampingFactor(pub Scalar);

#[derive(Component, Reflect, Default, Copy, Clone)]
pub struct CharacterController;

#[derive(Component, Reflect, Default, Copy, Clone)]
pub struct MaxSlopeAngle(pub Scalar);

#[derive(Component, Reflect, Default, Copy, Clone)]
#[reflect(Component)]
pub struct PlayerVisual {
    pub texture_kind: TextureKind,
    pub color: Color,
    pub size: Vec2,
}

#[derive(Reflect, Default, Clone, Copy)]
pub enum TextureKind {
    #[default]
    Player,
    Enemy,
}

#[derive(Component, Reflect, Default, Copy, Clone)]
pub struct PlayerInput;

#[derive(Component, Reflect, Default, Copy, Clone)]
pub struct UiInput;

#[derive(Component, Reflect, Default, Copy, Clone)]
pub struct GlobalInput;
