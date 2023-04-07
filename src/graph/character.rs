use std::collections::HashMap;

use crate::graph::automaton::Automaton;

use crate::graph::graphparser::characterjson::CharacterJSON;

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub dialogues: HashMap<String, Vec<String>>,
    pub automata: Vec<Automaton>,
}

impl Character {
    pub fn create_character(filedest: &String) -> Character {
        let characterjson = CharacterJSON::create_character_json(filedest);
        
        let mut automata = Vec::new();
        for automaton in characterjson.automata.into_iter() {
            automata.push(Automaton::create_automaton(automaton));
        }

        return Character {
            name: characterjson.name,
            dialogues: characterjson.dialogues,
            automata: automata,
        };
    }
}