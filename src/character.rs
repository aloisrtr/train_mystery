use bevy::prelude::*;

#[derive(Component)]
pub struct CharacterMarker;

#[derive(Component)]
pub struct Behavior;

#[derive(Bundle)]
pub struct Character {
    _marker: CharacterMarker,

    room_sprite: SpriteBundle,
    behavior: Behavior,
}