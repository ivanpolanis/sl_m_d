fn main() {
    let mut input = String::new();

    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_e) => {
            println!("Tu entrada no es valida");
        }
    }

    let input = parse_input(input);

    let pwd: [[f64; 3]; 3] = [[35.0, 53.0, 12.0], [12.0, 21.0, 5.0], [2.0, 4.0, 1.0]];

    let result = to_number(&input, &pwd);

    let mod_result = get_mod(&result);

    let message = get_message(&mod_result);

    let _invs: [[f64; 3]; 3] = [[1.0, -5.0, 13.0], [-2.0, 11.0, -31.0], [6.0, -34.0, 99.0]];

    let invs: [[f64; 3]; 3] = [[1.0, 25.0, 13.0], [28.0, 11.0, 29.0], [6.0, 26.0, 9.0]];

    let invs_result = to_number(&message, &invs);

    println!("{:?}", message);

    let invs_mod = get_mod(&invs_result);

    let original = get_message(&invs_mod);
    println!("{:?}", original);
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
