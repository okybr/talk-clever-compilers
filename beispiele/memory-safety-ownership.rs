use std::error::Error;
use std::io;
use std::result::Result;
use std::vec::Vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Lese Arraygröße zur Laufzeit ein:
    let mut eingabe = String::new();
    io::stdin().read_line(&mut eingabe)?;
    let anzahl: usize = eingabe.trim().parse()?;

    // Definiere array als Referenz auf einen neu allokierten Speicherbereich:
    let mut array: Vec<i32> = Vec::with_capacity(anzahl);

    // Definiere weitere_referenz als weitere Referenz auf selbigen Speicher:
    // Dabei passiert ein sogenannter Move.
    let mut weitere_referenz = array;

    // Versuche Variable array zu benuten:
    array[0] = 1;

    Ok(())

    // Am Ende des Blocks wird implizit die Drop-Funktion auf array aufgerufen.
}
