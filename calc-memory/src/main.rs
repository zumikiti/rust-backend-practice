use std::io::stdin;

fn main() {
    let mut memories: Vec<f64> = vec![0.0; 10];
    let mut prev_result: f64 = 0.0;

    for line in stdin().lines() {
        // 一行読み取って空白なら終了
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        // 空白分割
        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();

        // メモリへの書き込み
        let is_memory = tokens[0].starts_with("mem");
        if is_memory && tokens[0].ends_with("+") {
            add_and_print_memory(&mut memories, tokens[0], prev_result);
            continue;
        } else if is_memory && tokens[0].ends_with("-") {
            add_and_print_memory(&mut memories, tokens[0], -prev_result);
            continue;
        }

        // 計算
        let left = eval_token(tokens[0], memories);
        let right = eval_token(tokens[2], memories);
        let result = eval_expression(left, tokens[1], right);

        // 結果表示
        print_value(result);

        prev_result = result
    }
}

fn print_value(value: f64) {
    println!(" => {}", value)
}

fn add_and_print_memory(memories: &mut Vec<f64>, token: &str, prev_result: f64) {
    let slot_index: usize = token[3..token.len() - 1].parse().unwrap();
    memories[slot_index] += prev_result;
    print_value(memories[slot_index])
}

fn eval_token(token: &str, memories: Vec<f64>) -> f64 {
    if token.starts_with("mem") {
        let slot_index: usize = token[3..].parse().unwrap();
        memories[slot_index]
    } else {
        token.parse().unwrap()
    }
}

fn eval_expression(left: f64, operator: &str, right: f64) -> f64 {
    match operator {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        _ => {
            // 入力が正しい場合は、ここに来ない
            unreachable!()
        }
    }
}
