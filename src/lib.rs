mod game_state;
mod train;
pub mod room;
pub mod character;
mod graph;

use std::cmp::{max, min};
use bevy::prelude::*;
use crate::game_state::{GameplayState, GameState, MenuState};
use crate::room::{Room, RoomCharacterStorage};
use crate::train::{ROOMS_COUNT, Train};

pub fn run() {
    App::new()
        .insert_resource(GameState {
            gameplay_state: GameplayState::Uninit,
            opened_menu: MenuState::None
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(handle_input)
        .add_system(animate_sprites)
        .run()
}

/// Sets up out assets, cameras, etc
fn setup(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    // Spawn rooms, characters and train
    let rooms = [(); 6].map(|_| commands.spawn(Room::default()).id());
    game_state.gameplay_state = GameplayState::Room { room_id: rooms[0], selected_character: 0 };
    commands.spawn(Train { rooms });

    commands.spawn(Camera2dBundle::default());
    // Load up assets
    let wagon_texture = asset_server.load("wagon/wagon_ext.png");
    for i in 0..ROOMS_COUNT {
        commands.spawn(SpriteBundle {
            texture: wagon_texture.clone(),
            transform: Transform::from_translation(Vec3::new(925f32 * i as f32, 0f32, 1f32)),
            .. default()
        });
    }

    let locomotive_texture = asset_server.load("locomotive/locomotive.png");
    let texture_atlas = texture_atlases.add(TextureAtlas::from_grid(locomotive_texture, Vec2::new(966f32, 626f32), 3, 1, None, None));
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_translation(Vec3::new(915f32 * 6f32, 0f32, 0f32)),
            .. default()
        },
        Animation {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            frames: 3
        }
    ));

    /*
    let background_image = asset_server.load("background.png");
    commands.spawn(SpriteBundle {
        texture: background_image,
        .. default()
    });
     */
}

/// Deals with player input
fn handle_input(
    keys: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    train: Query<&Train>,
    rooms: Query<&RoomCharacterStorage>,
    mut camera: Query<&mut Transform, With<Camera2d>>
) {
    let train = train.get_single().unwrap();
    if keys.just_pressed(KeyCode::Left) {
        match game_state.as_mut() {
            GameState {
                opened_menu: MenuState::None,
                gameplay_state: GameplayState::Hub { selected_room}
            } => {
                *selected_room = selected_room.checked_sub(1).unwrap_or(0);
            },
            GameState {
                opened_menu: MenuState::None,
                gameplay_state: GameplayState::Room { selected_character, room_id }
            } => {
                let characters = rooms.get(*room_id).unwrap();
                let characters_count = characters.0.iter().fold(0, |acc, c| if c.is_some() { acc + 1 } else { acc });
                if characters_count != 0 {
                    *selected_character = selected_character.checked_sub(1).unwrap_or(0)
                }
            },
            _ => ()
        }
    }
    if keys.just_pressed(KeyCode::Right) {
        match game_state.as_mut() {
            GameState {
                opened_menu: MenuState::None,
                gameplay_state: GameplayState::Hub { selected_room}
            } => {
                *selected_room = min(*selected_room + 1, ROOMS_COUNT - 1);
            },
            GameState {
                opened_menu: MenuState::None,
                gameplay_state: GameplayState::Room { selected_character, room_id }
            } => {
                let characters = rooms.get(*room_id).unwrap();
                let characters_count = characters.0.iter().fold(0, |acc, c| if c.is_some() { acc + 1} else { acc });
                if characters_count != 0 {
                    *selected_character = (*selected_character + 1) % characters_count;
                }
            },
            _ => ()
        }
    }

    if keys.just_pressed(KeyCode::Space) {
        // Interacts with the closest intractable entity, advances the dialogue or selects a dialogue option
        match game_state.as_mut() {
            GameState {
                gameplay_state: GameplayState::Hub { selected_room },
                opened_menu: MenuState::None
            } => {
                game_state.gameplay_state = GameplayState::Room { room_id: train.rooms[*selected_room], selected_character: 0 }
            },
            GameState {
                gameplay_state: GameplayState::Room { .. },
                opened_menu: MenuState::None
            } => {
                // Interact with the selected character
            }
            _ => ()
        }
    }
    if keys.just_pressed(KeyCode::Tab) {
        // Opens or closes journal no matter the game state
        match game_state.opened_menu {
            MenuState::None => game_state.opened_menu = MenuState::Journal,
            MenuState::Journal => game_state.opened_menu = MenuState::None,
            _ => ()
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        // Pauses the game
        match game_state.opened_menu {
            MenuState::None => game_state.opened_menu = MenuState::Pause,
            MenuState::Pause => game_state.opened_menu = MenuState::None,
            _ => ()
        }
    }
    if keys.just_pressed(KeyCode::Back) {
        // Pauses the game
        match game_state.as_mut() {
            GameState {
                gameplay_state: GameplayState::Room { room_id, ..},
                opened_menu: MenuState::None
            } => {
                let selected_room = train.rooms.iter().enumerate().find_map(|(i, r)| if r == room_id { Some(i) } else { None }).unwrap();
                game_state.gameplay_state = GameplayState::Hub { selected_room }
            },
            _ => ()
        }
    }

    // Deal with camera
    match game_state.as_mut() {
        GameState {
            gameplay_state: GameplayState::Hub { selected_room},
            ..
        } => {
            let mut camera_transform = camera.get_single_mut().unwrap();
            camera_transform.translation.x = 925f32 * *selected_room as f32
        },
        GameState {
            gameplay_state: GameplayState::Room { .. },
            ..
        } => {

        },
        _ => ()
    }
}

fn animate_sprites(time: Res<Time>, mut query: Query<(&mut Animation, &mut TextureAtlasSprite)>) {
    for (mut animation, mut sprite) in &mut query {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            sprite.index += 1;
            if sprite.index == animation.frames {
                sprite.index = 0;
            }
        }
    }
}


#[derive(Component)]
pub struct Animation {
    pub timer: Timer,
    pub frames: usize,
}