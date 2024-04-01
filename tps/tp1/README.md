Como compilar
```rust
rustc main.rs
./main [Buscar en el archivo] [Archivo de texto]

cargo run Fernand
cargo run ../README.md
```

# Ejemplos

Ejemplos de cada elemento de las expresiones regulares usando el comando `grep` en Linux:

1. **Caracteres Normales:**
   
   ```bash
   # Buscar la palabra "casa" en un archivo
   grep "casa" archivo.txt
   ```

2. **Metacaracteres:**
   - **Period (`.`):**

      ```bash
         # Buscar cualquier palabra de cuatro letras que comience con "c" y termine con "o"
         grep "t..o" archivo.txt
      ```
      ```bash
         " 
         jose todos somos
         t11R8% casa de maria
         "
         # Output:
         jose "todo"s somos
         "t11o"R8% casa de maria
      ```
     
     
   - **Bracket Expresion (`[]`):**
   
     ```bash
     # Buscar palabras que comiencen con "c", seguidas de una vocal y luego una "s"
     grep "c[aeiou]s" archivo.txt

     # Buscar palabras que comiencen con "c", que contengan una caracter y o un numero entre 1 y 9 y luego una "s"
     grep "c[y1-9]s" archivo.txt
     ```
     
   - **Bracket Expresion Negada (`^[]`):**
   
     ```bash
     # Buscar palabras que no contengan una "a"
     grep "[^a]" archivo.txt
     ```
  
3. **Character Classes:**
   - `[:digit:]`:
   
     ```bash
     # Buscar líneas que contengan un número
     grep "[[:digit:]]" archivo.txt
     ```

4. **Anchoring:**
   - `^`:
      **Inicio de línea:**
      ```bash
      # Buscar líneas que comiencen con "Inicio"
      grep "^jos" archivo.txt
      ```

      ```bash
      text =
      " 
      jose todos somos
      casa de maria
      josTender turro
      "
      # Output:
      "jos"e todos somos
      "jos"Tender turro
      ```


   - `$`:
   
      ```bash
      # Buscar líneas que terminen con "fin"
      grep "os$" archivo.txt
      ```

      ```bash
      text =
      " 
      jose todos somos
      casa de maria
      josTender turros
      "
      # Output:
      jose todos som"os"
      josTender turr"os"
      ```

5. **Repetition:**
   - `?`:
   
     ```bash
     # Buscar palabras que contengan "color" o "colour"
     grep "colou?r" archivo.txt
     ```
     
   - `*`:
   
     ```bash
     # Buscar palabras que contengan "aa", "aaa", "aaaa", etc.
     grep "a*" archivo.txt
     ```

   - `+`:
   
     ```bash
     # Buscar palabras que contengan "a", "aa", "aaa", etc., pero no "a"
     grep "a+" archivo.txt
     ```

   - `{n}`:
   
     ```bash
     # Buscar palabras de tres letras que comiencen con "a"
     grep "a\{3\}" archivo.txt
     ```

   - `{n,}`:
   
     ```bash
     # Buscar palabras de al menos tres letras que comiencen con "a"
     grep "a\{3,\}" archivo.txt
     ```

   - `{,m}`:
   
     ```bash
     # Buscar palabras de hasta tres letras que comiencen con "a"
     grep "a\{,3\}" archivo.txt
     ```

   - `{n,m}`:
   
     ```bash
     # Buscar palabras de tres a cinco letras que comiencen con "a"
     grep "a\{3,5\}" archivo.txt
     ```

Estos son solo algunos ejemplos básicos de cómo usar `grep` con diferentes elementos de las expresiones regulares. Puedes combinar estos elementos y ajustar los patrones según tus necesidades específicas de búsqueda.

# Además, su implementación deberá permitir la concatenación, la alternancia, y la precedencia de expresiones regulares. Por ejemplo las siguientes expresiones regulares deberán estar soportadas:
Aquí tienes ejemplos de cómo se pueden utilizar esas expresiones regulares en un contexto de búsqueda o manipulación de texto utilizando herramientas como `grep` en un sistema Unix/Linux:

1. `ab.cd`: Buscar líneas que contengan "ab", seguido de cualquier carácter, luego "cd".
   ```bash
   grep "ab.cd" archivo.txt
   ```

2. `ab.*cd`: Buscar líneas que contengan "ab", seguido de cero o más caracteres, luego "cd".
   ```bash
   grep "ab.*cd" archivo.txt
   ```

3. `a[bc]d`: Buscar líneas que contengan "ad", "abd" o "acd".
   ```bash
   grep "a[bc]d" archivo.txt
   ```

4. `ab{2,4}cd`: Buscar líneas que contengan "ab", seguido de 2 a 4 repeticiones de "b", luego "cd".
   ```bash
   grep "ab\{2,4\}cd" archivo.txt
   ```

5. `abc|de+f`: Buscar líneas que contengan "abc" o "def", "deef", "deeeef", etc.
   ```bash
   grep "abc\|de+f" archivo.txt
   ```

6. `la [aeiou] es una vocal`: Buscar líneas que contengan "la ", seguido de una vocal y luego " es una vocal".
   ```bash
   grep "la [aeiou] es una vocal" archivo.txt
   ```

7. `la ^[aeiou] no es una vocal`: Buscar líneas que contengan "la ", seguido de "^" y una vocal, y luego " no es una vocal".
   ```bash
   grep "la \^[aeiou] no es una vocal" archivo.txt
   ```

8. `hola [[:alpha:]]+`: Buscar líneas que contengan "hola ", seguido de uno o más caracteres alfabéticos.
   ```bash
   grep "hola [[:alpha:]]+" archivo.txt
   ```

9. `[[:digit:]] es un numero`: Buscar líneas que contengan un dígito seguido de " es un numero".
   ```bash
   grep "[[:digit:]] es un numero" archivo.txt
   ```

10. `el caracter [[:alnum:]] no es un simbolo`: Buscar líneas que contengan "el caracter ", seguido de un carácter alfanumérico y luego " no es un símbolo".
    ```bash
    grep "el caracter [[:alnum:]] no es un simbolo" archivo.txt
    ```

11. `hola[[:space:]]mundo`: Buscar líneas que contengan "hola", seguido de un espacio en blanco, y luego "mundo".
    ```bash
    grep "hola[[:space:]]mundo" archivo.txt
    ```

12. `[[:upper:]]ascal[[:upper:]]ase`: Buscar líneas que contengan una letra en mayúscula, seguida de "ascal", seguida de otra letra en mayúscula, y luego "ase".
    ```bash
    grep "[[:upper:]]ascal[[:upper:]]ase" archivo.txt
    ```

13. `es el fin$`: Buscar líneas que terminen con "es el fin".
    ```bash
    grep "es el fin$" archivo.txt
    ```

Estos son solo ejemplos de cómo podrías usar estas expresiones regulares con `grep`. Pueden variar dependiendo del contenido de tu archivo y de tus necesidades específicas.


# MyRegex
Se planteo el uso de &str y String para la implementación de la expresión regular.