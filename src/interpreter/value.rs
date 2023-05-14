use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub enum Value {
    Bool(bool),
    Number(f64),
    Text(String),
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(bool) => Debug::fmt(bool, f),
            Value::Number(number) => Debug::fmt(number, f),
            Value::Text(text) => Debug::fmt(text, f),
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
