use rand::Rng;

fn main() {
    let mut mun_of_current = 0;

    while mun_of_current < 3 {
        let quiz_mode = rand::thread_rng().gen_range(1..=2);

        match quiz_mode {
            1 => {
                let op1 = rand::thread_rng().gen_range(0..100);
                let op2 = rand::thread_rng().gen_range(0..100);

                println!("{} + {} = ??", op1, op2);
                println!("?? の数値を入力してください:");

                let mut ans_input = String::new();
                std::io::stdin().read_line(&mut ans_input).unwrap();
                let ans_input = ans_input.trim().parse::<i32>().unwrap();
                if ans_input == op1 + op2 {
                    println!("正解");
                    mun_of_current += 1;
                } else {
                    println!("不正解");
                }
            }
            2 => {
                let op1 = rand::thread_rng().gen_range(0..100);
                let op2 = rand::thread_rng().gen_range(0..100);

                println!("{} - {} = ??", op1, op2);
                println!("?? の数値を入力してください:");

                let mut ans_input = String::new();
                std::io::stdin().read_line(&mut ans_input).unwrap();
                let ans_input = ans_input.trim().parse::<i32>().unwrap();

                if ans_input == op1 - op2 {
                    println!("正解");
                    mun_of_current += 1;
                } else {
                    println!("不正解");
                }
            }
            _ => unreachable!(),
        }

        println!("クリア！")
    }
}
