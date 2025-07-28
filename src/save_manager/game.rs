use crate::prelude::*;
#[cfg(target_arch = "wasm32")]
use crate::save_manager::{LocalStorageReader, LocalStorageWriter};
use moonshine_save::save::DefaultSaveFilter;

pub const FILE_GAME_SAVE: &str = "game.ron";
pub struct GameSaveManagerPlugin;

struct GameSaveEvent {
    #[cfg(not(target_arch = "wasm32"))]
    pub path: String,
    #[cfg(target_arch = "wasm32")]
    pub stream: LocalStorageWriter,
}

impl Plugin for GameSaveManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(save_on::<GameSaveEvent>).add_systems(
            Update,
            (
                save_game_event,
                load_game_event,
                // test_game_player,
            ),
        );
    }
}

impl SingleEvent for GameSaveEvent {}

impl SaveEvent for GameSaveEvent {
    type SaveFilter = DefaultSaveFilter;

    fn filter_entity(&self, entity: Entity) -> bool {
        println!("{entity:?}");
        true
    }

    fn before_serialize(&mut self, _: EntityWorldMut) {
        println!("before_serialize");
    }

    fn after_serialize(&mut self, _: EntityWorldMut) {
        println!("after_serialize");
    }

    fn component_filter(&self) -> SceneFilter {
        println!("component_filter all");
        SceneFilter::deny_all()
            .allow::<Player>()
            .allow::<Transform>()
            .allow::<Name>()
            .allow::<PlayerVisual>()
    }

    fn resource_filter(&self) -> SceneFilter {
        SceneFilter::deny_all()
    }

    fn output(self) -> SaveOutput {
        #[cfg(target_arch = "wasm32")]
        {
            info!("output stream created!");
            SaveOutput::stream(self.stream)
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            SaveOutput::file(self.path)
        }
    }
}

fn save_game_event(mut commands: Commands, mut save_event: EventReader<SaveGameEvent>) {
    for _ in save_event.read() {
        #[cfg(target_arch = "wasm32")]
        {
            let writer = LocalStorageWriter {
                key: FILE_GAME_SAVE.to_string(),
                buffer: Vec::new(),
            };
            info!("Start saving");
            commands.trigger_save(GameSaveEvent { stream: writer });
        }

        #[cfg(not(target_arch = "wasm32"))]
        commands.trigger_save(GameSaveEvent {
            path: FILE_GAME_SAVE.to_string(),
        });
    }
}

fn load_game_event(mut commands: Commands, mut game_events: EventReader<LoadGameEvent>) {
    for _ in game_events.read() {
        #[cfg(target_arch = "wasm32")]
        {
            let reader = LocalStorageReader::new(FILE_GAME_SAVE.to_string());
            if reader.data.is_empty() {
                info!("is empty key {FILE_GAME_SAVE}");
                return;
            }
            info!("Start loading game");
            commands.trigger_load(LoadWorld::default_from_stream(reader));
        }

        #[cfg(not(target_arch = "wasm32"))]
        commands.trigger_load(LoadWorld::default_from_file(FILE_GAME_SAVE));
    }
}

// fn test_game_player(
//     mut commands: Commands,
//     mut game_events: EventReader<SaveGameEvent>,
//     query: Query<Entity, With<Save>>,
// ) {
//     for _ in game_events.read() {
//         if let Ok(entity) = query.single() {
//             commands.entity(entity).log_components();
//         }
//         commands.trigger_save(GameSaveEvent::new(FILE_GAME_SAVE));
//     }
// }
