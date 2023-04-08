use bevy::prelude::*;

#[derive(Resource)]
pub struct GameState {
    pub opened_menu: MenuState,
    pub gameplay_state: GameplayState,
}

#[derive(Resource)]
pub enum GameplayState {
    Uninit,
    Hub {
        selected_room: usize,
    },
    Room {
        room_id: Entity,
        selected_character: usize,
    },
}

#[derive(Resource, Default)]
pub enum MenuState {
    #[default]
    None,
    Pause,
    Journal,
}
