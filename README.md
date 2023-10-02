# SL-M-D

This README provides an overview of the Rust code for an encrypted messaging system. This system allows users to encrypt and decrypt messages using a predefined key matrix. Below is a detailed explanation of how the code works and how to use it.

## Overview

The provided code is an interactive command-line program that allows users to perform two main actions:

1. **Encrypt a Message**: The user enters a text message, and the program encrypts it using a predefined key matrix.

2. **Decrypt a Message**: The user enters an encrypted message, and the program decrypts it using the inverse of the predefined key matrix.

The program also includes options for exiting and handling invalid inputs.

## Code Structure

The code is divided into several parts:

- **constants.rs**: Defines constants used in the program, such as the key matrix and its inverse.

- **main.rs**: Contains the `main()` function that initiates the program's execution. In this file, you'll find the main loop that allows the user to perform actions and handle inputs.

- **Auxiliary Functions**: The code includes several auxiliary functions that perform specific tasks, such as displaying the menu, converting input text into a suitable form for encryption, matrix multiplication, and more.

## Using the Program

1. **Running the Program**: To run the program, compile the Rust code and execute the resulting binary. The program will start an interactive loop that allows the user to select a menu option.

2. **Menu Options**: The program presents a menu with three options:
   - `1`: Decrypt a message
   - `0`: Encrypt a message
   - `-1`: Exit

3. **Text Input**: When you choose to encrypt or decrypt, you will be prompted to enter a text message. Ensure you input a valid message.

4. **Results**: The program will display the encrypted or decrypted message, depending on the selected option.

5. **Errors and Invalid Inputs**: The program handles errors and invalid inputs, such as invalid letters or numbers in the menu.

## Important Notes

- The key matrix used for encryption and its inverse are defined in the `constants.rs` file. Make sure you understand how these matrices affect encryption and decryption before using the program in a production environment.

- This program is a simple educational example and is not recommended for actual encryption of sensitive data. Real encryption systems must have much stronger security measures.

## Contributors

- [Iván Valentín Polanis](https://github,com/ivanpolanis)

- [Agustín Murray](https://github.com/agumurray)

## License

This code is provided under an open-source license. Please refer to the attached license file for more details.

Enjoy using this Rust-based encrypted messaging system!

