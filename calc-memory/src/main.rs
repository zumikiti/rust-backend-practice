use std::io::stdin;

fn main() {
    for line in stdin().lines() {
        // 一行読み取って空白なら終了
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        // 空白分割
        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();

        // 計算
        let left: f64 = tokens[0].parse().unwrap();
        let right: f64 = tokens[2].parse().unwrap();
        let result = match tokens[1] {
            "+" => add_value(left, right),
            "-" => subtract_value(left, right),
            "*" => multiply_value(left, right),
            "/" => divide_value(left, right),
            _ => {
                // 入力が正しい場合は、ここに来ない
                unreachable!()
            }
        };

        // 結果表示
        print_value(result)
    }

    fn print_value(value: f64) {
        println!(" => {}", value)
    }

    fn add_value(left: f64, right: f64) -> f64 {
        left + right
    }

    fn subtract_value(left: f64, right: f64) -> f64 {
        left - right
    }

    fn multiply_value(left: f64, right: f64) -> f64 {
        left * right
    }

    fn divide_value(left: f64, right: f64) -> f64 {
        left / right
    }
}
