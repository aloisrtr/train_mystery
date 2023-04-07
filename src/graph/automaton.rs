use std::collections::HashMap;
use serde_derive::{Deserialize};

use crate::graph::automatonstate::AutomatonState;

pub fn create_automaton(filedest: &str) {
    println!("Ouverture du fichier {filedest}");
}

#[derive(Deserialize, Debug)]
pub struct Automaton {
    state: String,
    statelist: HashMap<String, AutomatonState>,
}

impl Automaton {

}


#[cfg(test)]
mod automata {
    use super::*;

    #[test]
    fn create_automaton_test() {
        create_automaton("res/automata/character1.json");
    }
}