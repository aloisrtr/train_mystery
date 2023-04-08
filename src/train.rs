use bevy::prelude::*;

pub const ROOMS_COUNT: usize = 7;

#[derive(Component)]
pub struct Train {
    pub rooms: [Entity; ROOMS_COUNT],
}
