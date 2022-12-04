use std::env;
enum Ops {
    Add,
    Multiply,
    Exponent,
    Subtract,
    Divide,
}
enum Literal {
    Number(i64),
    Op(Ops),
}
struct Expr {
    lit: Option<Literal>,
    right: Option<Box<Expr>>,
    left: Option<Box<Expr>>,
}
enum Token {
    Number(i64),
    Operator(Ops),
    OpenParenth,
    CloseParenth,
}
impl Expr {
    fn new() -> Expr {
        Expr { lit: None, left: None, right: None }
    }
}
impl Clone for Expr {
    fn clone(&self) -> Self {
        Expr {
            lit: self.lit.clone(),
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }
}
impl Clone for Literal {
    fn clone(&self) -> Self {
        match self {
            Literal::Number(num) => Literal::Number(*num),
            Literal::Op(op) => match op {
                Ops::Add => Literal::Op(Ops::Add),
                Ops::Subtract => Literal::Op(Ops::Subtract),
                Ops::Multiply => Literal::Op(Ops::Multiply),
                Ops::Exponent => Literal::Op(Ops::Multiply),
                Ops::Divide => Literal::Op(Ops::Divide),
            },
        }
    }
}
impl Clone for Ops {
    fn clone(&self) -> Self {
        match self {
            Ops::Add => Ops::Add,
            Ops::Subtract => Ops::Subtract,
            Ops::Multiply => Ops::Multiply,
            Ops::Exponent => Ops::Multiply,
            Ops::Divide => Ops::Divide,
        }
    }
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
        } else {
            return Err("Unknown token");
        }
        idx += 1;
    }
    Ok(tokens)
}
/*
 * BINARYEXPR -> OPENPARENTH BINARYEXPR CLOSEPARENTH | BINARYEXPR OPERATOR BINARYEXPR | NUMBER
 * OPERATOR -> * | - | ^ | /  */
fn math_parse(tokens: &Vec<Token>, start: &mut usize, current: &mut usize, expr: &mut Expr) {
    if *current >= tokens.len() {
        return;
    }
    match &tokens[*current] {
        Token::OpenParenth => {
            *start = *current;
            let mut local_e = Expr::new();
            math_parse(tokens, start, current, expr);
        }
        Token::Number(num) => {
            let local_e = Expr {
                lit: Some(Literal::Number(*num)),
                left: None,
                right: None,
            };
            loop {
                if *current >= tokens.len() {
                    println!("expected operator.");
                    return;
                }
                match &tokens[*current] {
                    Token::Operator(op) => {
                        *current += 1;
                        let mut right_e = Expr::new();
                        math_parse(tokens, start, current, &mut right_e);
                        match right_e.lit {
                            Some(Literal::Number(_)) => {
                                *expr = Expr {
                                    lit: Some(Literal::Op(op.clone())),
                                    left: Some(Box::new(local_e.clone())),
                                    right: Some(Box::new(right_e.clone())),
                                };
                            }
                            Some(Literal::Op(_)) => {
                                *expr = Expr {
                                    lit: Some(Literal::Op(op.clone())),
                                    left: Some(Box::new(right_e.clone())),
                                    right: Some(Box::new(local_e.clone())),
                                };
                            }
                            None => {}
                        }
                    }
                    _ => {
                        println!("expected operator.");
                        return;
                    }
                }
                *current += 1;
            }
        }
        Token::CloseParenth => {}
        Token::Operator(op) => {}
    }
}
fn print_tokens(tokens: &Result<Vec<Token>, &str>) {
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
                    Token::Number(num) => println!("{num}"),
                    Token::CloseParenth => println!("CloseParenth"),
                    Token::OpenParenth => println!("OpenParenth"),
                }
            }
        }
        Err(_) => {}
    }
}
fn main() {
    let args = env::args();
    if args.len() != 2 {
        println!("Please offer only one CLI argument which should be a math expression");
        return;
    }
    let args_vec = args.collect::<Vec<String>>();
    let tokens = math_lexer(&args_vec[1]);
    print_tokens(&tokens);
}
