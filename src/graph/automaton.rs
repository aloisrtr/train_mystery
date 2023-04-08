use std::collections::HashMap;


use crate::graph::graphparser::characterjson::AutomatonJSON;

use crate::graph::automatonstate::AutomatonState;


#[derive(Debug)]
pub struct Automaton {
    state: String,
    statemap: HashMap<String, AutomatonState>,
}

impl Automaton {
    pub fn create_automaton(automatonjson: AutomatonJSON, dialogues: &HashMap<String, Vec<String>>) -> Automaton {

        assert!(automatonjson.states.contains_key(&automatonjson.initstate), "L'état {} n'est pas défini !", automatonjson.initstate);

        let mut statemap = HashMap::with_capacity(automatonjson.states.len());
        for (statename, statejson) in automatonjson.states.clone().into_iter() {
            statemap.insert(statename.to_owned(), AutomatonState::create_state(statejson, dialogues, &automatonjson.states));
        }

        return Automaton {
            state: automatonjson.initstate,
            statemap: statemap,
        };
    }

    pub fn get_state(&self) -> &AutomatonState {
        return &self.statemap[&self.state];
    }
}