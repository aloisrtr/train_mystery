use std::collections::HashMap;
use std::io;
use std::path::Path;

use bevy::prelude::*;
use bevy::ui::Overflow::Visible;
use serde_derive::Deserialize;

/// GENERAL CHARACTER BUNDLE
#[derive(Component, Deref, DerefMut, Debug)]
pub struct Name(String);

#[derive(Deserialize, Debug)]
pub struct CharacterJSON {
    pub name: String,
    pub sprite: String,
    pub behavior: BehaviorJSON,
}

#[derive(Bundle)]
pub struct CharacterBundle {
    pub name: Name,
    pub behavior: BehaviorAutomaton,
    pub sprite: SpriteBundle,
}

impl CharacterBundle {
    pub fn from_json<P: AsRef<Path> + std::fmt::Debug>(
        file: P,
        asset_server: &Res<AssetServer>,
    ) -> io::Result<CharacterBundle> {
        // Deserialize the character file
        let character_json =
            serde_json::from_str::<CharacterJSON>(&std::fs::read_to_string(file)?)?;

        // every character has a bit different position
        // allumeuse    => -400. -300. 0. 
        // averageman   => 0. -300. 0.
        // averagewomen => 400. -300. 0.

        // barista      => -400. 0. 0.
        // conducteur   => 0. 0. 0.
        // enfant       => 400. 0. 0.

        // pote         => -400. 300. 0.
        // racaille     => 0. 300. 0.
        // vieux1       => 400. 300. 0. 

        let mut translation = Vec3::new(0.0, 0.0, 0.0);
        println!("character name: {}", character_json.name);
        if character_json.name == "Enfants" {
            translation = Vec3::new(-400.0, -180.0, 0.0);
        } else if character_json.name == "Average Man" {
            translation = Vec3::new(-300.0, -180.0, 0.0);
        } else if character_json.name == "Pote" {
            translation = Vec3::new(-200.0, -180.0, 0.0);
        } else if character_json.name == "Mr Dawson" {
            translation = Vec3::new(-100.0, -180.0, 0.0);
        } else if character_json.name == "Conducteur" {
            translation = Vec3::new(0.0, -180.0, 0.0);
        } else if character_json.name == "Average Woman" {
            translation = Vec3::new(100.0, -180.0, 0.0);
        } else if character_json.name == "Controlleur" {
            translation = Vec3::new(200.0, -180.0, 0.0);
        } else if character_json.name == "Racaille" {
            translation = Vec3::new(300.0, -180.0, 0.0);
        } else if character_json.name == "Barista" {
            translation = Vec3::new(400.0, -180.0, 0.0);
        } else if character_json.name == "Allumeuse" {
            translation = Vec3::new(500.0, -180.0, 0.0);
        } 
        translation += Vec3::new(0.0, 0.0, 10.0);

        Ok(CharacterBundle {
            name: Name(character_json.name.clone()),
            behavior: BehaviorAutomaton::from(character_json.behavior),
            sprite: SpriteBundle {
                texture: asset_server.load(character_json.sprite),
                visibility: Visibility::Hidden,
                transform: Transform::from_scale(Vec3::new(0.35, 0.35, 0.0))
                    .with_translation(translation),
                ..default()
            },
        })
    }
}

/// BEHAVIOR AUTOMATON
#[derive(Deserialize, Debug, Clone)]
pub struct BehaviorJSON {
    pub init_state: String,
    pub states: HashMap<String, StateJSON>,
}

#[derive(Component, Debug)]
pub struct BehaviorAutomaton {
    pub init_state: String,
    pub current_state: String,
    pub states: HashMap<String, State>,
}
impl From<BehaviorJSON> for BehaviorAutomaton {
    fn from(json: BehaviorJSON) -> Self {
        BehaviorAutomaton {
            init_state: json.init_state.clone(),
            current_state: json.init_state,
            states: json
                .states
                .into_iter()
                .map(|(k, v)| (k, State::from(v)))
                .collect(),
        }
    }
}
impl BehaviorAutomaton {
    pub fn reset_automaton(&mut self) {
        self.current_state = self.init_state.clone();
    }

    pub fn current_state_name(&self) -> &str {
        &self.current_state
    }

    pub fn current_state(&self) -> &State {
        &self.states[&self.current_state]
    }

    pub fn change_state(&mut self, event: &str) {
        if let Some(state) = self.current_state().edges.get(event) {
            self.current_state = state.to_owned();
        }
    }

    pub fn fetch_dialogue(&self) -> Vec<String> {
        self.current_state().dialogue.clone()
    }

    pub fn fetch_location(&self) -> usize {
        self.current_state().location
    }
}

/// STATE
#[derive(Deserialize, Debug, Clone)]
pub struct StateJSON {
    pub dialogue: Vec<String>,
    pub location: usize,
    pub edges: HashMap<String, String>,
}

#[derive(Component, Debug)]
pub struct State {
    pub dialogue: Vec<String>,
    pub location: usize,
    pub edges: HashMap<String, String>,
}
impl From<StateJSON> for State {
    fn from(json: StateJSON) -> Self {
        State {
            dialogue: json.dialogue,
            location: json.location,
            edges: json.edges,
        }
    }
}
