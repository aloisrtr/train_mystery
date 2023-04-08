use serde_derive::{Deserialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct StateJSON {
    pub dialogue: String,
    pub location: String,
    pub events: HashMap<String, String>,
    pub edges: HashMap<String, String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct AutomatonJSON {
    pub initstate: String,
    pub states: HashMap<String, StateJSON>
}

#[derive(Deserialize, Debug)]
pub struct CharacterJSON {
    pub name: String,
    pub dialogues: HashMap<String, Vec<String>>,
    pub automata: Vec<AutomatonJSON>,
}

impl CharacterJSON {
    pub fn create_character_json(filedest: &str) -> CharacterJSON {
        //println!("Ouverture du fichier {filedest}");
        
        let characterjsonfile = std::fs::read_to_string(&filedest).unwrap();
        let characterjson = serde_json::from_str::<CharacterJSON>(&characterjsonfile).unwrap();

        //println!("{:?}", characterjson);
        return characterjson;
    }
}