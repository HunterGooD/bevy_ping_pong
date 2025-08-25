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

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct Scores {
    pub right_score: i32,
    pub left_score: i32,
}

impl Scores {
    pub fn clean_scores(&mut self) {
        self.left_score = 0;
        self.right_score = 0;
    }
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct CountdownTimer {
    pub timer: Timer,
}

impl CountdownTimer {
    pub fn new(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Once),
        }
    }

    pub fn reset(&mut self) {
        self.timer.reset();
    }
}
