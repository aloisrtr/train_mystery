mod character;
mod game_state;
pub mod room;
mod train;

use crate::character::{BehaviorAutomaton, CharacterBundle, Name};
use crate::game_state::{GameState, GameplayState, MenuState};
use crate::room::{Room, RoomCharacterStorage};
use crate::train::{Train, ROOMS_COUNT};
use bevy::window::WindowResolution;
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    // window::WindowResolution,
    render::view::visibility::RenderLayers,
    text::{BreakLineOn, Text2dBounds},
};
use std::cmp::min;
use std::fs;

// 1080p Does not work
// const WIDTH: f32 = 1920.;
// const HEIGHT: f32 = 1080.;

// 720p
const WIDTH: f32 = 1280.;
const HEIGHT: f32 = 720.;

const FPS: f32 = 60.;

#[derive(Component, Deref, DerefMut, Default)]
pub struct CameraPosition(pub Vec3);

#[derive(Resource, Default)]
pub struct Event(Option<String>);

#[derive(Resource, Default)]
pub struct Dialogue {
    pub character_name: String,
    pub text: Vec<String>,
    pub lines_read: usize,
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WIDTH, HEIGHT),
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
        .add_system(animate_background)
        .add_system(interpolate_transforms)
        .add_system(display_dialogue)
        .run()
}

