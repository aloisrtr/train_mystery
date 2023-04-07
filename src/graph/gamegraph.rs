use std::collections::HashMap;

use crate::graph::graphparser::gamegraphjson::GameGraphJSON;

use crate::graph::character::Character;

#[derive(Debug)]
pub struct GameGraph {
    characters: HashMap<String, Character>,
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
}

#[cfg(test)]
mod automata {
    use super::*;

    #[test]
    fn create_gamegraph_test() {
        let gamegraph = GameGraph::create_gamegraph("res/automata/characterlist.json");
        println!("{gamegraph:?}");
    }
}