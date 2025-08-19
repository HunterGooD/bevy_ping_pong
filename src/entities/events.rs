use crate::prelude::*;

#[derive(Event, Default, Clone, Copy, Debug)]
pub struct SaveSettingEvent;

#[derive(Event, Default, Clone, Copy, Debug)]
pub struct SaveGameEvent;

#[derive(Event, Default, Clone, Copy, Debug)]
pub struct LoadGameEvent;

#[derive(Event, Clone, Copy, Debug)]
pub struct MoveEvent {
    pub entity: Entity,
    pub move_intent: Vec2,
}

#[derive(Event, Clone, Copy, Debug)]
pub struct SpeedBoostEvent {
    pub entity: Entity,
    pub speed_boost: f32,
}
