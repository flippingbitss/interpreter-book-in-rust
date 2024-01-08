use crate::{
    ast::{Expr, Program, Stmt},
    object::Object,
};

pub fn eval_program(prog: Program) -> Result<Object, &str> {
    let mut result = Ok(Object::Null);
    for stmt in prog.stmts {
        result = eval_stmt(stmt);
    }
    result
}

fn eval_stmt(stmt: Stmt<'_>) -> Result<Object, &str> {
    match stmt {
        Stmt::Expr { expr } => eval(expr),
        Stmt::Block { stmts, .. } => {
            let mut result = Object::Null;
            for stmt in stmts {
                result = eval_stmt(stmt)?
            }
            Ok(result)
        }
        _ => Ok(Object::Null),
    }
}

fn eval(expr: Expr) -> Result<Object, &str> {
    match expr {
        //Expr::Identifier { token, value } => todo!(),
        Expr::IntLiteral { value, .. } => Ok(Object::Integer(value)),
        Expr::BoolLiteral { value, .. } => Ok(Object::Bool(value)),
        //Expr::FnLiteral { token, parameters, block } => todo!(),
        //Expr::Call { token, function, arguments } => todo!(),
        Expr::Prefix { op, expr, .. } => {
            let right = eval(*expr)?;
            eval_prefix_expr(op, right)
        }
        Expr::Infix {
            left, op, right, ..
        } => eval_infix_expr(op, eval(*left)?, eval(*right)?),
        Expr::If {
            condition,
            consequence,
            alternative,
            ..
        } => eval_conditional_expr(*condition, *consequence, alternative.map(|s| *s)),
        _ => Err("not supported expr type"),
    }
}

fn eval_conditional_expr<'a>(
    condition: Expr<'a>,
    consequence: Stmt<'a>,
    alternative: Option<Stmt<'a>>,
) -> Result<Object, &'a str> {
    let cond = eval(condition)?;
    match cond {
        Object::Bool(value) => {
            if value {
                println!("bool is true");
                eval_program(Program {
                    stmts: vec![consequence],
                })
            } else if alternative.is_some() {
                eval_program(Program {
                    stmts: vec![alternative.unwrap()],
                })
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
            _ => return Err("not supported"),
        }),
        (Object::Bool(left), Object::Bool(right)) => Ok(match op {
            b"==" => Object::Bool(left == right),
            b"!=" => Object::Bool(left != right),
            _ => return Err("not supported"),
        }),

        _ => Err("operand can only be applied to numbers"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, object::Object, parser::Parser};

    use super::eval_program;

    fn eval_prog<'a>(input: &'a str) -> Result<Object, &str> {
        let l = Lexer::new(input.as_bytes());
        let mut parser = Parser::new(l);
        let prog = parser.parse();

        match prog {
            Ok(p) => eval_program(p),
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
}
