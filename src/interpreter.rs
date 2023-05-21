pub mod value;

use crate::ast::{BinaryOperator, Expression, Function, Literal, Program, Statement};
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

struct State<'a> {
    variables: HashMap<&'a str, Value>,
    functions: HashMap<&'a str, &'a Function<'a>>,
}

impl State<'_> {
    fn new() -> State<'static> {
        State {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }
}

pub fn run<'a>(program: &'a Program<'a>) -> HashMap<&'a str, Value> {
    let mut state = State::new();
    run_statements(&program.statements, &mut state, false);
    state.variables
}

fn run_statements<'a>(
    statements: &'a [Statement<'a>],
    state: &mut State<'a>,
    is_function: bool,
) -> Option<Value> {
    for statement in statements {
        match statement {
            Statement::Assign(identifier, expression) => {
                let value = evaluate(expression, state);
                state.variables.insert(*identifier, value);
            }
            Statement::Function(identifier, function) => {
                state.functions.insert(*identifier, function);
            }
            Statement::If(condition, statements) => {
                let cond = match evaluate(condition, state) {
                    Value::Bool(cond) => cond,
                    cond => panic!("can't condition if with {cond:?}"),
                };
                if cond {
                    if let Some(return_value) = run_statements(statements, state, is_function) {
                        return Some(return_value);
                    }
                }
            }
            Statement::Return(expression) => {
                if is_function {
                    return Some(evaluate(expression, state));
                } else {
                    panic!("can't return in top level function")
                }
            }
            Statement::While(condition, statements) => loop {
                let cond = match evaluate(condition, state) {
                    Value::Bool(cond) => cond,
                    cond => panic!("can't condition while with {cond:?}"),
                };
                if !cond {
                    break;
                }
                if let Some(return_value) = run_statements(statements, state, is_function) {
                    return Some(return_value);
                }
            },
        }
    }
    None
}

fn evaluate(expression: &Expression, state: &State) -> Value {
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
        Expression::Call(function, arguments) => {
            assert_eq!(arguments.len(), state.functions[function].arguments.len());
            let mut call_state = State::new();
            for (i, argument) in arguments.iter().enumerate() {
                let arg_name = state.functions[function].arguments[i];
                let arg_value = evaluate(argument, state);
                call_state.variables.insert(arg_name, arg_value);
            }
            let return_value =
                run_statements(&state.functions[function].statements, &mut call_state, true);
            return_value.unwrap()
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
            let object = evaluate(object, state);
            match (object, *member) {
                (Value::Text(text), "length") => Value::Number(text.len() as f64),
                (Value::Array(elements), "length") => Value::Number(elements.len() as f64),
                (object, _) => panic!("unknown member {member:?} of value {object:?}"),
            }
        }
        Expression::Variable(variable) => state.variables[*variable].clone(),
    }
}
