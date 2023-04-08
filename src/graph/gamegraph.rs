use std::collections::HashMap;

use crate::graph::graphparser::gamegraphjson::GameGraphJSON;

use crate::graph::character::Character;

#[derive(Debug)]
pub struct GameGraph {
    pub characters: HashMap<String, Character>,
}

impl GameGraph {
    pub fn create_gamegraph(filedest: &str) -> GameGraph {
        let gamegraphjson = GameGraphJSON::create_gamegraph_json(filedest);
        let mut characters = HashMap::with_capacity(gamegraphjson.characters.len());
        for (name, characterfiledest) in gamegraphjson.characters.iter() {
            characters.insert(name.to_owned(), Character::create_character(characterfiledest));
        }

        return GameGraph {characters: characters};
    }    

    pub fn reset_character_automata(&mut self) {
        for (_name, character) in &mut self.characters {
            character.reset_automata();
        }
    }

    pub fn launch_event(&mut self, eventname: &str) {
        for (_name, character) in &mut self.characters {
            character.launch_event(eventname);
        }
    }
}

#[cfg(test)]
mod automata {
    use super::*;

    #[test]
    fn create_gamegraph_test() {
        let mut gamegraph = GameGraph::create_gamegraph("res/automata/characterlist.json");
        println!("{gamegraph:?}");
        let mut text = gamegraph.characters.get_mut("Allumeuse").expect("REASON").get_text();
        println!("{text:?}");
        gamegraph.launch_event("allumeuse_default_1");
        text = gamegraph.characters.get_mut("Allumeuse").expect("REASON").get_text();
        println!("{text:?}");
        let diary = gamegraph.characters.get_mut("Allumeuse").expect("REASON").get_diary();
        println!("{diary:?}");
        gamegraph.launch_event("allumeuse_default_2");
        text = gamegraph.characters.get_mut("Allumeuse").expect("REASON").get_text();
        println!("{text:?}");
    }
}