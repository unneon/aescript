use std::fmt::{Debug, Display, Formatter};

#[derive(Clone)]
pub enum Value {
    Number(f64),
    Text(String),
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(number) => Debug::fmt(number, f),
            Value::Text(text) => Debug::fmt(text, f),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(number) => Display::fmt(number, f),
            Value::Text(text) => Display::fmt(text, f),
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
