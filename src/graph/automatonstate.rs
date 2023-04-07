use std::collections::HashMap;


#[derive(Debug)]
pub struct AutomatonState {
    pub dialogue: String,
    pub location: String,
    pub edges: HashMap<String, AutomatonState>,
}