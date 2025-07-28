use crate::prelude::*;

#[derive(Event, Default, Clone, Copy, Debug)]
pub struct SaveSettingEvent;

#[derive(Event, Default, Clone, Copy, Debug)]
pub struct SaveGameEvent;

#[derive(Event, Default, Clone, Copy, Debug)]
pub struct LoadGameEvent;
