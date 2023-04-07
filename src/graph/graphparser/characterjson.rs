use serde_derive::{Deserialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct StateJSON {
    dialogue: Vec<String>,
    location: String,
    edges: HashMap<String, String>
}

#[derive(Deserialize, Debug)]
pub struct CharacterJSON {
    name: String,
    states: HashMap<String, StateJSON>,
}

impl CharacterJSON {
    pub fn create_character_json(filedest: &str) -> CharacterJSON {
        println!("Ouverture du fichier {filedest}");
        
        let characterjsonfile = std::fs::read_to_string(&filedest).unwrap();
        let characterjson = serde_json::from_str::<CharacterJSON>(&characterjsonfile).unwrap();

        println!("{:?}", characterjson);
        return characterjson;
    }
}