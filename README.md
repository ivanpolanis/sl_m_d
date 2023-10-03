# SL-M-D

Este README proporciona una descripción general del código Rust para un sistema de mensajería cifrada. Este sistema permite a los usuarios cifrar y descifrar mensajes utilizando una matriz de clave predefinida. A continuación, se ofrece una explicación detallada de cómo funciona el código y cómo utilizarlo.

## Descripción general

El código proporcionado es un programa interactivo de línea de comandos que permite a los usuarios realizar dos acciones principales:

1. **Cifrar un mensaje**: El usuario introduce un mensaje de texto y el programa lo cifra utilizando una matriz de clave predefinida.

2. **Descifrar un mensaje**: El usuario introduce un mensaje cifrado y el programa lo descifra utilizando la inversa de la matriz de clave predefinida.

El programa también incluye opciones para salir y manejar entradas no válidas.

## Estructura del código

El código se divide en varias partes:

- **constants.rs**: Define constantes utilizadas en el programa, como la matriz de clave y su inversa.

- **main.rs**: Contiene la función `main()` que inicia la ejecución del programa. En este archivo, encontrarás el bucle principal que permite al usuario realizar acciones y manejar entradas.

- **Funciones auxiliares**: El código incluye varias funciones auxiliares que realizan tareas específicas, como mostrar el menú, convertir el texto de entrada en una forma adecuada para el cifrado, multiplicación de matrices y más.

## Uso del programa

1. **Ejecutar el programa**: Para ejecutar el programa, compila el código Rust y ejecuta el archivo binario resultante. El programa iniciará un bucle interactivo que permite al usuario seleccionar una opción de menú.

2. **Opciones de menú**: El programa presenta un menú con tres opciones:
   - `1`: Descifrar un mensaje
   - `0`: Cifrar un mensaje
   - `-1`: Salir

3. **Entrada de texto**: Cuando elijas cifrar o descifrar, se te pedirá que ingreses un mensaje de texto. Asegúrate de ingresar un mensaje válido.

4. **Resultados**: El programa mostrará el mensaje cifrado o descifrado, según la opción seleccionada.

5. **Errores y entradas no válidas**: El programa maneja errores y entradas no válidas, como letras o números no válidos en el menú.

## Notas importantes

- Este programa es un ejemplo educativo simple y no se recomienda para el cifrado real de datos sensibles. Los sistemas de cifrado reales deben contar con medidas de seguridad mucho más sólidas.

- Para pasar de letras a números y viceversa se usará la siguiente tabla como equivalencias:

| A | B | C | D | E | F | G | H | I | J | K | L | M | N | Ñ | O | P | Q | R | S | T | U | V | W | X | Y | Z | Espacio | . | , |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10| 11| 12| 13| 14| 15| 16| 17| 18| 19| 20| 21| 22| 23| 24| 25| 26| 27| 28 | 29 | 30|


## Contribuyentes

- [Iván Valentín Polanis](https://github,com/ivanpolanis)

- [Agustín Murray](https://github.com/agumurray)

