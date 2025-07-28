use crate::prelude::*;

#[derive(States, Reflect, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameStates {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // add splash with image and effect fade out
    // Splash,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    // LoadingGame load from file or start new game
    LoadingGame,
    // During this State the actual game logic is executed
    Playing,
}

#[derive(States, Reflect, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum MenuStates {
    #[default]
    Disable,
    MainMenu,
    PauseMenu,
    Setting,
    GameOver,
}

#[derive(States, Reflect, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum SettingsStates {
    #[default]
    Audio,
    Controls,
    Other,
}
