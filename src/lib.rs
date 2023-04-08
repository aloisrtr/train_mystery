pub mod character;
mod game_state;
pub mod room;
mod train;

use crate::character::{BehaviorAutomaton, CharacterBundle};
use crate::game_state::{GameState, GameplayState, MenuState};
use crate::room::{Room, RoomCharacterStorage};
use crate::train::{Train, ROOMS_COUNT};
use bevy::prelude::*;
use bevy::window::WindowResolution;
use std::cmp::min;
use std::fs;

#[derive(Component, Deref, DerefMut, Default)]
pub struct CameraPosition(pub Vec3);

#[derive(Resource, Default)]
pub struct Event(Option<String>);

#[derive(Resource, Default)]
pub struct Dialogue {
    pub text: Vec<String>,
    pub lines_read: usize,
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1920., 1080.),
                resizable: false,
                title: "Train Schizophrenia".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(GameState {
            gameplay_state: GameplayState::Uninit,
            opened_menu: MenuState::None,
        })
        .insert_resource(Event::default())
        .insert_resource(Dialogue::default())
        .add_startup_system(setup)
        .add_system(handle_input)
        .add_system(dispatch_event)
        .add_system(move_characters)
        .add_system(animate_sprites)
        .add_system(interpolate_transforms)
        .run()
}

/// Sets up out assets, cameras, etc
fn setup(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    // Spawn rooms, characters and train
    let mut rooms = [(); 7].map(|_| Room::default());

    let mut character_rooms = vec!();
    for file in fs::read_dir("assets/automata").unwrap() {
        let chara = CharacterBundle::from_json(file.unwrap().path(), &asset_server).unwrap();
        let room = chara.behavior.fetch_location();
        character_rooms.push((commands.spawn(chara).id(), room));
    }
    for (id, room) in character_rooms {
        rooms[room].add_character(id)
    }

    let rooms = rooms.map(|r| commands.spawn(r).id());
    game_state.gameplay_state = GameplayState::Room {
        room_id: rooms[0],
        selected_character: 0,
    };
    commands.spawn(Train { rooms });

    commands.spawn((
        Camera2dBundle::default(),
        WagonCamera,
        CameraPosition(Camera2dBundle::default().transform.translation),
    ));
    // Load up assets
    let wagon_texture = asset_server.load("wagon/wagon_ext.png");
    let texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
        wagon_texture,
        Vec2::new(945f32, 626f32),
        3,
        1,
        None,
        None,
    ));

    for i in 0..(ROOMS_COUNT - 1) {
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas.clone(),
                sprite: TextureAtlasSprite::new(i % 3),
                transform: Transform::from_translation(Vec3::new(
                    925f32 * i as f32,
                    0f32,
                    (i % 2) as f32 + 1f32,
                )),
                ..default()
            },
            Animation {
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                frames: 3,
            },
        ));
    }

    let locomotive_texture = asset_server.load("locomotive/locomotive.png");
    let texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
        locomotive_texture,
        Vec2::new(966f32, 626f32),
        3,
        1,
        None,
        None,
    ));
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_translation(Vec3::new(915f32 * 6f32, 0f32, 0f32)),
            ..default()
        },
        Animation {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            frames: 3,
        },
    ));


    let audio_train = asset_server.load("audio/train.ogg");
    audio.play_with_settings(
        audio_train,
        PlaybackSettings {
            repeat: true,
            volume: 0.25,
            speed: 1.0,
        },
    );

    let audio_birds = asset_server.load("audio/birds.ogg");
    audio.play_with_settings(
        audio_birds,
        PlaybackSettings {
            repeat: true,
            volume: 1.5,
            speed: 1.0,
        },
    );

    let audio_wind = asset_server.load("audio/wind.ogg");
    audio.play_with_settings(
        audio_wind,
        PlaybackSettings {
            repeat: true,
            volume: 1.5,
            speed: 1.0,
        },
    );

    let audio_wind = asset_server.load("audio/Night-on-the-Docks-Sax.ogg");
    audio.play_with_settings(audio_wind, PlaybackSettings::LOOP);
}

