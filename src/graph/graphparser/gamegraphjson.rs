use serde_derive::{Deserialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct GameGraphJSON {
    pub characters: HashMap<String, String>,
}

impl GameGraphJSON {
    pub fn create_gamegraph_json(filedest: &str) -> GameGraphJSON {
        //println!("Ouverture du fichier {filedest}");
        
        let gamegraphjsonfile = std::fs::read_to_string(&filedest).unwrap();
        let gamegraphjson = serde_json::from_str::<GameGraphJSON>(&gamegraphjsonfile).unwrap();

        //println!("{:?}", gamegraphjson);
        return gamegraphjson;
    }
}