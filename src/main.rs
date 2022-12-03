use std::env;
enum Ops {
    Add,
    Multiply,
    Exponent,
    Subtract,
    Divide,
}
enum Expr {
    Op(Ops),
    Right(Box<Expr>),
    Left(Box<Expr>),
}
enum Token {
    Number(i64),
    Operator(Ops),
    OpenParenth,
    CloseParenth,
}
fn math_lexer(math_expr: &str) -> Result<Vec<Token>, &str> {
    let math_expr_bytes = math_expr.as_bytes();
    let mut tokens = Vec::new();
    let mut token = String::new();
    let mut idx: usize = 0;
    loop {
        if idx >= math_expr_bytes.len() {
            break;
        } else if token.len() == 0 && math_expr_bytes[idx] == '0' as u8 {
            return Err("cannot have leading zeroes for numbers");
        } else if math_expr_bytes[idx] >= '0' as u8 && math_expr_bytes[idx] <= '9' as u8 {
            while idx < math_expr_bytes.len()
                && math_expr_bytes[idx] >= '0' as u8
                && math_expr_bytes[idx] <= '9' as u8
            {
                token += &(math_expr_bytes[idx] as char).to_string();
                idx += 1;
            }
            tokens.push(Token::Number(token.parse::<i64>().unwrap()));
            token.clear();
            continue;
        } else if math_expr_bytes[idx] == '(' as u8 {
            tokens.push(Token::OpenParenth);
        } else if math_expr_bytes[idx] == ')' as u8 {
            tokens.push(Token::CloseParenth);
        } else if math_expr_bytes[idx] == '*' as u8
            || math_expr_bytes[idx] == '/' as u8
            || math_expr_bytes[idx] == '^' as u8
            || math_expr_bytes[idx] == '+' as u8
            || math_expr_bytes[idx] == '-' as u8
        {
            let c = math_expr_bytes[idx] as char;
            if c == '*' {
                tokens.push(Token::Operator(Ops::Multiply));
            } else if c == '/' {
                tokens.push(Token::Operator(Ops::Divide));
            } else if c == '^' {
                tokens.push(Token::Operator(Ops::Exponent));
            } else if c == '+' {
                tokens.push(Token::Operator(Ops::Add));
            } else if c == '-' {
                tokens.push(Token::Operator(Ops::Subtract));
            }
        }
        idx += 1;
    }
    Ok(tokens)
}
fn math_parse(tokens: Vec<Token>) {}
fn main() {
    let args = env::args();
    if args.len() != 2 {
        println!("Please offer only one CLI argument which should be a math expression");
        return;
    }
    let args_vec = args.collect::<Vec<String>>();
    let tokens = math_lexer(&args_vec[1]);
    match tokens {
        Ok(vec_tokens) => {
            for t in vec_tokens {
                match t {
                    Token::Operator(op) => {
                        use Ops::*;
                        match op {
                            Divide => println!("Divide"),
                            Add => println!("Add"),
                            Subtract => println!("Subtract"),
                            Exponent => println!("Exponent"),
                            Multiply => println!("Multiply"),
                        }
                    }
                    Token::Number(num) => {
                        println!("{num}");
                    }
                    Token::CloseParenth => println!("CloseParenth"),
                    Token::OpenParenth => println!("OpenParenth"),
                }
            }
        }
        Err(_) => {}
    }
}
