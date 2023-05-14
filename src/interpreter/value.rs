use std::fmt::{Debug, Formatter};
use std::ops::Index;

#[derive(Clone)]
pub enum Value {
    Array(Vec<Value>),
    Bool(bool),
    Number(f64),
    Text(String),
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Array(values) => Debug::fmt(values, f),
            Value::Bool(bool) => Debug::fmt(bool, f),
            Value::Number(number) => Debug::fmt(number, f),
            Value::Text(text) => Debug::fmt(text, f),
        }
    }
}

impl Index<usize> for Value {
    type Output = Value;

    fn index(&self, index: usize) -> &Value {
        match self {
            Value::Array(elements) => &elements[index],
            _ => panic!("can't index {self:?}"),
        }
    }
}

impl PartialEq<bool> for Value {
    fn eq(&self, rhs: &bool) -> bool {
        match self {
            Value::Bool(lhs) => lhs == rhs,
            _ => false,
        }
    }
}

impl PartialEq<i32> for Value {
    fn eq(&self, rhs: &i32) -> bool {
        match self {
            Value::Number(lhs) => *lhs == *rhs as f64,
            _ => false,
        }
    }
}

impl PartialEq<&str> for Value {
    fn eq(&self, rhs: &&str) -> bool {
        match self {
            Value::Text(lhs) => lhs == rhs,
            _ => false,
        }
    }
}
