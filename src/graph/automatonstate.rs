use std::collections::HashMap;

use crate::graph::graphparser::characterjson::StateJSON;

#[derive(Debug)]
pub struct AutomatonState {
    pub dialogue: String,
    pub location: String,
    pub events: HashMap<String, String>,
    pub edges: HashMap<String, String>
}

impl AutomatonState {
    pub fn create_state(automatonjson: StateJSON, dialogues: &HashMap<String, Vec<String>>, statesmap: &HashMap<String, StateJSON>) -> AutomatonState {
        
        assert!(dialogues.contains_key(&automatonjson.dialogue), "Le dialogue {} n'est pas défini !", automatonjson.dialogue);
        
        for (event, statename) in &automatonjson.edges {
            assert!(statesmap.contains_key(statename), "L'état {} n'est pas défini dans les transitions !", statename);
        }


        return AutomatonState {
            dialogue: automatonjson.dialogue,
            location: automatonjson.location,
            events: automatonjson.events,
            edges: automatonjson.edges,
        }
    }
}