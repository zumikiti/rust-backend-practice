use std::io::stdin;

fn main() {
    let mut memory = Memory::new();
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
            memory.add_and_print_memory(tokens[0], prev_result);
            continue;
        } else if is_memory && tokens[0].ends_with("-") {
            memory.add_and_print_memory(tokens[0], -prev_result);
            continue;
        }

        // 計算
        let left = memory.eval_token(tokens[0]);
        let right = memory.eval_token(tokens[2]);
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
    slots: Vec<(String, f64)>,
}

impl Memory {
    fn new() -> Self {
        Self {
            slots: vec![],
        }
    }

    fn add_and_print_memory(&mut self, token: &str, prev_result: f64) {
        let slot_name = &token[3..token.len() - 1];
        for slot in self.slots.iter_mut() {
            if slot.0 == slot_name {
                // メモリが見つかったので、値を更新・表示して終了
                slot.1 += prev_result;
                print_value(slot.1);
                return;
            }
        }

        // メモリが見つからなかったので、最後の要素を追加する
        self.slots.push((slot_name.to_string(), prev_result));

        print_value(prev_result)
    }

    fn eval_token(&self, token: &str) -> f64 {
        if token.starts_with("mem") {
            let slot_name = &token[3..];
            for slot in &self.slots {
                if slot.0 == slot_name {
                    // メモリが見つかったので、値を返して終了
                    return slot.1;
                }
            }

            // メモリが見つからなかったので、初期値を返す
            0.0
        } else {
            token.parse().unwrap()
        }
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
