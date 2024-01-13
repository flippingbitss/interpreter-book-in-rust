use crate::{
    ast::{Expr, Program, Stmt},
    env::Env,
    object::Object,
};

pub fn eval_program<'a>(prog: Program<'a>, env: &mut Env<'a>) -> Result<Object, &'a str> {
    let mut result = Ok(Object::Null);
    for stmt in prog.stmts {
        result = eval_stmt(stmt, env);
        match result {
            Ok(Object::ReturnValue(value)) => return Ok(*value),
            err @ Err(_) => return err,
            _ => {}
        }
    }
    result
}

fn eval_block<'a>(stmts: Vec<Stmt<'a>>, env: &mut Env<'a>) -> Result<Object, &'a str> {
    let mut result = Ok(Object::Null);
    for stmt in stmts {
        result = eval_stmt(stmt, env);
        match result {
            Ok(Object::ReturnValue(_)) => return result,
            err @ Err(_) => return err,
            _ => {}
        }
    }
    result
}

fn eval_stmt<'a>(stmt: Stmt<'a>, env: &mut Env<'a>) -> Result<Object, &'a str> {
    match stmt {
        Stmt::Expr { expr } => eval(expr, env),
        Stmt::Block { stmts, .. } => eval_block(stmts, env),
        Stmt::Return { value, .. } => Ok(Object::ReturnValue(Box::new(eval(value, env)?))),
        Stmt::Let { name, value, .. } => {
            let result = eval(value, env);
            match result {
                Ok(value) => {
                    if let Expr::Identifier { value: name, .. } = name {
                        env.set(name, value);
                    }
                    Ok(Object::Null)
                }
                _ => result,
            }
        }
    }
}

fn eval<'a>(expr: Expr<'a>, env: &mut Env<'a>) -> Result<Object, &'a str> {
    match expr {
        Expr::Identifier { value, .. } => eval_identifier(value, env),
        Expr::IntLiteral { value, .. } => Ok(Object::Integer(value)),
        Expr::BoolLiteral { value, .. } => Ok(Object::Bool(value)),
        //Expr::FnLiteral { token, parameters, block } => todo!(),
        //Expr::Call { token, function, arguments } => todo!(),
        Expr::Prefix { op, expr, .. } => {
            let right = eval(*expr, env)?;
            eval_prefix_expr(op, right)
        }
        Expr::Infix {
            left, op, right, ..
        } => eval_infix_expr(op, eval(*left, env)?, eval(*right, env)?),
        Expr::If {
            condition,
            consequence,
            alternative,
            ..
        } => eval_conditional_expr(*condition, *consequence, alternative.map(|s| *s), env),
        _ => Err("not supported expr type"),
    }
}

fn eval_identifier<'a>(ident: &[u8], env: &mut Env<'a>) -> Result<Object, &'a str> {
    match env.get(ident) {
        Some(value) => Ok(value),
        None => Err("variable not found"),
    }
}

fn eval_conditional_expr<'a>(
    condition: Expr<'a>,
    consequence: Stmt<'a>,
    alternative: Option<Stmt<'a>>,
    env: &mut Env<'a>,
) -> Result<Object, &'a str> {
    let cond = eval(condition, env)?;
    match cond {
        Object::Bool(value) => {
            if value {
                eval_stmt(consequence, env)
            } else if alternative.is_some() {
                eval_stmt(alternative.unwrap(), env)
            } else {
                Ok(Object::Null)
            }
        }
        _ => return Err("conditional expression isn't a boolean"),
    }
}

fn eval_prefix_expr(op: &[u8], right: Object) -> Result<Object, &str> {
    match op {
        b"!" => match right {
            Object::Bool(value) => Ok(Object::Bool(!value)),
            _ => Err("operator '!' only applies to boolean types"),
        },
        b"-" => match right {
            Object::Integer(value) => Ok(Object::Integer(-value)),
            _ => Err("operator '-' only applies to numbers"),
        },
        _ => Err("operator not supported"),
    }
}

