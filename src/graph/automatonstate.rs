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
    pub fn create_state(automatonjson: StateJSON) -> AutomatonState {
        return AutomatonState {
            dialogue: automatonjson.dialogue,
            location: automatonjson.location,
            events: automatonjson.events,
            edges: automatonjson.edges,
        }
    }
}