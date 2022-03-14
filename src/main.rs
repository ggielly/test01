//use std::env;
//use std::fs;

// Pour lire le fichier en mode buffer
use std::fs::File;
use std::io::{self, BufReader,BufRead};
use std::str;


fn main() {
    println!("Lecture de texte.");

    fonction_lecture();
}



/* Structure de la trame LINKY

    ADCO : Identifiant du compteur
    OPTARIF : Option tarifaire (type d’abonnement)
    ISOUSC : Intensité souscrite
    BASE : Index si option = base (en Wh)
    HCHC : Index heures creuses si option = heures creuses (en Wh)
    HCHP : Index heures pleines si option = heures creuses (en Wh)
    EJP HN : Index heures normales si option = EJP (en Wh)
    EJP HPM : Index heures de pointe mobile si option = EJP (en Wh)
    BBR HC JB : Index heures creuses jours bleus si option = tempo (en Wh)
    BBR HP JB : Index heures pleines jours bleus si option = tempo (en Wh)
    BBR HC JW : Index heures creuses jours blancs si option = tempo (en Wh)
    BBR HC JW : Index heures pleines jours blancs si option = tempo (en Wh)
    BBR HC JR : Index heures creuses jours rouges si option = tempo  (en Wh)
    BBR HP JR : Index heures pleines jours rouges si option = tempo (en Wh)
    PEJP : Préavis EJP si option = EJP 30mn avant période EJP
    PTEC : Période tarifaire en cours
    DEMAIN : Couleur du lendemain si option = tempo
    IINST : Intensité instantanée (en ampères)
    ADPS : Avertissement de dépassement de puissance souscrite (en ampères)
    IMAX : Intensité maximale (en ampères)
    PAPP : Puissance apparente (en Volt.ampères)
    HHPHC : Groupe horaire si option = heures creuses ou tempo
    MOTDETAT : Mot d’état (autocontrôle)

Une trame commence toujours par l’étiquette ADCO et se termine par le MOTDETAT.
Chaque message, ou ligne, d’une trame est formé de la manière suivante :

ETIQUETTE espace VALEUR espace CHECKSUM

Seules l’ETIQUETTE et la VALEUR nous seront utiles. La CHEKSUM, ou somme de contrôle sert uniquement à vérifier l’intégrité que la trame.





 */


fn fonction_lecture() -> io::Result<()> {

    // lecture du fichier d'exemple de la trame
    let filename = File::open("ok.txt")?;


    // boucle de parse

    let reader = BufReader::new(filename);
    let mut i = 0;

    for vars in reader.split(b' ') {
        //let mut i = i + 1;
        let bytes = vars.unwrap();
        let s = str::from_utf8(&bytes).unwrap();
        println!("Ligne {} {}", s, i+1);

        }



/*    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        println!("{}. {}", index + 1, line);
    }
    */



/*    let contents = fs::read_to_string(filename)
        .expect("Erreur de lecture du fichier.");
    println!("Qui contient le texte :\n{}", contents); */

    Ok(())

}
