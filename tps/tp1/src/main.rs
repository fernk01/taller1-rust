mod regex; // Esto importa el archivo regex.rs

use std::env; // Importa el módulo env
use regex::Regex; // Importa la estructura Regex


fn main() {
    // let re = match Regex::new("ab.c") {
    //     Ok(regex) => regex,
    //     Err(e) => {
    //         println!("Error al crear la expresión regular: {}", e);
    //         return;
    //     }
    // };

    // let test_str = "bab1c";
    // let result = match re.test(test_str) {
    //     Ok(result) => result,
    //     Err(e) => {
    //         println!("Error al probar la expresión regular: {}", e);
    //         return;
    //     }
    // };

    // // Imprime el resultado
    // if result {
    //     println!("La cadena '{}' coincide con la expresión regular.", test_str);
    // } else {
    //     println!("La cadena '{}' no coincide con la expresión regular.", test_str);
    // }

        // Obtener los argumentos de la línea de comandos
        let args: Vec<String> = env::args().collect();

        // Verificar si se proporcionaron argumentos suficientes
        if args.len() < 3 {
            println!("Uso: {} <expresión regular> <cadena>", args[0]);
            return;
        }

        println!("Expresión regular: {}, {}", args[1], args[1].len());
        println!("Cadena: {}, {}", args[2], args[2].len());

        let a = "a\\[1";
        println!("Expresión regular: {}, {}", a, a.len());
}
