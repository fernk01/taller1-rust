use std::env;
use std::io;

use std::fs::File;
// Agregando el modulo de Buffer para lectura
use std::io::BufReader;
use std::io::prelude::*;


fn main() {
    match line_command() {
        Ok((text_find, file_name)) => {
            println!("Texto a buscar: {}", text_find);
            println!("Nombre del archivo: {}", file_name);

            // Llamar a la función para leer el archivo línea por línea
            match read_file_line_by_line(&file_name) {
                Ok(lines) => {
                    println!("Archivo leído línea por línea:");
                    for line in &lines {
                        if line.contains(&text_find) {
                            println!("{}", line);
                        }
                    }
                    for line in &lines {
                        println!("{}", line);
                    }

                }
                Err(e) => println!("Error al leer el archivo: {}", e),
            }
        }
        Err(e) => println!("Error al leer la línea de comandos: {}", e),
    }
}

fn line_command() -> Result<(String, String), std::io::Error> {
    // Obtener los argumentos de la línea de comandos
    let args: Vec<String> = env::args().collect();

    // Verificar si se proporcionaron argumentos suficientes
    if args.len() < 3 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Uso: <nombre_texto> <nombre_archivo>",
        ));
    }

    // Obtener el primer y segundo argumento pasados en la línea de comandos
    let text_find = args[1].clone();
    let file_name = args[2].clone();

    Ok((text_find, file_name))
}

fn read_file_line_by_line(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }

    Ok(lines)
}
