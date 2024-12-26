use std::io::stdin;

fn main() {
    let mut memory = Memory {
        slots: vec![0.0; 10],
    };
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
            add_and_print_memory(&mut memory, tokens[0], prev_result);
            continue;
        } else if is_memory && tokens[0].ends_with("-") {
            add_and_print_memory(&mut memory, tokens[0], -prev_result);
            continue;
        }

        // 計算
        let left = eval_token(tokens[0], &memory);
        let right = eval_token(tokens[2], &memory);
        let result = eval_expression(left, tokens[1], right);

        // 結果表示
        print_value(result);

        prev_result = result
    }
}

fn print_value(value: f64) {
    println!(" => {}", value)
}

struct Memory {
    slots: Vec<f64>,
}

fn add_and_print_memory(
    memory: &mut Memory,
    token: &str,
    prev_result: f64,
) {
    let slot_index: usize = match token[3..token.len() - 1].parse() {
        Ok(index) => index,
        Err(_) => {
            println!("token len {}", token.len());
            println!("Failed to parse slot index from token: {}", token);
            return;
        }
    };
    memory.slots[slot_index] += prev_result;
    print_value(memory.slots[slot_index])
}

fn eval_token(token: &str, memory: &Memory) -> f64 {
    if token.starts_with("mem") {
        let slot_index: usize = token[3..].parse().unwrap();
        memory.slots[slot_index]
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
