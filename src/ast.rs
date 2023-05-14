#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    And,
    Or,
}

pub enum Literal<'a> {
    Bool(bool),
    Number(f64),
    Text(&'a str),
}

pub enum Expression<'a> {
    BinaryOperator(Box<Expression<'a>>, BinaryOperator, Box<Expression<'a>>),
    Literal(Literal<'a>),
    Member(Box<Expression<'a>>, &'a str),
    Variable(&'a str),
}

pub enum Statement<'a> {
    Assign(&'a str, Expression<'a>),
}

pub struct Program<'a> {
    pub statements: Vec<Statement<'a>>,
}
