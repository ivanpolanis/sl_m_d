fn main() {
    let input = "abc".to_lowercase();
    let pwd: [[f64; 3]; 3] = [[1.0, 0.0, 5.0], [0.0, 2.0, 0.0], [4.0, 0.0, 3.0]];

    let result = to_number(&input, &pwd);

    for r in &result {
        println!("{:?}", r);
    }

    let mod_result = get_mod(&result);

    for r in &mod_result {
        println!("{:?}", r);
    }

    let message = get_message(&mod_result);

    let invs: [[f64; 3]; 3] = [
        [-3.0 / 17.0, 0.0, -5.0 / 17.0],
        [0.0, 1.0 / 2.0, 0.0],
        [4.0 / 17.0, 0.0, -1.0 / 17.0],
    ];

    let invs_result = to_number(&message, &invs);

    println!("{:?}", message);

    let original = get_message(&invs_result);

    println!("{:?}", original);
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

fn matrix_prod(arr: &Vec<f64>, pwd: &[[f64; 3]; 3]) -> Vec<f64> {
    let mut res = vec![0.0; 3];

    for i in 0..3 {
        for j in 0..3 {
            res[i] += arr[j] * pwd[j][i];
        }
        res[i] = res[i].round();
    }

    res
}

fn get_mod(arr: &Vec<Vec<f64>>) -> Vec<f64> {
    let mut res = vec![];

    println!("estoy con un modulo anashe");
    for i in arr.iter() {
        for j in 0..i.len() {
            println!("{:?}", (i[j] % 30.0).abs());
            if i[j] % 30.0 == 0.0 {
                res.push(30.0);
            } else {
                res.push(i[j] % 30.0);
            }
        }
    }

    res
}
