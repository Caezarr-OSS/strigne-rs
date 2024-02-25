/// Programme pour compresser un fichier

use std::fs::File;
use std::io::Read;
use std::io::Write;
use flate2::Compression;
use flate2::write::GzEncoder;

/// Fonction principale du programme
fn main() {
    // ## Ouvrir le fichier à compresser

    /// Ouvre le fichier `fichier_a_compresser.txt` en mode lecture.
    let mut input_file = File::open("fichier_a_compresser.txt").unwrap();

    // ## Créer un fichier pour stocker le fichier compressé

    /// Crée un fichier `fichier_compresse.gz` en mode écriture.
    let mut output_file = File::create("fichier_compresse.gz").unwrap();

    // ## Encapsuleur GzEncoder pour la compression

    /// Crée un encodeur GzEncoder pour compresser les données écrites dans `output_file`.
    /// Le niveau de compression est défini par `Compression::default()`.
    let mut encoder = GzEncoder::new(&mut output_file, Compression::default());

    // ## Lire le fichier d'entrée par blocs et les écrire dans l'encodeur

    /// Buffer pour stocker les données lues du fichier d'entrée.
    let mut buf = [0; 4096];

    /// Boucle de lecture du fichier d'entrée par blocs.
    loop {
        /// Lit `n` octets du fichier d'entrée dans le buffer `buf`.
        let n = input_file.read(&mut buf).unwrap();

        /// Si la fin du fichier est atteinte, quitte la boucle.
        if n == 0 {
            break;
        }

        /// Écrit les `n` octets lus dans l'encodeur GzEncoder.
        encoder.write(&buf[..n]).unwrap();
    }

    // ## Finaliser l'encodage et fermer les fichiers

    /// Finalise l'encodage et flush les données restantes dans l'encodeur.
    encoder.finish().unwrap();

    /// Flush les données du fichier de sortie et ferme le fichier.
    output_file.flush().unwrap();
}