/// Deals with player input
fn handle_input(
    keys: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut event: ResMut<Event>,
    mut dialogue: ResMut<Dialogue>,
    train: Query<&Train>,
    rooms: Query<&RoomCharacterStorage>,
    mut camera: Query<&mut CameraPosition, With<Camera2d>>,
    behavior_automaton: Query<&BehaviorAutomaton>,
) {
    let train = train.get_single().unwrap();
    if keys.just_pressed(KeyCode::Left) && dialogue.text.is_empty() {
        match game_state.as_mut() {
            GameState {
                opened_menu: MenuState::None,
                gameplay_state: GameplayState::Hub { selected_room },
            } => {
                *selected_room = selected_room.checked_sub(1).unwrap_or(0);
            }
            GameState {
                opened_menu: MenuState::None,
                gameplay_state:
                    GameplayState::Room {
                        selected_character,
                        room_id,
                    },
            } => {
                let characters = rooms.get(*room_id).unwrap();
                let characters_count =
                    characters
                        .0
                        .iter()
                        .fold(0, |acc, c| if c.is_some() { acc + 1 } else { acc });
                if characters_count != 0 {
                    *selected_character = selected_character.checked_sub(1).unwrap_or(0)
                }
            }
            _ => (),
        }
    }
    if keys.just_pressed(KeyCode::Right) && dialogue.text.is_empty() {
        match game_state.as_mut() {
            GameState {
                opened_menu: MenuState::None,
                gameplay_state: GameplayState::Hub { selected_room },
            } => {
                *selected_room = min(*selected_room + 1, ROOMS_COUNT - 1);
            }
            GameState {
                opened_menu: MenuState::None,
                gameplay_state:
                    GameplayState::Room {
                        selected_character,
                        room_id,
                    },
            } => {
                let characters = rooms.get(*room_id).unwrap();
                let characters_count =
                    characters
                        .0
                        .iter()
                        .fold(0, |acc, c| if c.is_some() { acc + 1 } else { acc });
                if characters_count != 0 {
                    *selected_character = (*selected_character + 1) % characters_count;
                }
            }
            _ => (),
        }
    }

    if keys.just_pressed(KeyCode::Space) {
        if !dialogue.text.is_empty() {
            if dialogue.lines_read == dialogue.text.len() {
                dialogue.text = vec!();
                dialogue.lines_read = 0;
            } else {
                dialogue.lines_read += 1;
            }
        } else {
            // Interacts with the closest intractable entity, advances the dialogue or selects a dialogue option
            match game_state.as_mut() {
                GameState {
                    gameplay_state: GameplayState::Hub { selected_room },
                    opened_menu: MenuState::None,
                } => {
                    game_state.gameplay_state = GameplayState::Room {
                        room_id: train.rooms[*selected_room],
                        selected_character: 0,
                    }
                }
                GameState {
                    gameplay_state: GameplayState::Room { selected_character, room_id },
                    opened_menu: MenuState::None,
                } => {
                    // Interact with the selected character
                    let character = rooms.get(*room_id).unwrap().0[*selected_character];
                    if let Some(character) = character {
                        let behavior = behavior_automaton.get(character).unwrap();
                        event.0 = Some(behavior.current_state_name().to_string());
                        dialogue.text = behavior.fetch_dialogue();
                        dialogue.lines_read = 0;
                    }
                }
                _ => (),
            }
        }
    }
    if keys.just_pressed(KeyCode::Tab) && dialogue.text.is_empty() {
        // Opens or closes journal no matter the game state
        match game_state.opened_menu {
            MenuState::None => game_state.opened_menu = MenuState::Journal,
            MenuState::Journal => game_state.opened_menu = MenuState::None,
            _ => (),
        }
    }
    if keys.just_pressed(KeyCode::Escape) && dialogue.text.is_empty() {
        // Pauses the game
        match game_state.opened_menu {
            MenuState::None => game_state.opened_menu = MenuState::Pause,
            MenuState::Pause => game_state.opened_menu = MenuState::None,
            _ => (),
        }
    }
    if keys.just_pressed(KeyCode::Back) && dialogue.text.is_empty() {
        match game_state.as_mut() {
            GameState {
                gameplay_state: GameplayState::Room { room_id, .. },
                opened_menu: MenuState::None,
            } => {
                let selected_room = train
                    .rooms
                    .iter()
                    .enumerate()
                    .find_map(|(i, r)| if r == room_id { Some(i) } else { None })
                    .unwrap();
                game_state.gameplay_state = GameplayState::Hub { selected_room };
            }
            _ => (),
        }
    }

    // Deal with camera
    let mut camera_position = camera.get_single_mut().unwrap();
    match game_state.as_mut() {
        GameState {
            gameplay_state: GameplayState::Hub { selected_room },
            ..
        } => {
            camera_position.x = 925f32 * *selected_room as f32;
        }
        GameState {
            gameplay_state: GameplayState::Room { .. },
            ..
        } => {}
        _ => (),
    }
}

fn interpolate_transforms(mut query: Query<(&CameraPosition, &mut Transform)>) {
    for (position, mut transform) in &mut query {
        transform.translation = transform.translation * 0.95 + position.0 * 0.05
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

fn dispatch_event(mut event: ResMut<Event>, mut behavior_automata: Query<&mut BehaviorAutomaton>) {
    if let Some(event) = event.0.take() {
        for mut behavior_automaton in &mut behavior_automata {
            behavior_automaton.change_state(&event);
        }
    }
}

fn move_characters() {}

#[derive(Component)]
pub struct Animation {
    pub timer: Timer,
    pub frames: usize,
}

#[derive(Component)]
struct WagonCamera;
