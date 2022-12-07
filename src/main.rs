use std::io::{self, Write};
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
                Ops::Exponent => Literal::Op(Ops::Exponent),
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
            Ops::Exponent => Ops::Exponent,
            Ops::Divide => Ops::Divide,
        }
    }
}
fn math_lexer(math_expr: &String) -> Result<Vec<Token>, &'static str> {
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
fn get_precedence(op: &Ops) -> u8 {
    match op {
        Ops::Add => 1,
        Ops::Subtract => 1,
        Ops::Multiply => 2,
        Ops::Divide => 2,
        Ops::Exponent => 3,
    }
}
/*
 * BINARYEXPR -> OPENPARENTH BINARYEXPR CLOSEPARENTH | BINARYEXPR OPERATOR BINARYEXPR | NUMBER
 * OPERATOR -> * | - | ^ | /  */
fn math_parse(
    tokens: &Vec<Token>,
    mut start: usize,
    mut current: usize,
    expr: &mut Expr,
) -> Result<(), &'static str> {
    if current >= tokens.len() {
        return Ok(());
    }
    match &tokens[current] {
        Token::OpenParenth => {
            start = current;
            let mut local_e = Expr::new();
            math_parse(tokens, start, current + 1, &mut local_e)?;
            *expr = local_e.clone();
        }
        Token::Number(num) => {
            expr.lit = Some(Literal::Number(*num));
            current += 1;
            if current >= tokens.len() {
                return Ok(());
            }
            match &tokens[current] {
                Token::Operator(op) => {
                    *expr = Expr {
                        lit: Some(Literal::Op(op.clone())),
                        left: Some(Box::new(expr.clone())),
                        right: None,
                    };
                    let mut right_e = Expr::new();
                    math_parse(tokens, start, current + 1, &mut right_e)?;
                    current += 1;
                    if current >= tokens.len() {
                        return Err("expected right operand.");
                    }
                    match &tokens[current] {
                        Token::OpenParenth => {
                            expr.right = Some(Box::new(right_e.clone()));
                        }
                        _ => {
                            match right_e.lit {
                                Some(Literal::Op(_)) => {
                                    // loop until we find the right place to put expr. Like
                                    // a BST (Binary Search Tree)
                                    let first_op_precedence = get_precedence(op);
                                    let mut top_right = Box::new(right_e.clone());
                                    let mut current_expr = &mut top_right;
                                    loop {
                                        match current_expr.lit {
                                            Some(Literal::Op(ref current_op)) => {
                                                let second_op_precedence =
                                                    get_precedence(&current_op);
                                                if first_op_precedence < second_op_precedence {
                                                    expr.right = Some(current_expr.clone());
                                                    *current_expr = Box::new(expr.clone());
                                                    *expr = *top_right.clone();
                                                    break;
                                                } else if second_op_precedence
                                                    <= first_op_precedence
                                                {
                                                    match current_expr.left {
                                                        Some(ref mut left) => {
                                                            current_expr = left;
                                                        }
                                                        None => {
                                                            expr.right = current_expr.left.clone();
                                                            *current_expr = Box::new(expr.clone());
                                                            *expr = *top_right.clone();
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                            Some(Literal::Number(current_num)) => {
                                                expr.right = Some(Box::new(Expr {
                                                    lit: Some(Literal::Number(current_num)),
                                                    left: None,
                                                    right: None,
                                                }));
                                                *current_expr = Box::new(expr.clone());
                                                *expr = *top_right.clone();
                                                break;
                                            }
                                            None => {
                                                unreachable!()
                                            }
                                        }
                                    }
                                }
                                Some(Literal::Number(right_num)) => {
                                    expr.right = Some(Box::new(Expr {
                                        lit: Some(Literal::Number(right_num)),
                                        left: None,
                                        right: None,
                                    }));
                                }
                                _ => {
                                    return Err("expected right operand.");
                                }
                            }
                        }
                    }
                }
                Token::CloseParenth => {}
                _ => {
                    return Err("expected operator.");
                }
            }
        }
        Token::CloseParenth => match &tokens[start] {
            Token::OpenParenth => {}
            _ => return Err("Unexpected ')'"),
        },
        Token::Operator(_) => {
            return Err("Expected left operand.");
        }
    }
    Ok(())
}
fn traverse_expr_tree(expr: &Expr) -> i64 {
    match &expr.lit {
        Some(Literal::Number(num)) => *num,
        Some(Literal::Op(op)) => match op {
            Ops::Add => match &expr.left {
                Some(l) => match &expr.right {
                    Some(r) => traverse_expr_tree(&l) + traverse_expr_tree(&r),
                    None => traverse_expr_tree(&l),
                },
                None => match &expr.right {
                    Some(r) => traverse_expr_tree(&r),
                    None => 0,
                },
            },
            Ops::Subtract => match &expr.left {
                Some(l) => match &expr.right {
                    Some(r) => traverse_expr_tree(&l) - traverse_expr_tree(&r),
                    None => -traverse_expr_tree(&l),
                },
                None => match &expr.right {
                    Some(r) => -traverse_expr_tree(&r),
                    None => 0,
                },
            },
            Ops::Multiply => match &expr.left {
                Some(l) => match &expr.right {
                    Some(r) => traverse_expr_tree(&l) * traverse_expr_tree(&r),
                    None => traverse_expr_tree(&l),
                },
                None => match &expr.right {
                    Some(r) => traverse_expr_tree(&r),
                    None => 0,
                },
            },
            Ops::Divide => match &expr.left {
                Some(l) => match &expr.right {
                    Some(r) => traverse_expr_tree(&l) / traverse_expr_tree(&r),
                    None => traverse_expr_tree(&l),
                },
                None => match &expr.right {
                    Some(r) => traverse_expr_tree(&r),
                    None => 0,
                },
            },
            Ops::Exponent => match &expr.left {
                Some(l) => match &expr.right {
                    Some(r) => {
                        traverse_expr_tree(&l).pow(traverse_expr_tree(&r).try_into().unwrap())
                    }
                    None => traverse_expr_tree(&l),
                },
                None => match &expr.right {
                    Some(r) => traverse_expr_tree(&r),
                    None => 0,
                },
            },
        },
        None => 0,
    }
}
fn print_tokens(tokens: &Vec<Token>) {
    for t in tokens {
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
fn main() -> Result<(), &'static str> {
    let mut expr_str = String::new();
    loop {
        print!("\r>");
        match io::stdout().flush() {
            Ok(_) => {}
            Err(_) => break,
        }
        match io::stdin().read_line(&mut expr_str) {
            Ok(_) => {
                let trimmed = expr_str.trim().to_string();
                println!("Calculating: {}", trimmed);
                let tokens = math_lexer(&trimmed)?;
                let mut expr = Expr::new();
                math_parse(&tokens, 0, 0, &mut expr)?;
                println!("{}", traverse_expr_tree(&expr));
                expr_str.clear();
            }
            Err(_) => break,
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn five_pow_five_times_four_minus_three() -> Result<(), &'static str> {
        let tokens = math_lexer(&"5 ^ 5 * 4 - 3".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr) == 5i64.pow(5) * 4 - 3);
        Ok(())
    }
    #[test]
    fn four_plus_four() -> Result<(), &'static str> {
        let tokens = math_lexer(&"4 + 4".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr) == 4 + 4);
        Ok(())
    }
    #[test]
    fn five_times_four_minus_three() -> Result<(), &'static str> {
        let tokens = math_lexer(&"5 * 4 - 3".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr) == 5 * 4 - 3);
        Ok(())
    }
    #[test]
    fn four_minus_four_times_three() -> Result<(), &'static str> {
        let tokens = math_lexer(&"4 - 4 * 3".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr) == 4 - 4 * 3);
        Ok(())
    }
    #[test]
    fn three_divide_three() -> Result<(), &'static str> {
        let tokens = math_lexer(&"3 / 3".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr) == 3 / 3);
        Ok(())
    }
    #[test]
    fn six_power_two_minus_four_times_three() -> Result<(), &'static str> {
        let tokens = math_lexer(&"6 ^ 2 - 4 * 3".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr) == 6i64.pow(2) - 4 * 3);
        Ok(())
    }
    #[test]
    fn two_power_two_power_two() -> Result<(), &'static str> {
        let tokens = math_lexer(&"2 ^ 2 ^ 2".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr) == 2i64.pow(2).pow(2));
        Ok(())
    }
    #[test]
    fn two_power_two_divide_two() -> Result<(), &'static str> {
        let tokens = math_lexer(&"2 ^ 2 / 2".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr) == 2i64.pow(2) / 2);
        Ok(())
    }
    #[test]
    fn three_minus_four_times_five_add_3_power_two() -> Result<(), &'static str> {
        let tokens = math_lexer(&"3 - 4 * 5 + 3 ^ 2".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr) == 3 - 4 * 5 + 3i64.pow(2));
        Ok(())
    }
}
