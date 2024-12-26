use std::io::stdin;

fn main() {
    let mut memory: f64 = 0.0;
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
        if tokens[0] == "mem+" {
            memory += prev_result;
            print_value(memory);

            continue;
        } else if tokens[0] == "mem-" {
            memory -= prev_result;
            print_value(memory);

            continue;
        }

        // 計算
        let left = if tokens[0] == "mem" {
            memory
        } else {
            tokens[0].parse().unwrap()
        };
        let right = if tokens[2] == "mem" {
            memory
        } else {
            tokens[2].parse().unwrap()
        };
        let result = match tokens[1] {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            _ => {
                // 入力が正しい場合は、ここに来ない
                unreachable!()
            }
        };

        // 結果表示
        print_value(result);

        prev_result = result
    }

    fn print_value(value: f64) {
        println!(" => {}", value)
    }
}
