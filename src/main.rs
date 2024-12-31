use std::{collections::HashMap, env, fs};

struct Interpreter {
    vars: HashMap<String, String>
}

impl Interpreter {

    pub fn eval(&mut self, s: String) {
        let lines = s.split("\n").filter(|el| el.trim() != ""); 
        for line in lines {
            let [name, sign, expr]: [&str; 3] = line.splitn(3, ' ').collect::<Vec<&str>>().try_into().unwrap();

            if name == "//" {
                continue;
            }

            if sign == "=" {
                let result: String = self.eval_expression(expr).to_string();
                self.vars.insert(String::from(name), result);
                continue;
            }

            println!("{}", self.eval_expression(expr).to_string());
        }
    }

    fn eval_expression(&self, s: &str) -> f64 {
        // let mut stack: Vec<&str> = vec![];
        let mut stack: Vec<f64> = vec![];
        // let tokens = s.split_whitespace().collect::<Vec<&str>>();
        let tokens = s.split(" ");

        for token in tokens {
            if token.parse::<f64>().is_ok() {
                stack.push(token.parse::<f64>().unwrap_or_else(|err| panic!("{err}")));
                continue;
            } else if self.vars.get(token).is_some() {
                stack.push(self.vars.get(token).unwrap().parse::<f64>().unwrap_or_default());
                continue;
            }
            match token {
                "+" => {
                    let (left, right) = self.destructure_f64_expr(&mut stack);
    
                    stack.append(&mut vec![(left + right)]);
                },
                "-" => {
                    let (left, right) = self.destructure_f64_expr(&mut stack);
    
                    stack.append(&mut vec![(left - right)]);
                },
                "/" => {
                    let (left, right) = self.destructure_f64_expr(&mut stack);
                    
                    if right == 0.0 {
                        panic!("No division by zero!");
                    }
    
                    stack.append(&mut vec![(left / right)]);
                },
                "*" => {
                    let (left, right) = self.destructure_f64_expr(&mut stack);
    
                    stack.append(&mut vec![(left * right)]);
                },
                "**" => {
                    let (left, right) = self.destructure_f64_expr(&mut stack);
    
                    stack.append(&mut vec![(f64::powf(left, right))]);
                },
                "%" => {
                    let (left, right) = self.destructure_f64_expr(&mut stack);
    
                    stack.append(&mut vec![(left % right)]);
                },
                _ => {
                    continue;
                }
            }
        }
        return stack[0];
    }

    fn destructure_f64_expr(&self, stack: &mut Vec<f64>) -> (f64, f64) {
        let right = stack.pop();
        let left  = stack.pop();

        if left.is_none() && right.is_none() {
            panic!("You done fucked up boii! You ain't got enough operators!");
        }

        let left  = left.unwrap_or_default();
        let right = right.unwrap_or_default();

        (left, right)
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath: &String = &args[1];
    let content: String = fs::read_to_string(filepath).unwrap_or_else(|err| {
        panic!("Couldn't read file {err:?}");
    });

    let mut interpreter = Interpreter { vars: HashMap::new() };

    interpreter.eval(content);
}
