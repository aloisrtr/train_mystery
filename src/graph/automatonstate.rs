use std::collections::HashMap;
use serde_derive::{Deserialize};


#[derive(Deserialize, Debug)]
pub struct AutomatonState {
    pub dialogue: String,
    pub location: i16,
    pub edges: HashMap<String, AutomatonState>,
}