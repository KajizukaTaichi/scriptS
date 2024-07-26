use sila_transpiler_infrastructure::*;

fn main() {
    println!("Hello, world of the ScriptS !!");
    println!(
        "{}",
        transpile_python(parse_program(
            r#"
                if 1 == 1 {
                    print 3;
                }
        "#
            .to_string()
        ))
    );
}

fn parse_program(source: String) -> Block {
    let mut program: Block = vec![];
    for code in tokenize_program(source) {
        let code = code.trim().to_string();
        if code.starts_with("if") {
            let expr = parse_expr(
                code[2..code.find("{").expect("チノちゃん「うるさいですね...」")].to_string(),
            );
            let code_true = parse_program(
                code[code.find("{").expect("チノちゃん「うるさいですね...」")
                    ..code.find("}").expect("チノちゃん「うるさいですね...」")]
                    .to_string(),
            );
            program.push(Instruction::If(expr, code_true, None))
        }
        if code.starts_with("print") {
            let expr = parse_expr(code[5..code.len()].to_string());
            program.push(Instruction::Print(expr))
        }
    }
    program
}

fn tokenize_program(input: String) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_parentheses: usize = 0;
    let mut in_quote = false;

    for c in input.chars() {
        match c {
            '{' if !in_quote => {
                in_parentheses += 1;
                current_token.push(c);
            }
            '}' if !in_quote => {
                current_token.push(c);
                in_parentheses -= 1;
            }
            '"' => {
                if in_parentheses == 0 {
                    if in_quote {
                        in_quote = false;
                        current_token.push(c);
                    } else {
                        in_quote = true;
                        current_token.push(c);
                    }
                } else {
                    current_token.push(c);
                }
            }
            ';' => {
                if in_parentheses != 0 || in_quote {
                    current_token.push(c);
                } else {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                }
            }
            _ => {
                current_token.push(c);
            }
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}

fn parse_expr(source: String) -> Expr {
    let mut expr = vec![];
    for token in tokenize_expr(source) {
        let chars: Vec<char> = token.trim().chars().collect();
        if let Ok(i) = token.parse::<i64>() {
            expr.push(Expr::Literal(Type::Integer(i)))
        } else if let Ok(f) = token.parse::<f64>() {
            expr.push(Expr::Literal(Type::Float(f)))
        } else if chars[0] == '"' && chars[chars.len() - 1] == '"' {
            let inner_string = String::from_iter(chars[1..chars.len() - 1].iter());
            expr.push(Expr::Literal(Type::String(inner_string)))
        } else if chars[0] == '(' && chars[chars.len() - 1] == ')' {
            let inner_brace = String::from_iter(chars[1..chars.len() - 1].iter());
            expr.push(parse_expr(inner_brace))
        } else if token == "+" {
            expr.push(Expr::Operator(Operator::Add))
        } else if token == "-" {
            expr.push(Expr::Operator(Operator::Sub))
        } else if token == "*" {
            expr.push(Expr::Operator(Operator::Mul))
        } else if token == "/" {
            expr.push(Expr::Operator(Operator::Div))
        } else if token == "%" {
            expr.push(Expr::Operator(Operator::Mod))
        } else if token == "==" {
            expr.push(Expr::Operator(Operator::Equal))
        } else if token == ">" {
            expr.push(Expr::Operator(Operator::Greater))
        } else if token == "<" {
            expr.push(Expr::Operator(Operator::Less))
        } else {
            expr.push(Expr::Literal(Type::Symbol(token)))
        }
    }
    Expr::Expr(expr)
}

fn tokenize_expr(input: String) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_parentheses: usize = 0;
    let mut in_quote = false;

    for c in input.chars() {
        match c {
            '(' if !in_quote => {
                if in_parentheses != 0 {
                    in_parentheses += 1;
                    current_token.push(c);
                } else {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                    in_parentheses += 1;
                    current_token.push(c);
                }
            }
            ')' if !in_quote => {
                if in_parentheses != 0 {
                    current_token.push(c);
                    in_parentheses -= 1;
                    if in_parentheses == 0 {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                } else {
                    panic!("Syntax error: invalid end of parentheses")
                }
            }
            '"' => {
                if in_parentheses == 0 {
                    if in_quote {
                        current_token.push(c);
                        in_quote = false;
                        tokens.push(current_token.clone());
                        current_token.clear();
                    } else {
                        in_quote = true;
                        current_token.push(c);
                    }
                } else {
                    current_token.push(c);
                }
            }
            ' ' | '\n' | '\t' | '\r' | '　' => {
                if in_parentheses != 0 || in_quote {
                    current_token.push(c);
                } else {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                }
            }
            _ => {
                current_token.push(c);
            }
        }
    }

    if in_parentheses != 0 {
        panic!("Syntax error: There isn't end of parentheses");
    }
    if in_quote {
        panic!("Syntax error: There isn't end of quote");
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}
