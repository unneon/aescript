pub mod value;

use crate::ast::{Expression, Literal, Program, Statement};
use crate::interpreter::value::Value;
use std::collections::HashMap;

pub fn run<'a>(program: &'a Program<'a>) -> HashMap<&'a str, Value> {
    let mut state = HashMap::new();
    for statement in &program.statements {
        match statement {
            Statement::Assign(identifier, expression) => {
                state.insert(*identifier, evaluate(expression, &state));
            }
        }
    }
    state
}

pub fn evaluate(expression: &Expression, state: &HashMap<&str, Value>) -> Value {
    match expression {
        Expression::Literal(literal) => match literal {
            Literal::Bool(bool) => Value::Bool(*bool),
            Literal::Number(number) => Value::Number(*number),
            Literal::Text(text) => Value::Text((*text).to_owned()),
        },
        Expression::Member(object, member) => {
            let object = evaluate(&object, state);
            match (&object, *member) {
                (Value::Text(text), "length") => Value::Number(text.len() as f64),
                _ => panic!("unknown member {member:?} of value {object:?}"),
            }
        }
        Expression::Variable(variable) => state[*variable].clone(),
    }
}
