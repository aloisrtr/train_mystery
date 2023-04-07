use std::collections::HashMap;


use crate::graph::graphparser::gamegraphjson::GameGraphJSON;

use crate::graph::automatonstate::AutomatonState;


#[derive(Debug)]
pub struct Automaton {
    state: String,
    statelist: HashMap<String, AutomatonState>,
}

impl Automaton {
    pub fn create_automaton(filedest: &String) -> Automaton {

        return Automaton {
            state: String::new(),
            statelist: HashMap::new(),
        };
    }
}