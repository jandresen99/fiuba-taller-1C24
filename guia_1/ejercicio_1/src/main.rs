use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{ self, BufRead, BufReader };
//use std::collections::HashMap;

fn read_lines(filename: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    Ok(lines)
}

fn main() -> Result<(), std::io::Error>{
    println!("Bienvenido al ahorcado de FIUBA!");
    let mut palabras = Vec::new();
    let mut adivinadas: Vec<char> = Vec::new();
    //let mut respuesta = HashMap::new();
    let mut intentos = 10;
    let mut palabra = String::new();

    let lines = read_lines("palabras.txt")?;
    for line in lines {
        palabras.push(line);
    }

    println!("{:?}", palabras);

    if let Some(p) = palabras.choose(&mut rand::thread_rng()) {
        palabra = p.clone();
    } else {
        println!("No se encontraron palabras para elegir.");
    }

    let adivinar = "_".repeat(palabra.len());

    let mut palabra_vec: Vec<char> = palabra.chars().collect();
    let mut adivinar_vec: Vec<char> = adivinar.chars().collect();

    let concatenated_string: String = adivinar_vec.iter().collect();


    println!("{:?}", palabra_vec);
    println!("{:?}", adivinar_vec);
    println!("{}", concatenated_string);


    //for i in "_".repeat(palabra.len()){
    //    println!("La palabra hasta el momento es: {}", adivinar);
    //    println!("Adivinaste las siguientes letras: {}", adivinar);
    //    
    //}

    Ok(())
}