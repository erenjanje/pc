use clap::{arg, Command};

const HANDLERS: phf::Map<&'static str, fn(&mut Vec<f64>)> = phf::phf_map!{
    "+" => exec_plus,
    "-" => exec_sub,
    "*" => exec_mul,
    "/" => exec_div,
    "^" => exec_pow,
    "sin" => exec_sin,
    "p" => exec_print,
};

/**
Stack changes:

- 2 pop
- 1 push
 */
fn exec_plus(stack: &mut Vec<f64>) {
    let num2 = stack.pop().unwrap();
    let num1 = stack.pop().unwrap();
    stack.push(num1+num2);
}

/**
Stack changes:

- 2 pop
- 1 push
 */
fn exec_sub(stack: &mut Vec<f64>) {
    let num2 = stack.pop().unwrap();
    let num1 = stack.pop().unwrap();
    stack.push(num1-num2);
}

/**
Stack changes:

- 2 pop
- 1 push
 */
fn exec_mul(stack: &mut Vec<f64>) {
    let num2 = stack.pop().unwrap();
    let num1 = stack.pop().unwrap();
    stack.push(num1*num2);
}

/**
Stack changes:

- 2 pop
- 1 push
 */
fn exec_div(stack: &mut Vec<f64>) {
    let num2 = stack.pop().unwrap();
    let num1 = stack.pop().unwrap();
    stack.push(num1/num2);
}

/**
Stack changes:

- 2 pop
- 1 push
 */
fn exec_pow(stack: &mut Vec<f64>) {
    let num2 = stack.pop().unwrap();
    let num1 = stack.pop().unwrap();
    stack.push(num1.powf(num2));
}

/**
Stack changes:

- 1 pop
- 1 push
 */
fn exec_sin(stack: &mut Vec<f64>) {
    let num = stack.pop().unwrap();
    stack.push(num.sin());
}

/**
Stack changes:

- No change
 */
fn exec_print(stack: &mut Vec<f64>) {
    for (i, elem) in stack.iter().rev().enumerate() {
        println!("{}: {}", !(i as isize), elem);
    }
}

fn exec_identifier(stack: &mut Vec<f64>, identifier: &str) {
    match HANDLERS.get(identifier) {
        Some(fun) => fun(stack),
        None => panic!("Undefined operator: {identifier}"),
    }
}

fn exec_expression(expr: &str) {
    let mut stack = Vec::<f64>::new();
    exec(&mut stack, expr);
}

fn exec(stack: &mut Vec<f64>, expr: &str) {
    for tok in expr.split_whitespace() {
        let num = tok.parse::<f64>();
        match num {
            Ok(number) => stack.push(number),
            Err(_) => exec_identifier(stack, tok)
        }
        // println!("\"{tok}\": {:?}", stack);
    }
}

use rustyline::error::ReadlineError;
fn interactive() -> Result<bool, ReadlineError> {
    use rustyline::{DefaultEditor};
    
    let mut rl = DefaultEditor::new().unwrap();
    let mut stack = Vec::<f64>::new();

    loop {
        let line = rl.readline("> ");
        match line {
            Ok(line_string) => {
                rl.add_history_entry(&line_string)?;
                exec(&mut stack, &line_string);
            },
            Err(_) => break,
        }
    };
    return Ok(true);
}

fn main() {
    let matches = Command::new("pc")
        .version("0.0.1")
        .author("Erencan Ceyhan")
        .about("A postfix calculator written in Rust")
        .arg(arg!([filename] "file"))
        .arg(arg!(-s --string <STRING>))
        .arg(arg!(-i --interactive))
        .get_matches();

    if matches.get_flag("interactive") {
        interactive().unwrap();
    } else if let Some(expression_string) = matches.get_one::<String>("string") {
        exec_expression(&expression_string);
    } else {
        interactive().unwrap();
    }
}
