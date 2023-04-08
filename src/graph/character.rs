use std::collections::{HashSet, HashMap};

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
            automata.push(Automaton::create_automaton(automaton, &characterjson.dialogues));
        }

        return Character {
            name: characterjson.name,
            dialogues: characterjson.dialogues,
            automata: automata,
        };
    }

    pub fn get_text(&self) -> Vec<String> {
        let mut dialogueSet = HashSet::new();
        for automaton in &self.automata {
            dialogueSet.insert(automaton.get_state().dialogue.clone());
        }

        let mut dialogueVec = Vec::new();
        for dialogue in dialogueSet {
            dialogueVec.extend(self.dialogues[&dialogue].clone());
        }

        return dialogueVec;
    }


}