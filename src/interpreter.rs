pub mod value;

use crate::ast::{BinaryOperator, Expression, Literal, Program, Statement};
use crate::interpreter::value::Value;
use std::collections::HashMap;

macro_rules! evalute_binary_expression {
    ($vop:ident $vlhs:ident $vrhs:ident $($op:ident $lhs:ident $rhs:ident => $t:ident $e:expr,)*) => {
        match ($vop, $vlhs, $vrhs) {
            $((BinaryOperator::$op, Value::$lhs($vlhs), Value::$rhs($vrhs)) => Value::$t($e),)*
            (op, lhs, rhs) => panic!("can't evaluate {lhs:?} {op:?} {rhs:?}"),
        }
    };
}

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
        Expression::Array(subexprs) => Value::Array(
            subexprs
                .iter()
                .map(|subexpr| evaluate(subexpr, state))
                .collect(),
        ),
        Expression::BinaryOperator(lhs, op, rhs) => {
            let lhs = evaluate(lhs, state);
            let rhs = evaluate(rhs, state);
            evalute_binary_expression! {
                op lhs rhs
                Add Number Number => Number lhs + rhs,
                Add Text Text => Text lhs + &rhs,
                Subtract Number Number => Number lhs - rhs,
                Multiply Number Number => Number lhs * rhs,
                Divide Number Number => Number lhs / rhs,
                Equal Number Number => Bool lhs == rhs,
                Equal Text Text => Bool lhs == rhs,
                NotEqual Number Number => Bool lhs != rhs,
                NotEqual Text Text => Bool lhs != rhs,
                And Bool Bool => Bool lhs && rhs,
                Or Bool Bool => Bool lhs || rhs,
            }
        }
        Expression::Index(array, index) => {
            let array = evaluate(array, state);
            let index = evaluate(index, state);
            match (array, index) {
                (Value::Array(array), Value::Number(index)) => array[index as usize].clone(),
                (array, index) => panic!("can't index {array:?} with {index:?}"),
            }
        }
        Expression::Literal(literal) => match literal {
            Literal::Bool(bool) => Value::Bool(*bool),
            Literal::Number(number) => Value::Number(*number),
            Literal::Text(text) => Value::Text((*text).to_owned()),
        },
        Expression::Member(object, member) => {
            let object = evaluate(&object, state);
            match (object, *member) {
                (Value::Text(text), "length") => Value::Number(text.len() as f64),
                (Value::Array(elements), "length") => Value::Number(elements.len() as f64),
                (object, _) => panic!("unknown member {member:?} of value {object:?}"),
            }
        }
        Expression::Variable(variable) => state[*variable].clone(),
    }
}
