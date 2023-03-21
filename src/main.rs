use clap::{arg, Command};

#[derive(Debug)]
struct Matrix {
    row: usize,
    col: usize,
    data: Box<[f64]>,
}

impl Matrix {
    fn from(row: usize, col: usize, data: Box<[f64]>) -> Matrix {
        Matrix{
            row: row,
            col: col,
            data: data
        }
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")?;
        for i in 0..self.row {
            write!(f, "   ")?;
            for j in 0..self.col {
                write!(f, " {} ", self.data[i*self.col + j])?;
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

impl std::ops::Add<Matrix> for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Matrix) -> Self::Output {
        Matrix::from(self.row, self.col, std::iter::zip(self.data.iter(), rhs.data.iter())
            .map(|(a,b)| a+b)
            .collect::<Vec<f64>>()
            .into_boxed_slice())
    }
}

impl std::ops::Sub<Matrix> for Matrix {
    type Output = Matrix;

    fn sub(self, rhs: Matrix) -> Self::Output {
        Matrix::from(self.row, self.col, std::iter::zip(self.data.iter(), rhs.data.iter())
            .map(|(a,b)| a-b)
            .collect::<Vec<f64>>()
            .into_boxed_slice())
    }
}

enum Value {
    Number(f64),
    Matrix(Matrix),
}

impl Value {
    fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    fn is_matrix(&self) -> bool {
        match self {
            Value::Matrix(_) => true,
            _ => false,
        }
    }

    fn to_number(&self) -> Option<f64> {
        match self {
            Value::Number(num) => Some(*num),
            _ => None,
        }
    }

    fn to_matrix<'lt>(&'lt self) -> Option<&'lt Matrix> {
        match self {
            Value::Matrix(ref mat) => Some(mat),
            _ => None
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{}", num),
            Self::Matrix(mat) => write!(f, "{}", mat),
        }
    }
}

const HANDLERS: phf::Map<&'static str, fn(&mut Vec<Value>)> = phf::phf_map!{
    "pi" => exec_pi,
    "+" => exec_plus,
    "-" => exec_sub,
    "*" => exec_mul,
    "/" => exec_div,
    "^" => exec_pow,
    "sin" => exec_sin,
    "cos" => exec_cos,
    "tan" => exec_tan,
    "cot" => exec_cot,
    "exp" => exec_exp,
    "asin" => exec_asin,
    "acos" => exec_acos,
    "atan" => exec_atan,
    "acot" => exec_acot,
    "atan2" => exec_atan2,
    "p" => exec_print,
    "matrix" => exec_matrix,
};

/**
Stack changes:

- 1 push
 */
fn exec_pi(stack: &mut Vec<Value>) {
    stack.push(Value::Number(std::f64::consts::PI));
}

/**
Variables: `col` then `row`

Stack changes:

- `row`*`col` + 2 pop
- 1 push
 */
fn exec_matrix(stack: &mut Vec<Value>) {
    let col_value = stack.pop().unwrap();
    let row_value = stack.pop().unwrap();
    if row_value.is_matrix() || col_value.is_matrix() {
        panic!("Matrix size must be numbers");
    }
    // We checked before, so unwrap cannot panic
    let row = row_value.to_number().unwrap() as usize;
    let col = col_value.to_number().unwrap() as usize;
    let mut mat = Vec::<f64>::with_capacity(row*col);
    for _ in 0..(row*col) {
        mat.push(stack.pop().unwrap().to_number().expect("Matrix elements must be numbers"));
    }
    mat.reverse();
    stack.push(Value::Matrix(Matrix::from(row, col, mat.into_boxed_slice())));
}

/**
Stack changes:

- 2 pop
- 1 push
 */
fn exec_plus(stack: &mut Vec<Value>) {
    let val2 = stack.pop().unwrap();
    let val1 = stack.pop().unwrap();
    match (val1, val2) {
        (Value::Number(lhs), Value::Number(rhs)) => {
            stack.push(Value::Number(lhs+rhs));
        }
        (Value::Matrix(lhs), Value::Matrix(rhs)) => {
            stack.push(Value::Matrix(lhs+rhs));
        }
        (lhs,rhs) => panic!("Unsupported operations on {} and {}", lhs, rhs),
    }
}

/**
Stack changes:

- 2 pop
- 1 push
 */
fn exec_sub(stack: &mut Vec<Value>) {
    let val2 = stack.pop().unwrap();
    let val1 = stack.pop().unwrap();
    match (val1, val2) {
        (Value::Number(lhs), Value::Number(rhs)) => {
            stack.push(Value::Number(lhs-rhs));
        }
        (Value::Matrix(lhs), Value::Matrix(rhs)) => {
            stack.push(Value::Matrix(lhs-rhs));
        }
        (lhs,rhs) => panic!("Unsupported operations on {} and {}", lhs, rhs),
    }
}

/**
Stack changes:

- 2 pop
- 1 push
 */
fn exec_mul(stack: &mut Vec<Value>) {
    let val2 = stack.pop().unwrap();
    let val1 = stack.pop().unwrap();
    match (val1, val2) {
        (Value::Number(lhs), Value::Number(rhs)) => {
            stack.push(Value::Number(lhs*rhs));
        }
        (lhs,rhs) => panic!("Unsupported operations on {} and {}", lhs, rhs),
    }
}

/**
Stack changes:

- 2 pop
- 1 push
 */
fn exec_div(stack: &mut Vec<Value>) {
    let val2 = stack.pop().unwrap();
    let val1 = stack.pop().unwrap();
    match (val1, val2) {
        (Value::Number(lhs), Value::Number(rhs)) => {
            stack.push(Value::Number(lhs/rhs));
        }
        (lhs,rhs) => panic!("Unsupported operations on {} and {}", lhs, rhs),
    }
}

