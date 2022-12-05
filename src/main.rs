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
        Expr {
            lit: None,
            left: None,
            right: None,
        }
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
fn math_lexer(math_expr: String) -> Result<Vec<Token>, &'static str> {
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
                token += &math_expr_bytes[idx].to_string();
                idx += 1;
            }
            tokens.push(Token::Number(
                token.parse::<i64>().expect("a number that fits with i64"),
            ));
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
        } else if math_expr_bytes[idx] != ' ' as u8 {
            return Err("Unknown token");
        }
        idx += 1;
    }
    Ok(tokens)
}
fn get_precedence(op: &Ops) {
    match op {
        Ops::Add => 1,
        Ops::Subtract => 1,
        Ops::Multiply => 2,
        Ops::Divide => 2,
        Ops::Exponent => 3,
    };
}
/*
 * BINARYEXPR -> OPENPARENTH BINARYEXPR CLOSEPARENTH | BINARYEXPR OPERATOR BINARYEXPR | NUMBER
 * OPERATOR -> * | - | ^ | /  */
fn math_parse(
    tokens: &Vec<Token>,
    start: &mut usize,
    current: &mut usize,
    expr: &mut Expr,
) -> Result<(), &'static str> {
    if *current >= tokens.len() {
        expr.lit = None;
        return Ok(());
    }
    match &tokens[*current] {
        Token::OpenParenth => {
            *current += 1;
            *start = *current;
            let mut local_e = Expr::new();
            math_parse(tokens, start, current, &mut local_e)?;
            *expr = local_e.clone();
            if *current >= tokens.len() {
                return Err("Unclosed opening parenthesis");
            }
            match &tokens[*current] {
                Token::CloseParenth => {},
                _ => return Err("Unclosed opening parenthesis")
            }
        }
        Token::Number(num) => {
            expr.lit = Some(Literal::Number(*num));
            *current += 1;
            if *current >= tokens.len() {
                return Ok(());
            }
            match &tokens[*current] {
                Token::Operator(op) => {
                    *current += 1;
                    if *current >= tokens.len() {
                        return Err("Expected Open Parenthesis or Number.");
                    }
                    match &tokens[*current] {
                        Token::Number(right_num) => {
                            *current += 1;
                            let mut right_e = Expr::new();
                            math_parse(tokens, start, current, &mut right_e)?;
                            match right_e.lit {
                                Some(Literal::Op(ref right_op)) => {
                                    let first_op_precedence = get_precedence(op);
                                    let second_op_precedence = get_precedence(right_op);
                                    if first_op_precedence < second_op_precedence {
                                        right_e.left = Some(Box::new(Expr {
                                            lit: Some(Literal::Number(*right_num)),
                                            left: None,
                                            right: None,
                                        }));
                                        *expr = Expr {
                                            lit: Some(Literal::Op(op.clone())),
                                            left: Some(Box::new(right_e.clone())),
                                            right: Some(Box::new(expr.clone())),
                                        };
                                    } else if second_op_precedence <= first_op_precedence {
                                        *expr = right_e.clone();
                                        expr.left = Some(Box::new(Expr {
                                            lit: Some(Literal::Op(op.clone())),
                                            left: None,
                                            right: None,
                                        }));
                                    }
                                }
                                None => {
                                    return Ok(());
                                },
                                _ => {
                                    return Err("expected operator after number.");
                                }
                            }
                        }
                        Token::OpenParenth => {
                            *current += 1;
                            let mut right_e = Expr::new();
                            math_parse(tokens, start, current, &mut right_e)?;
                            match right_e.lit {
                                Some(_) => {
                                    expr.left = Some(Box::new(right_e));
                                    expr.lit = Some(Literal::Op(op.clone()));
                                },
                                _ => {
                                    return Err("Expected closing parenth or something.");
                                }
                            }
                        }
                        _ => println!("Expected Open Parenthesis or Number."),
                    }
                    *current += 1;
                }
                _ => {
                    return Err("expected operator.");
                }
            }
            *current += 1;
        }
        Token::CloseParenth => {
            return Ok(());
        }
        Token::Operator(op) => {
            match &tokens[*start] {
                Token::Number(_) => {
                },
                _ => return Err("operator with no operands?")
            }
            *current += 1;
            let mut right_e = Expr::new();
            math_parse(tokens, start, current, &mut right_e)?;
            expr.lit = Some(Literal::Op(op.clone()));
            expr.right = Some(Box::new(right_e.clone()));
        }
    }
    Ok(())
}
fn traverse_expr_tree(expr: &Expr) -> i64 {
    match &expr.lit {
        Some(Literal::Number(num)) => *num,
        Some(Literal::Op(op)) => {
            match op {
                Ops::Add => {
                    match &expr.left {
                        Some(l) => {
                            match &expr.right {
                                Some(r) => {
                                    traverse_expr_tree(&l) + traverse_expr_tree(&r)
                                },
                                None => traverse_expr_tree(&l),
                            }
                        },
                        None => {
                            match &expr.right {
                                Some(r) => {
                                    traverse_expr_tree(&r)
                                },
                                None => 0,
                            }
                        }
                    }
                },
                Ops::Subtract => {
                    match &expr.left {
                        Some(l) => {
                            match &expr.right {
                                Some(r) => {
                                    traverse_expr_tree(&l) - traverse_expr_tree(&r)
                                },
                                None => -traverse_expr_tree(&l),
                            }
                        },
                        None => {
                            match &expr.right {
                                Some(r) => {
                                    -traverse_expr_tree(&r)
                                },
                                None => 0,
                            }
                        }
                    }

                },
                Ops::Multiply => {
                    match &expr.left {
                        Some(l) => {
                            match &expr.right {
                                Some(r) => {
                                    traverse_expr_tree(&l) * traverse_expr_tree(&r)
                                },
                                None => traverse_expr_tree(&l),
                            }
                        },
                        None => {
                            match &expr.right {
                                Some(r) => {
                                    traverse_expr_tree(&r)
                                },
                                None => 0,
                            }
                        }
                    }

                },
                Ops::Divide => {
                    match &expr.left {
                        Some(l) => {
                            match &expr.right {
                                Some(r) => {
                                    traverse_expr_tree(&l) / traverse_expr_tree(&r)
                                },
                                None => traverse_expr_tree(&l),
                            }
                        },
                        None => {
                            match &expr.right {
                                Some(r) => {
                                    traverse_expr_tree(&r)
                                },
                                None => 0,
                            }
                        }
                    }

                },
                Ops::Exponent => {
                    match &expr.left {
                        Some(l) => {
                            match &expr.right {
                                Some(r) => {
                                    traverse_expr_tree(&l).pow(traverse_expr_tree(&r).try_into().unwrap())
                                },
                                None => traverse_expr_tree(&l),
                            }
                        },
                        None => {
                            match &expr.right {
                                Some(r) => {
                                    traverse_expr_tree(&r)
                                },
                                None => 0,
                            }
                        }
                    }

                },
            }
        },
        None => 0
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
fn main() -> Result<(), &'static str> {
    let args = env::args();
    if args.len() != 2 {
        return Err("Please offer only one CLI argument which should be a math expression");
    }
    let args_vec = args.collect::<Vec<String>>();
    let tokens = math_lexer(String::from(&args_vec[1]))?;
    let mut start = 0;
    let mut current = 0;
    let mut expr = Expr::new();
    math_parse(&tokens, &mut start, &mut current, &mut expr)?;
    println!("{}", traverse_expr_tree(&expr));
    Ok(())
}
