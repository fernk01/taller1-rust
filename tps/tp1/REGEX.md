# Repetition
Si cualquiera de estos caracteres se coloca al inicio de una expresión regular nose considera el metacaracter.

Cada uno de estos metacaracteres evaluan el caracter precedente, es decir, el caracter que se encuentra antes de ellos.

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