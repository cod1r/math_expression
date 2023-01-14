use std::{
    convert::TryInto,
    io::{self, Write},
};
const TOP_PREC: u8 = 4;
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
    precedence: u8,
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
            precedence: 0,
        }
    }
}
impl Clone for Expr {
    fn clone(&self) -> Self {
        Expr {
            lit: self.lit.clone(),
            left: self.left.clone(),
            right: self.right.clone(),
            precedence: self.precedence,
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
        } else if math_expr_bytes[idx] >= b'0' && math_expr_bytes[idx] <= b'9' {
            while idx < math_expr_bytes.len()
                && math_expr_bytes[idx] >= b'0'
                && math_expr_bytes[idx] <= b'9'
            {
                token += &(math_expr_bytes[idx] as char).to_string();
                idx += 1;
            }
            tokens.push(Token::Number(
                token.parse::<i64>().expect("a number that fits with i64"),
            ));
            if token.len() > 1 && token.as_bytes()[0] == b'0' {
                return Err("numbers cannot have leading zeroes");
            }
            token.clear();
            continue;
        } else if math_expr_bytes[idx] == b'(' {
            tokens.push(Token::OpenParenth);
        } else if math_expr_bytes[idx] == b')' {
            tokens.push(Token::CloseParenth);
        } else if math_expr_bytes[idx] == b'*'
            || math_expr_bytes[idx] == b'/'
            || math_expr_bytes[idx] == b'^'
            || math_expr_bytes[idx] == b'+'
            || math_expr_bytes[idx] == b'-'
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
        } else if math_expr_bytes[idx] != b' ' && math_expr_bytes[idx] != b'\t' {
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
 *
 * Old grammar:
 * BINARYEXPR -> OPENPARENTH BINARYEXPR CLOSEPARENTH | BINARYEXPR OPERATOR BINARYEXPR | NUMBER
 * OPERATOR -> * | - | ^ | /
   Some more advice
   - Expr could be an enum
   - manual Clone impls could be derived
   - 'x' as u8 == b'x'
   - if-else if chains can be match blocks
   - math_lexer should take &[u8] for the input
   - you should really assign math_expr_bytes[idx] to a variable since you write it tens of times
   - as char can be char::from
   - your grammar definition is left recursive
   - to be honest I didn't read the parser too much. it seems really complicated
   - traverse_expr_tree looks fine except for the fact that your definition of Expr makes it inelegant
   - your print method could be replaced with a Debug impl (derived) on Token
   - you terminate the whole repl when an expr fails to parse. I think it would be better to print an error but continue reading.
   - on the other hand, just unwrap the IO results. no need to wory about them and breaking from the loop is kinda confusing honestly especially since you don't print the error

   Some advice I got from discord on defining grammar
       expr0 = expr1 (binary_operator expr1)*
       expr1 = unary_operator* expr2
       expr2 = OPEN_PAREN expr0 CLOSE_PAREN | NUMBER
       // define unary_ and binary_operator as necessary


 * OLD
   New grammar:
   uses Augmented Backus Naur Form
   EXPR -> NUMBER OPERATOR EXPR | OPENPARENTH EXPR CLOSEPARENTH *(OPERATOR EXPR) | NUMBER
   OPERATOR -> + | - | * | / | ^
   NUMBER -> INTEGER THAT CAN FIT INTO i64
   THIS GRAMMAR IS BAD BC IT NEVER CONSIDERED SIGNED NUMBERS (EX: -1 or +1)

**/
fn reconcile_trees(left: &mut Expr, right: &mut Expr) {
    // loop until we find the right place to put expr. Like
    // a BST (Binary Search Tree)
    let first_op_precedence = left.precedence;
    let mut top_right = Box::new(right.clone());
    let mut current_expr = &mut top_right;
    loop {
        match current_expr.lit {
            Some(Literal::Op(_)) => {
                let second_op_precedence = current_expr.precedence;
                if first_op_precedence < second_op_precedence {
                    left.right = Some(current_expr.clone());
                    *current_expr = Box::new(left.clone());
                    *left = *top_right.clone();
                    break;
                } else if second_op_precedence <= first_op_precedence {
                    match current_expr.left {
                        Some(ref mut left_e) => {
                            current_expr = left_e;
                        }
                        None => {
                            current_expr.left = Some(Box::new(left.clone()));
                            *left = *top_right.clone();
                            break;
                        }
                    }
                }
            }
            Some(Literal::Number(current_num)) => {
                left.right = Some(Box::new(Expr {
                    lit: Some(Literal::Number(current_num)),
                    left: None,
                    right: None,
                    precedence: 0,
                }));
                *current_expr = Box::new(left.clone());
                *left = *top_right.clone();
                break;
            }
            None => {
                unreachable!()
            }
        }
    }
}
/*
  New new grammer:
   expr1 -> number (op expr1)* | open_parenth expr1 close_parenth *(expr1) | expr2
   expr2 -> (unary_op number)* | number
   op -> unary_op | * | / | ^
   unary_op -> - | +
*/
fn math_parse(
    tokens: &Vec<Token>,
    start: usize,
    mut current: usize,
    expr: &mut Expr,
) -> Result<(), &'static str> {
    if current >= tokens.len() {
        return Ok(());
    }
    match &tokens[current] {
        Token::OpenParenth => {
            let mut local_e = Expr::new();
            math_parse(tokens, current + 1, current + 1, &mut local_e)?;
            *expr = local_e.clone();
            let mut parentheses = Vec::new();
            while current < tokens.len() {
                match &tokens[current] {
                    Token::Number(_) | Token::Operator(_) => {}
                    Token::OpenParenth => {
                        parentheses.push(&tokens[current]);
                    }
                    Token::CloseParenth => {
                        if !parentheses.is_empty() {
                            parentheses.pop();
                        } else {
                            return Err("Unexpected close parenthesis.");
                        }
                    }
                }
                current += 1;
                if parentheses.is_empty() {
                    break;
                }
            }
            if !parentheses.is_empty() {
                return Err("Expected ')'");
            }
            if current >= tokens.len() {
                expr.precedence = TOP_PREC;
                return Ok(());
            }
            match &tokens[current] {
                Token::Operator(op) => {
                    *expr = Expr {
                        lit: Some(Literal::Op(op.clone())),
                        left: Some(Box::new(expr.clone())),
                        right: None,
                        precedence: get_precedence(op),
                    };
                    current += 1;
                    if current >= tokens.len() {
                        return Err("expected right operand.");
                    }
                    let mut right_e = Expr::new();
                    math_parse(tokens, current, current, &mut right_e)?;
                    reconcile_trees(expr, &mut right_e);
                }
                Token::CloseParenth => {
                    if start == 0 {
                        return Err("Unexpected ')'");
                    }
                }
                _ => return Err("expected operator."),
            }
            expr.precedence = TOP_PREC;
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
                        precedence: get_precedence(op),
                    };
                    current += 1;
                    if current >= tokens.len() {
                        return Err("expected right operand.");
                    }
                    let mut right_e = Expr::new();
                    math_parse(tokens, start, current, &mut right_e)?;
                    match &right_e.lit {
                        Some(Literal::Op(right_op)) => {
                            match right_op {
                                Ops::Multiply | Ops::Divide => {
                                    match right_e.left {
                                        Some(_) => {}
                                        None => return Err(
                                            "Multiplication or Division is not a unary operator.",
                                        ),
                                    }
                                }
                                _ => {}
                            }
                            reconcile_trees(expr, &mut right_e);
                        }
                        Some(Literal::Number(right_num)) => {
                            expr.right = Some(Box::new(Expr {
                                lit: Some(Literal::Number(*right_num)),
                                left: None,
                                right: None,
                                precedence: 0,
                            }));
                        }
                        _ => {
                            return Err("expected right operand.");
                        }
                    }
                }
                Token::CloseParenth => {}
                _ => {
                    return Err("expected operator.");
                }
            }
        }
        Token::CloseParenth => return Err("Unexpected ')'"),
        Token::Operator(op) => {
            expr.lit = Some(Literal::Op(op.clone()));
            current += 1;
            if current >= tokens.len() {
                return Err("expected right operand.");
            }
            match &tokens[current] {
                Token::Number(num) => {
                    expr.right = Some(Box::new(Expr {
                        lit: Some(Literal::Number(*num)),
                        left: None,
                        right: None,
                        precedence: 0,
                    }));
                    current += 1;
                    if current < tokens.len() {
                        match &tokens[current] {
                            Token::Operator(_) => {
                                let mut right_e = Expr::new();
                                math_parse(tokens, start, current, &mut right_e)?;
                                right_e.left = Some(Box::new(expr.clone()));
                                *expr = right_e.clone();
                            }
                            Token::CloseParenth => {}
                            _ => return Err("Expected operator."),
                        }
                    }
                }
                _ => {
                    if let Token::Operator(right_op) = &tokens[current] {
                        match right_op {
                            Ops::Multiply | Ops::Divide => {
                                return Err("Multiplication or Division is not a unary operator.")
                            }
                            _ => {}
                        }
                    }
                    let mut right_e = Expr::new();
                    math_parse(tokens, start, current, &mut right_e)?;
                    expr.right = Some(Box::new(right_e.clone()));
                }
            }
            expr.precedence = TOP_PREC;
        }
    }
    Ok(())
}
fn traverse_expr_tree(expr: &Expr) -> Result<i64, &'static str> {
    match &expr.lit {
        Some(Literal::Number(num)) => Ok(*num),
        Some(Literal::Op(op)) => match op {
            Ops::Add => match &expr.left {
                Some(l) => match &expr.right {
                    Some(r) => Ok(traverse_expr_tree(l)? + traverse_expr_tree(r)?),
                    None => traverse_expr_tree(l),
                },
                None => match &expr.right {
                    Some(r) => traverse_expr_tree(r),
                    None => Ok(0),
                },
            },
            Ops::Subtract => match &expr.left {
                Some(l) => match &expr.right {
                    Some(r) => Ok(traverse_expr_tree(l)? - traverse_expr_tree(r)?),
                    None => Ok(-traverse_expr_tree(l)?),
                },
                None => match &expr.right {
                    Some(r) => Ok(-traverse_expr_tree(r)?),
                    None => Ok(0),
                },
            },
            Ops::Multiply => match &expr.left {
                Some(l) => match &expr.right {
                    Some(r) => Ok(traverse_expr_tree(l)? * traverse_expr_tree(r)?),
                    None => traverse_expr_tree(l),
                },
                None => match &expr.right {
                    Some(r) => traverse_expr_tree(r),
                    None => Ok(0),
                },
            },
            Ops::Divide => match &expr.left {
                Some(l) => match &expr.right {
                    Some(r) => {
                        let right = traverse_expr_tree(r)?;
                        if right == 0 {
                            return Err("cannot divide by zero");
                        }
                        Ok(traverse_expr_tree(l)? / right)
                    },
                    None => traverse_expr_tree(l),
                },
                None => match &expr.right {
                    Some(r) => traverse_expr_tree(r),
                    None => Ok(0),
                },
            },
            Ops::Exponent => match &expr.left {
                Some(l) => match &expr.right {
                    Some(r) => {
                        let power = traverse_expr_tree(r)?;
                        if power < 0 {
                            return Err("cannot do power to negative numbers");
                        }
                        Ok(traverse_expr_tree(l)?.pow(power.try_into().unwrap()))
                    }
                    None => traverse_expr_tree(l),
                },
                None => match &expr.right {
                    Some(r) => traverse_expr_tree(r),
                    None => Ok(0),
                },
            },
        },
        None => Ok(0),
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
                let tokens_res = math_lexer(&trimmed);
                match tokens_res {
                    Ok(tokens) => {
                        let mut expr = Expr::new();
                        let parse_res = math_parse(&tokens, 0, 0, &mut expr);
                        let res = traverse_expr_tree(&expr);
                        match parse_res {
                            Ok(_) => match res {
                                Ok(v) => println!("{}", v),
                                Err(s) => println!("{}", s),
                            },
                            Err(s) => println!("{}", s),
                        }
                    }
                    Err(s) => println!("{}", s),
                }
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
        assert!(traverse_expr_tree(&expr)? == 5i64.pow(5) * 4 - 3);
        Ok(())
    }
    #[test]
    fn four_plus_four() -> Result<(), &'static str> {
        let tokens = math_lexer(&"4 + 4".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr)? == 4 + 4);
        Ok(())
    }
    #[test]
    fn five_times_four_minus_three() -> Result<(), &'static str> {
        let tokens = math_lexer(&"5 * 4 - 3".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr)? == 5 * 4 - 3);
        Ok(())
    }
    #[test]
    fn four_minus_four_times_three() -> Result<(), &'static str> {
        let tokens = math_lexer(&"4 - 4 * 3".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr)? == 4 - 4 * 3);
        Ok(())
    }
    #[test]
    fn three_divide_three() -> Result<(), &'static str> {
        let tokens = math_lexer(&"3 / 3".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr)? == 3 / 3);
        Ok(())
    }
    #[test]
    fn six_power_two_minus_four_times_three() -> Result<(), &'static str> {
        let tokens = math_lexer(&"6 ^ 2 - 4 * 3".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr)? == 6i64.pow(2) - 4 * 3);
        Ok(())
    }
    #[test]
    fn two_power_two_power_two() -> Result<(), &'static str> {
        let tokens = math_lexer(&"2 ^ 2 ^ 2".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr)? == 2i64.pow(2).pow(2));
        Ok(())
    }
    #[test]
    fn two_power_two_divide_two() -> Result<(), &'static str> {
        let tokens = math_lexer(&"2 ^ 2 / 2".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr)? == 2i64.pow(2) / 2);
        Ok(())
    }
    #[test]
    fn three_minus_four_times_five_add_3_power_two() -> Result<(), &'static str> {
        let tokens = math_lexer(&"3 - 4 * 5 + 3 ^ 2".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr)? == 3 - 4 * 5 + 3i64.pow(2));
        Ok(())
    }
    #[test]
    fn p_four_minus_three_p_times_p_three_minus_5_p() -> Result<(), &'static str> {
        let tokens = math_lexer(&"(4 - 3 ) * (3 - 5)".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr)? == (4 - 3) * (3 - 5));
        Ok(())
    }
    #[test]
    fn p_p_p_one_plus_one_p_p_p() -> Result<(), &'static str> {
        let tokens = math_lexer(&"(((1 + 1)))".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr)? == (1 + 1));
        Ok(())
    }
    #[test]
    fn p_p_one_plus_three_p_times_p_four_plus_five_p_p() -> Result<(), &'static str> {
        let tokens = math_lexer(&"((1 + 3) * (4 + 5))".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr)? == ((1 + 3) * (4 + 5)));
        Ok(())
    }
    #[test]
    fn unmatched_parentheses() -> Result<(), &'static str> {
        let tokens = math_lexer(&"((1) + 1".to_string())?;
        let mut expr = Expr::new();
        let res = math_parse(&tokens, 0, 0, &mut expr);
        match res {
            Err(_) => {}
            Ok(_) => return Err("Unmatched parentheses was not caught"),
        }
        Ok(())
    }
    #[test]
    fn extra_closing_parentheses() -> Result<(), &'static str> {
        let tokens = math_lexer(&"(1 + 1))))))))))))))))))))))))))))))))))))))".to_string())?;
        let mut expr = Expr::new();
        let res = math_parse(&tokens, 0, 0, &mut expr);
        match res {
            Err(_) => {}
            Ok(_) => return Err("Unmatched parentheses was not caught"),
        }
        Ok(())
    }
    #[test]
    fn unknown_token() -> Result<(), &'static str> {
        let tokens = math_lexer(&"1 _ 1".to_string());
        match tokens {
            Err(_) => {}
            Ok(_) => return Err("Unknown token wasn't caught"),
        }
        Ok(())
    }
    #[test]
    fn leading_zeroes() -> Result<(), &'static str> {
        let tokens = math_lexer(&"01 + 1".to_string());
        match tokens {
            Err(_) => {}
            Ok(_) => return Err("leading zero wasn't caught"),
        }
        Ok(())
    }
    #[test]
    fn neg_one_minus_one() -> Result<(), &'static str> {
        let tokens = math_lexer(&"-1-1".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr)? == -1 - 1);
        Ok(())
    }
    #[test]
    fn one_minus_neg_one() -> Result<(), &'static str> {
        let tokens = math_lexer(&"1--1".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr)? == 1 - -1);
        Ok(())
    }
    #[test]
    fn incorrect_unary_op() -> Result<(), &'static str> {
        let tokens = math_lexer(&"1-*1".to_string())?;
        let mut expr = Expr::new();
        let res = math_parse(&tokens, 0, 0, &mut expr);
        match res {
            Err(_) => {}
            Ok(_) => return Err("incorrect unary op was not caught"),
        }
        Ok(())
    }
    #[test]
    fn one_times_neg_neg_one() -> Result<(), &'static str> {
        let tokens = math_lexer(&"1*--1".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr)? == 1 * --1);
        Ok(())
    }
    #[test]
    fn five_times_neg_one_times_p_one_plus_four_p() -> Result<(), &'static str> {
        let tokens = math_lexer(&"5 * -1 * (1 + 4)".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr)? == 5 * -1 * (1 + 4));
        Ok(())
    }
    #[test]
    fn one_times_neg_neg_neg_neg_neg_one() -> Result<(), &'static str> {
        let tokens = math_lexer(&"1 * -----1".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr)? == 1 * -----1);
        Ok(())
    }
    #[test]
    fn one_times_pos_one() -> Result<(), &'static str> {
        let tokens = math_lexer(&"1 * +1".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr)? == 1 * (0 + 1));
        Ok(())
    }
    #[test]
    fn one_times_times_neg_one() -> Result<(), &'static str> {
        let tokens = math_lexer(&"1**-1".to_string())?;
        let mut expr = Expr::new();
        let res = math_parse(&tokens, 0, 0, &mut expr);
        match res {
            Err(_) => {}
            Ok(_) => return Err("incorrect unary op was not caught"),
        }
        Ok(())
    }
    #[test]
    fn neg_one_times_mult_one() -> Result<(), &'static str> {
        let tokens = math_lexer(&"-1 * *1".to_string())?;
        let mut expr = Expr::new();
        let res = math_parse(&tokens, 0, 0, &mut expr);
        match res {
            Err(_) => {}
            Ok(_) => return Err("incorrect unary op was not caught"),
        }
        Ok(())
    }
    #[test]
    fn neg_one_times_p_one_plus_four_time_four_p() -> Result<(), &'static str> {
        let tokens = math_lexer(&"-1 * ( 1 + 4 * 4)".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr)? == -1 * (1 + 4 * 4));
        Ok(())
    }
    #[test]
    fn expr2_in_expr1_times_expr2_in_expr1() -> Result<(), &'static str> {
        let tokens = math_lexer(&"(1 * -1) * (2 * -2)".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        assert!(traverse_expr_tree(&expr)? == (1 * -1) * (2 * -2));
        Ok(())
    }
    #[test]
    fn single_zero() -> Result<(), &'static str> {
        let tokens = math_lexer(&"1 + 0".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        let eval_res = traverse_expr_tree(&expr);
        match eval_res {
            Ok(_) => {},
            Err(_) => return Err("single zeroes should be allowed")
        }
        Ok(())
    }
    #[test]
    fn divide_by_zero() -> Result<(), &'static str> {
        let tokens = math_lexer(&"1 / 0".to_string())?;
        let mut expr = Expr::new();
        math_parse(&tokens, 0, 0, &mut expr)?;
        let eval_res = traverse_expr_tree(&expr);
        match eval_res {
            Ok(_) => return Err("divide by zero not caught"),
            Err(_) => {}
        }
        Ok(())
    }
}
