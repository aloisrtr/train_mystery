use std::collections::HashMap;

use crate::graph::graphparser::gamegraphjson::GameGraphJSON;

use crate::graph::automaton::Automaton;

#[derive(Debug)]
pub struct GameGraph {
    characters: HashMap<String, Automaton>,
}

impl GameGraph {
    pub fn create_gamegraph(filedest: &str) { //-> GameGraph {

    }
}

#[cfg(test)]
mod automata {
    use super::*;

    #[test]
    fn create_gamegraph_test() {
        GameGraphJSON::create_gamegraph_json("res/automata/characterlist.json");
    }
}