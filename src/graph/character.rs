use std::collections::HashMap;

use crate::graph::automaton::Automaton;

use crate::graph::graphparser::characterjson::{CharacterJSON, AutomatonJSON};

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub dialogues: HashMap<String, Vec<String>>,
    pub automata: Vec<AutomatonJSON>,
}

impl Character {
    pub fn create_character(filedest: &String) -> Character {
        let characterjson = CharacterJSON::create_character_json(filedest);

        return Character {
            name: characterjson.name,
            dialogues: characterjson.dialogues,
            automata: characterjson.automata,
        };
    }
}