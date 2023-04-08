use std::collections::{HashSet, HashMap};

use crate::graph::automaton::Automaton;

use crate::graph::graphparser::characterjson::CharacterJSON;

#[derive(Debug)]
pub struct Dialogue {
    pub text: Vec<String>,
    pub new: bool,
}


#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub diary: Vec<String>,
    pub dialogues: HashMap<String, Dialogue>,
    pub automata: Vec<Automaton>,
}

impl Character {
    pub fn create_character(filedest: &String) -> Character {
        let characterjson = CharacterJSON::create_character_json(filedest);
        
        let mut automata = Vec::new();
        for automaton in characterjson.automata.into_iter() {
            automata.push(Automaton::create_automaton(automaton, &characterjson.dialogues));
        }

        let mut dialogues = HashMap::with_capacity(characterjson.dialogues.len());
        for (name, text) in characterjson.dialogues.iter() {
            dialogues.insert(name.to_owned(), Dialogue {
                text: text.to_vec(),
                new: true,
            });
        }

        return Character {
            name: characterjson.name,
            diary: Vec::new(),
            dialogues: dialogues,
            automata: automata,
        };
    }

    pub fn get_diary(&self) -> Vec<String> {
        return self.diary.clone();
    }

    pub fn get_dialogue(&mut self, dialogue: &String) -> Vec<String> {
        let mut dial = self.dialogues.get_mut(dialogue).expect("Le dialogue n'existe pas :sad:!");
        if dial.new {
            dial.new = false; 
            self.diary.push((&dialogue).to_string());
        }
        return dial.text.clone();
    }

    pub fn get_text(&mut self) -> Vec<String> {
        let mut dialogue_set = HashSet::new();
        for automaton in &mut self.automata {
            dialogue_set.insert(automaton.get_state().dialogue.clone());
        }

        let mut dialogue_vec = Vec::new();
        for dialogue in dialogue_set {
            dialogue_vec.extend(self.get_dialogue(&dialogue));
        }

        return dialogue_vec;
    }

    pub fn launch_event(&mut self, eventname: &str) {
        for automaton in &mut self.automata {
            automaton.launch_event(eventname);
        }
    }
}