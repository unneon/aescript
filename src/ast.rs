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

#[derive(Debug)]
pub enum Literal<'a> {
    Bool(bool),
    Number(f64),
    Text(&'a str),
}

#[derive(Debug)]
pub enum Expression<'a> {
    Array(Vec<Expression<'a>>),
    BinaryOperator(Box<Expression<'a>>, BinaryOperator, Box<Expression<'a>>),
    Call(&'a str, Vec<Expression<'a>>),
    Index(Box<Expression<'a>>, Box<Expression<'a>>),
    Literal(Literal<'a>),
    Member(Box<Expression<'a>>, &'a str),
    MethodCall(Box<Expression<'a>>, &'a str, Vec<Expression<'a>>),
    Variable(&'a str),
}

#[derive(Debug)]
pub enum Statement<'a> {
    Assign(&'a str, Expression<'a>),
    Function(&'a str, Function<'a>),
    If(Expression<'a>, Vec<Statement<'a>>),
    Return(Expression<'a>),
    While(Expression<'a>, Vec<Statement<'a>>),
}

#[derive(Debug)]
pub struct Function<'a> {
    pub arguments: Vec<&'a str>,
    pub statements: Vec<Statement<'a>>,
}

#[derive(Debug)]
pub struct Program<'a> {
    pub statements: Vec<Statement<'a>>,
}
