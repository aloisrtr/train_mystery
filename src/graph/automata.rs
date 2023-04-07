use std::fs;

pub fn create_automata(nom_fichier: &str) {
        println!("Dans le fichier : {}", nom_fichier);
    
        let contenu = fs::read_to_string(nom_fichier)
            .expect("Quelque chose s'est mal passé lors de la lecture du fichier");
    
        println!("Dans le texte :\n{}", contenu);
}

#[cfg(test)]
mod automata {
    use super::*;

    #[test]
    fn create_automata_test() {
        create_automata("res/automata/perso1.aut");
    }
}