use rand::Rng;

fn main() {
    let mut num_of_correct = 0;
    while num_of_correct < 3 {
        let quiz_mode = rand::rng().random_range(1..=2);
        match quiz_mode {
            1 => loop {
                let op1 = rand::rng().random_range(0..100);
                let op2 = rand::rng().random_range(0..100);

                println!("{} + {} =??", op1, op2);
                println!("?? の値を入力してください。");
                let mut ans_input = String::new();
                std::io::stdin().read_line(&mut ans_input).unwrap();

                let ans_input = ans_input.trim().parse::<i32>().unwrap();
                dbg!(ans_input);

                if dbg!(ans_input == op1 + op2) {
                    println!("正解です！");
                    num_of_correct += 1;
                    break;
                } else {
                    println!("不正解です。正解は {} です。", op1 + op2);
                }
            },
            2 => loop {
                let op1 = rand::rng().random_range(0..100);
                let op2 = rand::rng().random_range(0..100);

                println!("{} - {} =??", op1, op2);
                println!("?? の値を入力してください。");
                let mut ans_input = String::new();
                std::io::stdin().read_line(&mut ans_input).unwrap();

                let ans_input = ans_input.trim().parse::<i32>().unwrap();
                dbg!(ans_input);

                if dbg!(ans_input == op1 - op2) {
                    println!("正解です！");
                    num_of_correct += 1;
                    break;
                } else {
                    println!("不正解です。正解は {} です。", op1 - op2);
                }
            },
            _ => unreachable!(),
        }
    }
}
