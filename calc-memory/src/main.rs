use std::{
    collections::{hash_map::Entry, HashMap},
    io::stdin,
};

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
    slots: HashMap<String, f64>,
}

impl Memory {
    fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }

    fn add_and_print_memory(&mut self, token: &str, prev_result: f64) {
        let slot_name = token[3..token.len() - 1].to_string();
        match self.slots.entry(slot_name) {
            Entry::Occupied(mut entry) => {
                // メモリが見つかったので、値を更新・表示して終了
                *entry.get_mut() += prev_result;
                print_value(*entry.get())
            }
            Entry::Vacant(entry) => {
                // メモリが見つからなかったので、最後の要素を追加する
                entry.insert(prev_result);
                print_value(prev_result)
            }
        }
    }

    fn eval_token(&self, token: &str) -> f64 {
        if token.starts_with("mem") {
            let slot_name = &token[3..];
            // self.slots.get(slot_name) の戻り値は Option<&f64>
            // Option の中身が参照のままで値が返せないので
            // copied() で Option<f64> に変換
            // また、メモリが見つからない場合の値として、 0.0 を設定
            self.slots.get(slot_name).copied().unwrap_or(0.0)
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