/// Sets up out assets, cameras, etc
fn setup(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    window: Query<&Window>,
    audio: Res<Audio>,
) {
    spawn_text_box(&mut commands, &window.get_single().unwrap(), &asset_server);
    // Spawn rooms, characters and train
    let mut rooms = [(); 7].map(|_| Room::default());

    let mut character_rooms = vec![];
    for file in fs::read_dir("assets/automata").unwrap() {
        let chara = CharacterBundle::from_json(file.unwrap().path(), &asset_server).unwrap();
        let room = chara.behavior.fetch_location();
        character_rooms.push((commands.spawn((chara, RenderLayers::layer(2))).id(), room));
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
        Camera2dBundle {
            camera: Camera {
                order: 0,
                ..default()
            },
            ..default()
        },
        BackgroundCamera,
        CameraPosition(Camera2dBundle::default().transform.translation),
        RenderLayers::layer(3),
    ));
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
                ..default()
            },
            camera: Camera {
                order: 1,
                is_active: false,
                ..default()
            },
            ..default()
        },
        OutsideCamera,
        CameraPosition(Camera2dBundle::default().transform.translation),
        RenderLayers::layer(0),
    ));
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
                ..default()
            },
            camera: Camera {
                order: 1,
                is_active: true,
                ..default()
            },
            ..default()
        },
        InsideCamera,
        RenderLayers::layer(2),
    ));
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
                ..default()
            },
            camera: Camera {
                order: 2,
                ..default()
            },
            ..default()
        },
        FixedCamera,
        RenderLayers::layer(1),
    ));

    // Load up assets

    // BACKGROUNDS
    let background_texture = asset_server.load("background/background2.png");
    commands.spawn((
        SpriteBundle {
            texture: background_texture,
            sprite: Sprite {
                rect: Some(Rect::new(
                    0f32,
                    0f32,
                    (ROOMS_COUNT as f32) * WIDTH,
                    // HEIGHT,
                    1714.0,
                )),
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(0.16, 0.16, 0.0))
            .with_translation(Vec3::new(0.0, HEIGHT / 3.2, 0.0)),
            ..default()
        },
        BackgroundAnimation {
            timer: Timer::from_seconds(1. / FPS, TimerMode::Repeating),
            speed: 1f32,
            size: 16382.0,
        },
        RenderLayers::layer(3),
    ));

    let desert_texture = asset_server.load("background/desert2.png");
    commands.spawn((
        SpriteBundle {
            texture: desert_texture,
            sprite: Sprite {
                rect: Some(Rect::new(
                    0f32,
                    0f32,
                    (ROOMS_COUNT as f32) * WIDTH,
                    1714.0,
                )),
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(0.3,0.3, 0.))
                .with_translation(Vec3::new(0.0, -HEIGHT / 7.2, 0.1)),
            ..default()
        },
        BackgroundAnimation {
            timer: Timer::from_seconds(1. / FPS, TimerMode::Repeating),
            speed: 60.0,
            size: 16384.0,
        },
        RenderLayers::layer(3),
    ));

    let grass_texture = asset_server.load("background/grass2.png");
    commands.spawn((
        SpriteBundle {
            texture: grass_texture,
            sprite: Sprite {
                rect: Some(Rect::new(
                    0f32,
                    0f32,
                    (ROOMS_COUNT as f32) * WIDTH,
                    1714.0,
                )),
                ..default()
            },
        transform: Transform::from_scale(Vec3::new(0.2, 0.2, 0.0))
                .with_translation(Vec3::new(0.0, -HEIGHT / 7.0, 1.2)),
            ..default()
        },
        BackgroundAnimation {
            timer: Timer::from_seconds(1. / FPS, TimerMode::Repeating),
            speed: 90.0,
            size: 16384.0,
        },
        RenderLayers::layer(3),
    ));

    let rails_texture = asset_server.load("background/rails2.png");
    commands.spawn((
        SpriteBundle {
            texture: rails_texture,
            sprite: Sprite {
                rect: Some(Rect::new(0f32, 0f32, (ROOMS_COUNT as f32) * WIDTH, 1714.0)),
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(1.0, 1080.0 / 1714.0 * 0.5, 1.0))
                .with_translation(Vec3::new(0.0, -1080.0 * 0.05, 0.9)),
            ..default()
        },
        BackgroundAnimation {
            timer: Timer::from_seconds(1. / FPS, TimerMode::Repeating),
            speed: 40.0,
            size: 16250.0,
        },
    ));

    // TRAINS
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
            transform: Transform::from_translation(Vec3::new(915f32 * 6f32, 0f32, 1f32)),
            ..default()
        },
        Animation {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            frames: 3,
        },
    ));

    let wagon_background_image = asset_server.load("wagon/wagon_int.png");
    commands.spawn((
        SpriteBundle {
            texture: wagon_background_image,
            ..default()
        },
        RenderLayers::layer(2),
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

    let ui_font_handle = asset_server.load("fonts/DejaVuSerif.ttf");
    commands.insert_resource(UiFont(ui_font_handle));
}

/// Deals with player input
fn handle_input(
    keys: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut event: ResMut<Event>,
    mut dialogue: ResMut<Dialogue>,
    train: Query<&Train>,
    rooms: Query<&RoomCharacterStorage>,
    behavior_automaton: Query<&BehaviorAutomaton>,
    name_query: Query<&Name>,
    mut outcamerapos: Query<&mut CameraPosition, With<OutsideCamera>>,
    mut outcamera: Query<&mut Camera, With<OutsideCamera>>,
    mut incamera: Query<&mut Camera, (With<InsideCamera>, Without<OutsideCamera>)>,
    mut visibility: Query<&mut Visibility, With<BehaviorAutomaton>>,
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
            dialogue.lines_read += 1;
            if dialogue.lines_read == dialogue.text.len() {
                dialogue.text = vec![];
                dialogue.lines_read = 0;
            }
        } else {
            // Interacts with the closest intractable entity, advances the dialogue or selects a dialogue option
            match game_state.as_mut() {
                GameState {
                    gameplay_state: GameplayState::Hub { selected_room },
                    opened_menu: MenuState::None,
                } => {
                    let room_id = train.rooms[*selected_room];

                    game_state.gameplay_state = GameplayState::Room {
                        room_id,
                        selected_character: 0,
                    };
                    outcamera.single_mut().is_active = false;
                    incamera.single_mut().is_active = true;
                }
                GameState {
                    gameplay_state:
                        GameplayState::Room {
                            selected_character,
                            room_id,
                        },
                    opened_menu: MenuState::None,
                } => {
                    // Interact with the selected character
                    let character_id_wrapper = rooms.get(*room_id).unwrap().0[*selected_character];
                    if let Some(character_id) = character_id_wrapper {
                        let behavior = behavior_automaton.get(character_id).unwrap();
                        let name = name_query.get(character_id).unwrap();
                        event.0 = Some(behavior.current_state_name().to_string());
                        dialogue.character_name = name.to_string();
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
                // Room entities are made visible
                let room = rooms.get(*room_id).unwrap();
                for entity in room.0.iter().filter_map(|x| *x) {
                    if let Ok(mut vis) = visibility.get_mut(entity) {
                        *vis = Visibility::Hidden
                    }
                }

                let selected_room = train
                    .rooms
                    .iter()
                    .enumerate()
                    .find_map(|(i, r)| if r == room_id { Some(i) } else { None })
                    .unwrap();
                game_state.gameplay_state = GameplayState::Hub { selected_room };
                outcamera.single_mut().is_active = true;
                incamera.single_mut().is_active = false;
            }
            _ => (),
        }
    }

    // Deal with camera
    let mut camera_position = outcamerapos.get_single_mut().unwrap();
    match game_state.as_mut() {
        GameState {
            gameplay_state: GameplayState::Hub { selected_room },
            ..
        } => {
            camera_position.x = 925f32 * *selected_room as f32;
        }
        GameState {
            gameplay_state: GameplayState::Room { room_id, .. },
            ..
        } => {
            // Room entities are made visible
            let room = rooms.get(*room_id).unwrap();
            for entity in room.0.iter().filter_map(|x| *x) {
                if let Ok(mut vis) = visibility.get_mut(entity) {
                    *vis = Visibility::Visible
                }
            }
        }
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

fn animate_background(time: Res<Time>, mut query: Query<(&mut BackgroundAnimation, &mut Sprite)>) {
    for (mut animation, mut sprite) in &mut query {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            // On translate le rectangle de la vue
            let mut rect = sprite.rect.unwrap();
            translate_rectangle(&mut rect, animation.speed, 0.0);
            sprite.rect = Some(rect);
            // Si on est trop loin, on wrap
            if rect.min.x > animation.size / 2.0 {
                sprite.rect = Some(Rect::new(
                    0.0,
                    0.0,
                    (ROOMS_COUNT as f32) * WIDTH,
                    rect.max.y,
                ))
            }
        }
    }
}

fn move_characters() {}

fn display_dialogue(
    dialogue: Res<Dialogue>,
    mut text_box_visibility: Query<&mut Visibility, With<TextBoxMarker>>,
    mut text_box: Query<&mut Text>,
) {
    if dialogue.text.is_empty() {
        *text_box_visibility.get_single_mut().unwrap() = Visibility::Hidden;
        return;
    }

    *text_box_visibility.get_single_mut().unwrap() = Visibility::Visible;
    for (i, mut text_box) in text_box.iter_mut().enumerate() {
        text_box.sections[0].value = if i == 0 {
            dialogue.text[dialogue.lines_read].clone()
        } else {
            dialogue.character_name.clone()
        };
    }
}

fn translate_rectangle(rect: &mut Rect, translation_x: f32, translation_y: f32) {
    rect.max.x += translation_x;
    rect.max.y += translation_y;
    rect.min.x += translation_x;
    rect.min.y += translation_y;
}
fn spawn_text_box(commands: &mut Commands, window: &Window, asset_server: &Res<AssetServer>) {
    let window_width = window.resolution.width();
    let window_height = window.resolution.height();

    let style = TextStyle {
        font: asset_server.load("fonts/DejaVuSerif.ttf"),
        font_size: 35f32,
        color: Color::WHITE,
    };

    let box_size = Vec2::new(window_width * 0.9, window_height * 0.3);
    let box_pos = Vec2::new(0.0, (window_height as f32) * -0.3);
    commands
        .spawn((
            TextBoxMarker,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.1, 0.1, 0.1, 0.8),
                    custom_size: Some(box_size),
                    ..default()
                },
                transform: Transform::from_translation(box_pos.extend(0.0)),
                visibility: Visibility::Hidden,
                ..default()
            },
            RenderLayers::layer(1),
        ))
        .with_children(|builder| {
            builder.spawn((
                Text2dBundle {
                    text: Text {
                        sections: vec![TextSection::new("", style.clone())],
                        alignment: TextAlignment::Left,
                        linebreak_behaviour: BreakLineOn::WordBoundary,
                    },
                    text_2d_bounds: Text2dBounds { size: box_size },
                    transform: Transform::from_translation(Vec3::Z),
                    visibility: Visibility::Inherited,
                    ..default()
                },
                RenderLayers::layer(1),
            ));
            builder.spawn((
                Text2dBundle {
                    text: Text {
                        sections: vec![TextSection::new("", style.clone())],
                        alignment: TextAlignment::Center,
                        linebreak_behaviour: BreakLineOn::WordBoundary,
                    },
                    text_2d_bounds: Text2dBounds { size: box_size },
                    transform: Transform::from_translation(
                        Vec3::Z
                            + Vec3::new(
                                -box_size[0] / 2f32 + 100f32,
                                box_size[1] / 2f32 - style.font_size,
                                0f32,
                            ),
                    ),
                    visibility: Visibility::Inherited,
                    ..default()
                },
                RenderLayers::layer(1),
            ));
        });
}

#[derive(Component)]
pub struct Animation {
    pub timer: Timer,
    pub frames: usize,
}

#[derive(Component)]
struct OutsideCamera;

#[derive(Component)]
struct FixedCamera;

#[derive(Component)]
struct InsideCamera;

#[derive(Component)]
struct BackgroundCamera;

#[derive(Component)]
struct TextBoxMarker;

#[derive(Component)]
pub struct BackgroundAnimation {
    pub timer: Timer,
    pub speed: f32,
    pub size: f32, // Longueur de l'image (pour savoir quand wrap)
}

#[derive(Resource)]
struct UiFont(Handle<Font>);
