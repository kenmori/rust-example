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
            add_and_print_memory(memory, prev_result);
            continue;
        } else if tokens[0] == "mem-" {
            add_and_print_memory(memory, -prev_result);
            continue;
        }

        // let left: f64 = tokens[0].parse().unwrap();
        // let right: f64 = tokens[2].parse().unwrap();

        let left = eval_token(tokens[0], memory);

        let right = eval_token(tokens[2], memory);

        let result = eval_expression(left, tokens[1], right);
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

fn add_and_print_memory(mut memory: f64, prev_result: f64) {
    memory += prev_result;
    print_output(memory);
}

fn eval_expression(left: f64, operator: &str, right: f64) -> f64 {
    match operator {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        _ => {
            unreachable!();
        }
    }
}
fn print_output(result: f64) {
    println!(" => {}", result);
}