/**
Stack changes:

- 2 pop
- 1 push
 */
fn exec_pow(stack: &mut Vec<Value>) {
    let val2 = stack.pop().unwrap();
    let val1 = stack.pop().unwrap();
    match (val1, val2) {
        (Value::Number(lhs), Value::Number(rhs)) => {
            stack.push(Value::Number(lhs.powf(rhs)));
        }
        (lhs,rhs) => panic!("Unsupported operations on {} and {}", lhs, rhs),
    }
}

/**
Stack changes:

- 1 pop
- 1 push
 */
fn exec_sin(stack: &mut Vec<Value>) {
    let val = stack.pop().unwrap();
    match val {
        Value::Number(value) => {
            stack.push(Value::Number(value.sin()));
        },
        value => panic!("Unsupported operation on {}", value),
    }
}

/**
Stack changes:

- 1 pop
- 1 push
 */
fn exec_cos(stack: &mut Vec<Value>) {
    let val = stack.pop().unwrap();
    match val {
        Value::Number(value) => {
            stack.push(Value::Number(value.cos()));
        },
        value => panic!("Unsupported operation on {}", value),
    }
}

/**
Stack changes:

- 1 pop
- 1 push
 */
fn exec_tan(stack: &mut Vec<Value>) {
    let val = stack.pop().unwrap();
    match val {
        Value::Number(value) => {
            stack.push(Value::Number(value.tan()));
        },
        value => panic!("Unsupported operation on {}", value),
    }
}

/**
Stack changes:

- 1 pop
- 1 push
 */
fn exec_cot(stack: &mut Vec<Value>) {
    let val = stack.pop().unwrap();
    match val {
        Value::Number(value) => {
            stack.push(Value::Number(1.0/value.tan()));
        },
        value => panic!("Unsupported operation on {}", value),
    }
}

/**
Stack changes:

- 1 pop
- 1 push
 */
fn exec_exp(stack: &mut Vec<Value>) {
    let val = stack.pop().unwrap();
    match val {
        Value::Number(value) => {
            stack.push(Value::Number(value.exp()));
        },
        value => panic!("Unsupported operation on {}", value),
    }
}

/**
Stack changes:

- 1 pop
- 1 push
 */
fn exec_exp2(stack: &mut Vec<Value>) {
    let val = stack.pop().unwrap();
    match val {
        Value::Number(value) => {
            stack.push(Value::Number(value.exp2()));
        },
        value => panic!("Unsupported operation on {}", value),
    }
}

/**
Stack changes:

- 1 pop
- 1 push
 */
fn exec_asin(stack: &mut Vec<Value>) {
    let val = stack.pop().unwrap();
    match val {
        Value::Number(value) => {
            stack.push(Value::Number(value.asin()));
        },
        value => panic!("Unsupported operation on {}", value),
    }
}

/**
Stack changes:

- 1 pop
- 1 push
 */
fn exec_acos(stack: &mut Vec<Value>) {
    let val = stack.pop().unwrap();
    match val {
        Value::Number(value) => {
            stack.push(Value::Number(value.acos()));
        },
        value => panic!("Unsupported operation on {}", value),
    }
}

/**
Stack changes:

- 1 pop
- 1 push
 */
fn exec_atan(stack: &mut Vec<Value>) {
    let val = stack.pop().unwrap();
    match val {
        Value::Number(value) => {
            stack.push(Value::Number(value.atan()));
        },
        value => panic!("Unsupported operation on {}", value),
    }
}

/**
Stack changes:

- 1 pop
- 1 push
 */
fn exec_acot(stack: &mut Vec<Value>) {
    let val = stack.pop().unwrap();
    match val {
        Value::Number(value) => {
            stack.push(Value::Number((1.0/value).atan()));
        },
        value => panic!("Unsupported operation on {}", value),
    }
}

/**
Stack changes:

- 2 pop
- 1 push
 */
fn exec_atan2(stack: &mut Vec<Value>) {
    let val2 = stack.pop().unwrap();
    let val1 = stack.pop().unwrap();
    match (val1, val2) {
        (Value::Number(lhs), Value::Number(rhs)) => {
            stack.push(Value::Number(lhs.atan2(rhs)));
        }
        (lhs,rhs) => panic!("Unsupported operations on {} and {}", lhs, rhs),
    }
}

/**
Stack changes:

- No change
 */
fn exec_print(stack: &mut Vec<Value>) {
    for (i, elem) in stack.iter().rev().enumerate() {
        println!("{}: {}", !(i as isize), elem);
    }
}

fn exec_identifier(stack: &mut Vec<Value>, identifier: &str) {
    match HANDLERS.get(identifier) {
        Some(fun) => fun(stack),
        None => panic!("Undefined operator: {identifier}"),
    }
}

fn exec_expression(expr: &str) {
    let mut stack = Vec::<Value>::new();
    exec(&mut stack, expr);
}

fn exec(stack: &mut Vec<Value>, expr: &str) {
    for tok in expr.split_whitespace() {
        let num = tok.parse::<f64>();
        match num {
            Ok(number) => stack.push(Value::Number(number)),
            Err(_) => exec_identifier(stack, tok)
        }
        // println!("\"{tok}\": {:?}", stack);
    }
}

use rustyline::error::ReadlineError;
fn interactive() -> Result<bool, ReadlineError> {
    use rustyline::{DefaultEditor};
    
    let mut rl = DefaultEditor::new().unwrap();
    let mut stack = Vec::<Value>::new();

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
