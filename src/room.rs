use bevy::prelude::*;

#[derive(Component, Default)]
pub struct RoomCharacterStorage(pub [Option<Entity>; 3]);

#[derive(Bundle, Default)]
pub struct Room {
    background: SpriteBundle,
    characters: RoomCharacterStorage,
}