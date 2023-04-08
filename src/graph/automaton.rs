use std::collections::HashMap;


use crate::graph::graphparser::characterjson::AutomatonJSON;

use crate::graph::state::State;


#[derive(Debug)]
pub struct Automaton {
    state: String,
    statemap: HashMap<String, State>,
}

impl Automaton {
    pub fn create_automaton(automatonjson: AutomatonJSON, dialogues: &HashMap<String, Vec<String>>) -> Automaton {

        assert!(automatonjson.states.contains_key(&automatonjson.initstate), "L'état {} n'est pas défini !", automatonjson.initstate);

        let mut statemap = HashMap::with_capacity(automatonjson.states.len());
        for (statename, statejson) in automatonjson.states.clone().into_iter() {
            statemap.insert(statename.to_owned(), State::create_state(statejson, dialogues, &automatonjson.states));
        }

        return Automaton {
            state: automatonjson.initstate,
            statemap: statemap,
        };
    }

    pub fn get_state(&self) -> &State {
        return &self.statemap[&self.state];
    }

    pub fn launch_event(&mut self, eventname: &str) {
        let state = &self.get_state();
        if let Some(newstate) = state.edges.get(eventname) {
            self.state = newstate.to_string();
        } 
    }

}