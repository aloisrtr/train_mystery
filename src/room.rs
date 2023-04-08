use bevy::prelude::*;

#[derive(Component, Default)]
pub struct RoomCharacterStorage(pub [Option<Entity>; 3]);

#[derive(Bundle, Default)]
pub struct Room {
    background: SpriteBundle,
    characters: RoomCharacterStorage,
}
impl Room {
    pub fn add_character(&mut self, entity: Entity) {
        for c in &mut self.characters.0 {
            if c.is_none() {
                *c = Some(entity);
                return;
            }
        }
        panic!("too many entities in single room")
    }

    pub fn remove_character(&mut self, index: usize) -> Option<Entity> {
        self.characters.0[index].take()
    }
}
