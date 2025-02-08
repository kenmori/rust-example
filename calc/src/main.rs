use std::io::stdin;

fn main() {
    // let some_values: i32 = {
    //     let mut line = String::new();
    //     std::io::stdin().read_line(&mut line).unwrap();
    //     line.trim().parse().unwrap()
    // };
    // println!("{}", some_values);

    let mut memory: f64 = 0.0;
    let mut prev_result: f64 = 0.0;

    for line in stdin().lines() {
        let line = line.unwrap();

        if line.is_empty() {
            break;
        }

        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();

        if tokens[0] == "mem+" {
            memory += prev_result;
            print_output(memory);
            continue;
        } else if tokens[0] == "mem-" {
            memory -= prev_result;
            print_output(memory);
            continue;
        }

        // let left: f64 = tokens[0].parse().unwrap();
        // let right: f64 = tokens[2].parse().unwrap();

        let left = eval_token(tokens[0], memory);

        let right = eval_token(tokens[2], memory);

        let result = match tokens[1] {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            _ => {
                unreachable!();
            }
        };
        print_output(result);
        prev_result = result;
    }
}

fn eval_token(token: &str, memory: f64) -> f64 {
    if token == "mem" {
        memory
    } else {
        token.parse().unwrap()
    }
}

fn print_output(result: f64) {
    println!(" => {}", result);
}
