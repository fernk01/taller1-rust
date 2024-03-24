use std::env;
use std::io;


use std::fs::File;
// Agregando el modulo de Buffer para lectura
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    // Obtener los argumentos de la línea de comandos
    let args: Vec<String> = env::args().collect();

    // Verificar si se proporcionaron argumentos suficientes
    if args.len() < 2 {
        println!("Uso: {} <nombre>", args[0]);
        return;
    }

    // Obtener el primer argumento pasado en la línea de comandos
    let nombre = &args[1];

    // Mostrar un mensaje con el nombre proporcionado
    println!("Hola, {}!", nombre);

    // Llamar a la función para leer el archivo
    match read_file(&nombre) {
        Ok(_) => println!("Archivo leído correctamente"),
        Err(e) => println!("Error al leer el archivo: {}", e),
    }

    // Llamar a la función para leer el archivo línea por línea
    // Llamar a la función para leer el archivo línea por línea
    match read_file_line_by_line(&nombre) {
        Ok(lines) => {
            println!("Archivo leído línea por línea:");
            for line in &lines {
                println!("{}", line);
            }

  
        }
        Err(e) => println!("Error al leer el archivo: {}", e),
    }


}

// Funcion para leer linea por linea un archivo
fn read_file(path : &str) -> std::io::Result<()> {
    // Vamos a abrir el archivo en modo read only
    // para ello utilizamos el metodo open al cual
    // debemos pasarle la ruta del archivo que deseamos leer
    let archivo = File::open(path)?;
    // Ahora creamos un buffer con el archivo, la ventaja de hacerlo
    // asi es que no debemos definir nuestro File como mutable, por
    // lo que es mas eficiente.
    let mut buf_reader = BufReader::new(archivo);
    let mut contenido = String::new();
    // Esta linea se encarga de leer el contenido completo
    // del archivo y lo coloca en un String, en este caso es
    // la mut variable contenido, el metodo read_to_string
    // recibe de parametro la referencia de la variable String
    // a la cual queremos pasarle el contenido del archivo
    buf_reader.read_to_string(&mut contenido)?;

    println!("Contenido del Archivo");
    println!("{}", contenido);

    Ok(())
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
