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
        let left = eval_token(tokens[0], memory);
        let right = eval_token(tokens[2], memory);
        let result = eval_expression(left, tokens[1], right);

        // 結果表示
        print_value(result);

        prev_result = result
    }

    fn print_value(value: f64) {
        println!(" => {}", value)
    }

    fn eval_token(token: &str, memory: f64) -> f64 {
        if token == "mem" {
            memory
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
}
