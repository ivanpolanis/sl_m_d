use std::io::Write;

enum Option {
    Encrypt,
    Decrypt,
}
fn menu() {
    println!("Seleccione una opción:");
    println!("1 - Desencriptar");
    println!("0 - Encriptar");
    println!("-1 - Salir");
}
fn main() {
    loop {
        menu();
        let mut option = String::new();

        std::io::stdin()
            .read_line(&mut option)
            .expect("No se puedo leer la entrada.");

        let option: i32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada no válida. Por favor, ingrese un número válido.");
                continue;
            }
        };

        let action = match option {
            1 => Option::Decrypt,
            0 => Option::Encrypt,
            -1 => break,
            _ => {
                println!("Opción no válida. Por favor, seleccione una opción válida.");
                continue;
            }
        };

        print!("Ingrese el texto: ");
        std::io::stdout()
            .flush()
            .expect("Error al limpiar el búfer de salida.");
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Tu entrada no es valida.");

        let input = parse_input(input);
        match action {
            Option::Encrypt => {
                let pwd: [[f64; 3]; 3] = [[35.0, 53.0, 12.0], [12.0, 21.0, 5.0], [2.0, 4.0, 1.0]];
                let result = to_number(&input, &pwd);

                let mod_result = get_mod(&result);

                let encrypted_msg = get_message(&mod_result);

                println!("El mensaje encriptado es: {:?}", encrypted_msg);
            }
            Option::Decrypt => {
                let invs: [[f64; 3]; 3] = [[1.0, 25.0, 13.0], [28.0, 11.0, 29.0], [6.0, 26.0, 9.0]];

                let invs_result = to_number(&input, &invs);

                let invs_mod = get_mod(&invs_result);

                let decrypted_msg = get_message(&invs_mod);
                println!("El mensaje desencriptado es: {:?}", decrypted_msg);
            }
        }

        let _invs: [[f64; 3]; 3] = [[1.0, -5.0, 13.0], [-2.0, 11.0, -31.0], [6.0, -34.0, 99.0]];
    }
}

fn parse_input(str: String) -> String {
    let mut input = str.trim().to_lowercase();

    let spaces_to_add = if input.len() % 3 != 0 {
        3 - (input.len() % 3)
    } else {
        0
    };

    input.push_str(&" ".repeat(spaces_to_add));

    return input;
}

fn to_number(input: &str, pwd: &[[f64; 3]; 3]) -> Vec<Vec<f64>> {
    let arr: Vec<char> = "0abcdefghijklmnñopqrstuvwxyz .,".chars().collect();

    let mut result = Vec::new();

    for group in input.chars().collect::<Vec<char>>().chunks(3) {
        let col = group
            .iter()
            .map(|&c| arr.iter().position(|&x| x == c).unwrap() as f64)
            .collect::<Vec<f64>>();
        let response = matrix_prod(&col, &pwd);
        result.push(response);
    }

    result
}

fn get_message(input: &Vec<f64>) -> String {
    let arr: Vec<char> = "0abcdefghijklmnñopqrstuvwxyz .,".chars().collect();
    let mut res = String::new();

    for letter in input {
        res.push(arr[*letter as usize]);
    }

    res
}

fn matrix_prod(col: &Vec<f64>, pwd: &[[f64; 3]; 3]) -> Vec<f64> {
    let mut res = vec![0.0; 3];

    for i in 0..3 {
        for j in 0..3 {
            res[i] += col[j] * pwd[i][j];
        }
        res[i] = res[i].round();
    }

    res
}

fn get_mod(arr: &Vec<Vec<f64>>) -> Vec<f64> {
    let mut res = vec![];

    for i in arr.iter() {
        for j in 0..i.len() {
            if i[j] % 30.0 == 0.0 {
                res.push(30.0);
            } else {
                res.push(i[j] % 30.0);
            }
        }
    }

    res
}
