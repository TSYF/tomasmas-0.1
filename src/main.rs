use std::{env, fs};

fn ev(s: String) {
    // let mut stack: Vec<&str> = vec![];
    let mut stack: Vec<f64> = vec![];
    // let tokens = s.split_whitespace().collect::<Vec<&str>>();
    let tokens = s.split_whitespace();
    
    for token in tokens {
        if token.parse::<f64>().is_ok() {stack.push(token.parse::<f64>().unwrap_or_else(|err| panic!("{err}")))}
        match token {
            "+" => {
                let right = stack.pop();
                let left  = stack.pop();

                if left.is_none() && right.is_none() {
                    panic!("You done fucked up boii! You ain't got enough operators!");
                }

                let left  = left.unwrap_or_default();
                let right = right.unwrap_or_default();


                stack.append(&mut vec![(left + right)]);
            },
            "-" => {
                let right = stack.pop();
                let left  = stack.pop();

                if left.is_none() && right.is_none() {
                    panic!("You done fucked up boii! You ain't got enough operators!");
                }

                let left  = left.unwrap_or_default();
                let right = right.unwrap_or_default();


                stack.append(&mut vec![(left - right)]);
            },
            "/" => {
                let right = stack.pop();
                let left  = stack.pop();

                if left.is_none() && right.is_none() {
                    panic!("You done fucked up boii! You ain't got enough operators!");
                }

                let left  = left.unwrap_or_default();
                let right = right.unwrap_or_default();
                
                if right == 0.0 {
                    panic!("No division by zero!");
                }

                stack.append(&mut vec![(left / right)]);
            },
            "*" => {
                let right = stack.pop();
                let left  = stack.pop();

                if left.is_none() && right.is_none() {
                    panic!("You done fucked up boii! You ain't got enough operators!");
                }

                let left  = left.unwrap_or_default();
                let right = right.unwrap_or_default();

                stack.append(&mut vec![(left * right)]);
            },
            "**" => {
                let right = stack.pop();
                let left  = stack.pop();

                if left.is_none() && right.is_none() {
                    panic!("You done fucked up boii! You ain't got enough operators!");
                }

                let left  = left.unwrap_or_default();
                let right = right.unwrap_or_default();

                stack.append(&mut vec![(f64::powf(left, right))]);
            },
            "%" => {
                let right = stack.pop();
                let left  = stack.pop();

                if left.is_none() && right.is_none() {
                    panic!("You done fucked up boii! You ain't got enough operators!");
                }

                let left  = left.unwrap_or_default();
                let right = right.unwrap_or_default();


                stack.append(&mut vec![(left % right)]);
            },
            _ => {
                continue;
            }
        }
        println!("{}", stack.pop().unwrap_or_default());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath: &String = &args[1];
    let content: String = fs::read_to_string(filepath).unwrap_or_else(|err| {
        panic!("Couldn't read file {err:?}");
    });

    ev(content);
}