fn eval_infix_expr(op: &[u8], left: Object, right: Object) -> Result<Object, &str> {
    match (left, right) {
        (Object::Integer(left), Object::Integer(right)) => Ok(match op {
            b"*" => Object::Integer(left * right),
            b"-" => Object::Integer(left - right),
            b"+" => Object::Integer(left + right),
            b"/" => Object::Integer(left / right),
            b"<" => Object::Bool(left < right),
            b">" => Object::Bool(left > right),
            b"==" => Object::Bool(left == right),
            b"!=" => Object::Bool(left != right),
            _ => return Err("operator not supported for given types"),
        }),
        (Object::Bool(left), Object::Bool(right)) => Ok(match op {
            b"==" => Object::Bool(left == right),
            b"!=" => Object::Bool(left != right),
            _ => return Err("operator not supported for given types"),
        }),

        _ => Err("operand can only be applied to numbers"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{env::Env, lexer::Lexer, object::Object, parser::Parser};

    use super::eval_program;

    fn eval_prog<'a>(input: &'a str) -> Result<Object, &str> {
        let l = Lexer::new(input.as_bytes());
        let mut parser = Parser::new(l);
        let prog = parser.parse();
        let mut env = Env::new();

        match prog {
            Ok(p) => eval_program(p, &mut env),
            Err(_) => panic!("failed to evaluate program"),
        }
    }

    fn assert_int_obj(obj: &Object, evalue: i64) {
        if let Object::Integer(value) = obj {
            assert_eq!(*value, evalue)
        } else {
            panic!("not an integer object")
        }
    }

    fn assert_bool_obj(obj: &Object, evalue: bool) {
        if let Object::Bool(value) = obj {
            assert_eq!(*value, evalue)
        } else {
            panic!("not an boolean object")
        }
    }

    #[test]
    fn test_int_expr() {
        let inputs = [
            ("5", 5),
            ("10", 10),
            ("-5", -5),
            ("-10", -10),
            ("5 + 5 + 5 + 5 - 10", 10),
            ("2 * 2 * 2 * 2 * 2", 32),
            ("-50 + 100 + -50", 0),
            ("5 * 2 + 10", 20),
            ("5 + 2 * 10", 25),
            ("20 + 2 * -10", 0),
            ("50 / 2 * 2 + 10", 60),
            ("2 * (5 + 10)", 30),
            ("3 * 3 * 3 + 10", 37),
            ("3 * (3 * 3) + 10", 37),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
        ];
        for (i, expected) in inputs {
            let obj = eval_prog(i).unwrap();
            assert_int_obj(&obj, expected);
        }
    }

    #[test]
    fn test_bool_expr() {
        let inputs = [
            ("true", true),
            ("false", false),
            ("1 < 2", true),
            ("1 > 2", false),
            ("1 < 1", false),
            ("1 > 1", false),
            ("1 == 1", true),
            ("1 != 1", false),
            ("1 == 2", false),
            ("1 != 2", true),
            ("true == true", true),
            ("false == false", true),
            ("true == false", false),
            ("true != false", true),
            ("false != true", true),
            ("(1 < 2) == true", true),
            ("(1 < 2) == false", false),
            ("(1 > 2) == true", false),
            ("(1 > 2) == false", true),
        ];
        for (i, expected) in inputs {
            let obj = eval_prog(i).unwrap();
            assert_bool_obj(&obj, expected);
        }
    }

    #[test]
    fn test_bang_op() {
        let inputs = [
            ("!true", false),
            ("!false", true),
            ("!!true", true),
            ("!!false", false),
        ];
        for (i, expected) in inputs {
            let obj = eval_prog(i).unwrap();
            assert_bool_obj(&obj, expected);
        }
    }

    #[test]
    fn test_bang_with_non_boolean_types() {
        let neg_input = ["!5", "!!5"];
        for i in neg_input {
            assert!(eval_prog(&i).is_err());
        }
    }

    #[test]
    fn test_conditional_expr() {
        let truthy_inputs = [
            ("if (true) { 10 }", 10),
            ("if (1 < 2) { 10 }", 10),
            ("if (1 > 2) { 10 } else { 20 }", 20),
            ("if (1 < 2) { 10 } else { 20 }", 10),
        ];
        let falsy_inputs = ["if (false) { 10 }", "if (1 > 2) { 10 }"];

        for (input, evalue) in truthy_inputs {
            let value = eval_prog(input).unwrap();
            eprintln!("{} {}", input, value);
            assert_int_obj(&value, evalue);
        }

        for input in falsy_inputs {
            let value = eval_prog(input);
            assert!(matches!(value.unwrap(), Object::Null));
        }
    }

    #[test]
    fn test_return_stmt() {
        let inputs = [
            ("return 10;", 10),
            ("return 10; 9;", 10),
            ("return 2 * 5; 9;", 10),
            ("9; return 2 * 5; 9;", 10),
            ("if (10 > 1) { if (10 > 1) { return 10; } return 1; }", 10),
        ];
        for (i, expected) in inputs {
            eprintln!("{}", i);
            let obj = eval_prog(i).unwrap();
            assert_int_obj(&obj, expected);
        }
    }

    #[test]
    fn test_errors() {
        let inputs = [
            "5 + true;",
            "5 + true; 5;",
            "-true",
            "true + false;",
            "5; true + false; 5",
            "if (10 > 1) { true + false; }",
            "
if (10 > 1) {
if (10 > 1) {
return true + false;
}
return 1;
}
",
            "foobar",
        ];

        for i in inputs {
            let res = eval_prog(i);
            assert!(res.is_err());
        }
    }

    #[test]
    fn test_let_stmts() {
        let inputs = [
            ("let a = 5; a;", 5),
            ("let a = 5 * 5; a;", 25),
            ("let a = 5; let b = a; b;", 5),
            ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
        ];

        for (i, evalue) in inputs {
            let obj = eval_prog(i).unwrap();
            assert_int_obj(&obj, evalue);
        }
    }
}
