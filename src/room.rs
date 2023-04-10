use bevy::prelude::*;

#[derive(Component, Default)]
pub struct RoomCharacterStorage {
    pub vec: [Option<Entity>; 3],
    pub size: usize,
}
impl RoomCharacterStorage {
    pub fn last_character(&self) -> Option<usize> {
        if self.size > 0 {
            return Some(self.size - 1);
        } else {
            return None;
        }
    }
}

#[derive(Bundle, Default)]
pub struct Room {
    background: SpriteBundle,
    characters: RoomCharacterStorage,
}
impl Room {
    pub fn add_character(&mut self, entity: Entity) {
        assert!(self.characters.size < 3, "too many entities in single room");
        self.characters.vec[self.characters.size] = Some(entity);
        self.characters.size += 1;
    }

    pub fn remove_character(&mut self, index: usize) {
        assert!(index <= 2, "index must be <= 2");
        self.characters.size = self.characters.size.checked_sub(1).unwrap_or(0);
        self.characters.vec[index] = self.characters.vec[self.characters.size];
        self.characters.vec[self.characters.size].take();
    }
}
