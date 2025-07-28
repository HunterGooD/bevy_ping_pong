use crate::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.157, 0.157, 0.157)))
            .add_systems(OnExit(GameStates::Loading), to_main_menu) // maybe there is another solution for this
            .add_loading_state(
                LoadingState::new(GameStates::Loading)
                    .continue_to_state(GameStates::Menu)
                    .load_collection::<AudioAssets>()
                    .load_collection::<TextureAssets>(),
            );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource, Reflect)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource, Reflect)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,

    #[asset(path = "textures/github.png")]
    pub git_hub: Handle<Image>,
}

fn to_main_menu(mut next_menu_state: ResMut<NextState<MenuStates>>) {
    next_menu_state.set(MenuStates::MainMenu)
}
