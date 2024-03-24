use std::env;
// use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
// use colored::*;

fn main() {
    match line_command() {
        Ok((text_find, file_name)) => {
            //println!("Texto a buscar: {}", text_find);
            //println!("Nombre del archivo: {}", file_name);

            let file = File::open(file_name);
            match file {
                Ok(f) => {
                    let lector = BufReader::new(f);

                    for line in lector.lines() {
                        match line {
                            Ok(line) => println!("Linea leida: {}", line),
                            Err(e) => println!("Error al leer la línea: {}", e),
                        }
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
            "Uso: my_grep <regular_expression> <path/to/file>",
        ));
    }

    // Obtener el primer y segundo argumento pasados en la línea de comandos
    let text_find = args[1].clone();
    let file_name = args[2].clone();

    Ok((text_find, file_name))
}
