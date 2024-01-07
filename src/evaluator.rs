use crate::{
    ast::{Expr, Program, Stmt},
    object::Object,
};

pub fn eval_program(prog: Program) -> Result<Object, &str> {
    let mut result = Ok(Object::Null);
    for stmt in prog.stmts {
        if let Stmt::Expr { expr } = stmt {
            result = eval(expr);
        }
    }
    result
}

fn eval(expr: Expr) -> Result<Object, &str> {
    match expr {
        //Expr::Identifier { token, value } => todo!(),
        Expr::IntLiteral { value, .. } => Ok(Object::Integer { value }),
        Expr::BoolLiteral { value, .. } => Ok(Object::Bool { value }),
        //Expr::FnLiteral { token, parameters, block } => todo!(),
        //Expr::Call { token, function, arguments } => todo!(),
        Expr::Prefix { op, expr, .. } => {
            let right = eval(*expr)?;
            eval_prefix_expr(op, right)
        }
        Expr::Infix {
            left, op, right, ..
        } => eval_infix_expr(op, eval(*left)?, eval(*right)?),
        //Expr::If { token, condition, consequence, alternative } => todo!(),
        _ => Err("not supported expr type"),
    }
}

fn eval_prefix_expr(op: &[u8], right: Object) -> Result<Object, &str> {
    match op {
        b"!" => match right {
            Object::Bool { value } => Ok(Object::Bool { value: !value }),
            _ => Err("operator '!' only applies to boolean types"),
        },
        b"-" => match right {
            Object::Integer { value } => Ok(Object::Integer { value: -value }),
            _ => Err("operator '-' only applies to numbers"),
        },
        _ => Err("operator not supported"),
    }
}

fn eval_infix_expr(op: &[u8], left: Object, right: Object) -> Result<Object, &str> {
    match (left, right) {
        (Object::Integer { value: left }, Object::Integer { value: right }) => {
            Ok(Object::Integer {
                value: match op {
                    b"*" => left * right,
                    b"-" => left - right,
                    b"+" => left + right,
                    b"/" => left / right,
                    _ => return Err("not supported"),
                },
            })
        }
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
        if let Object::Integer { value } = obj {
            assert_eq!(*value, evalue)
        } else {
            panic!("not an integer object")
        }
    }

    fn assert_bool_obj(obj: &Object, evalue: bool) {
        if let Object::Bool { value } = obj {
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
        let inputs = [("true", true), ("false", false)];
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
}
