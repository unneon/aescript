use crate::ast::{BinaryOperator, Expression, Function, Literal, Program, Statement};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_while1};
use nom::character::complete::{char, digit1, newline};
use nom::combinator::cut;
use nom::error::Error;
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, preceded};
use nom::{IResult, Parser};

pub fn parse(code: &str) -> Program {
    let (code, program) = program(code).unwrap();
    assert_eq!(code, "");
    program
}

fn program(code: &str) -> IResult<&str, Program> {
    let (code, statements) = separated_list0(newline, cut(statement))(code)?;
    Ok((code, Program { statements }))
}

fn statement(code: &str) -> IResult<&str, Statement> {
    statement1(code)
}

fn statement1(code: &str) -> IResult<&str, Statement> {
    alt((function, while_loop, if_statement, statement0))(code)
}

fn statement0(code: &str) -> IResult<&str, Statement> {
    alt((return_statement, assign))(code)
}

fn function(code: &str) -> IResult<&str, Statement> {
    let (code, _) = tag("func ")(code)?;
    let (code, name) = identifier(code)?;
    let (code, arguments) =
        delimited(char('('), separated_list0(tag(", "), identifier), char(')'))(code)?;
    let (code, statements) = many0(preceded(tag("\n    "), statement0))(code)?;
    let function = Function {
        arguments,
        statements,
    };
    Ok((code, Statement::Function(name, function)))
}

fn while_loop(code: &str) -> IResult<&str, Statement> {
    let (code, _) = tag("while ")(code)?;
    let (code, condition) = expression(code)?;
    let (code, statements) = many0(preceded(tag("\n    "), statement0))(code)?;
    Ok((code, Statement::While(condition, statements)))
}

fn if_statement(code: &str) -> IResult<&str, Statement> {
    let (code, _) = tag("if ")(code)?;
    let (code, condition) = expression(code)?;
    let (code, statements) = many0(preceded(tag("\n    "), statement0))(code)?;
    Ok((code, Statement::If(condition, statements)))
}

fn return_statement(code: &str) -> IResult<&str, Statement> {
    let (code, _) = tag("return ")(code)?;
    let (code, expression) = expression(code)?;
    Ok((code, Statement::Return(expression)))
}

fn assign(code: &str) -> IResult<&str, Statement> {
    let (code, identifier) = identifier(code)?;
    let (code, _) = tag(" = ")(code)?;
    let (code, expression) = expression(code)?;
    Ok((code, Statement::Assign(identifier, expression)))
}

fn identifier(code: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_ascii_lowercase() || c == '_')(code)
}

fn expression(code: &str) -> IResult<&str, Expression> {
    expression5(code)
}

fn expression5(code: &str) -> IResult<&str, Expression> {
    alt((logic, expression4))(code)
}

fn expression4(code: &str) -> IResult<&str, Expression> {
    alt((comparison, expression3))(code)
}

fn expression3(code: &str) -> IResult<&str, Expression> {
    alt((multiplication, expression2))(code)
}

fn expression2(code: &str) -> IResult<&str, Expression> {
    alt((addition, expression1))(code)
}

fn expression1(code: &str) -> IResult<&str, Expression> {
    alt((method, member, index, call, expression0))(code)
}

fn expression0(code: &str) -> IResult<&str, Expression> {
    alt((array, literal, variable))(code)
}

fn binary_expression<'a>(
    mut subexpression: impl Parser<&'a str, Expression<'a>, Error<&'a str>>,
    operator: impl Parser<&'a str, BinaryOperator, Error<&'a str>>,
    code: &'a str,
) -> IResult<&'a str, Expression> {
    let (code, lhs) = subexpression.parse(code)?;
    let (code, op) = delimited(char(' '), operator, char(' '))(code)?;
    let (code, rhs) = subexpression.parse(code)?;
    Ok((
        code,
        Expression::BinaryOperator(Box::new(lhs), op, Box::new(rhs)),
    ))
}

fn logic(code: &str) -> IResult<&str, Expression> {
    binary_expression(expression4, logic_operator, code)
}

fn comparison(code: &str) -> IResult<&str, Expression> {
    binary_expression(expression3, comparison_operator, code)
}

fn multiplication(code: &str) -> IResult<&str, Expression> {
    binary_expression(expression2, multiplicative_operator, code)
}

fn addition(code: &str) -> IResult<&str, Expression> {
    binary_expression(expression1, additive_operator, code)
}

fn logic_operator(code: &str) -> IResult<&str, BinaryOperator> {
    let (code, op) = alt((tag("and"), tag("or")))(code)?;
    let op = match op {
        "and" => BinaryOperator::And,
        "or" => BinaryOperator::Or,
        _ => unreachable!(),
    };
    Ok((code, op))
}

fn comparison_operator(code: &str) -> IResult<&str, BinaryOperator> {
    let (code, op) = alt((tag("=="), tag("!=")))(code)?;
    let op = match op {
        "==" => BinaryOperator::Equal,
        "!=" => BinaryOperator::NotEqual,
        _ => unreachable!(),
    };
    Ok((code, op))
}

fn multiplicative_operator(code: &str) -> IResult<&str, BinaryOperator> {
    let (code, op) = alt((char('*'), char('/')))(code)?;
    let op = match op {
        '*' => BinaryOperator::Multiply,
        '/' => BinaryOperator::Divide,
        _ => unreachable!(),
    };
    Ok((code, op))
}

fn additive_operator(code: &str) -> IResult<&str, BinaryOperator> {
    let (code, op) = alt((char('+'), char('-')))(code)?;
    let op = match op {
        '+' => BinaryOperator::Add,
        '-' => BinaryOperator::Subtract,
        _ => unreachable!(),
    };
    Ok((code, op))
}

fn array(code: &str) -> IResult<&str, Expression> {
    let (code, elements) =
        delimited(char('['), separated_list0(tag(", "), expression), char(']'))(code)?;
    Ok((code, Expression::Array(elements)))
}

fn literal(code: &str) -> IResult<&str, Expression> {
    let (code, literal) = alt((literal_bool, literal_number, literal_text))(code)?;
    Ok((code, Expression::Literal(literal)))
}

fn literal_bool(code: &str) -> IResult<&str, Literal> {
    let (code, value) = alt((tag("true"), tag("false")))(code)?;
    Ok((code, Literal::Bool(value == "true")))
}

fn literal_number(code: &str) -> IResult<&str, Literal> {
    let (code, number) = digit1(code)?;
    Ok((code, Literal::Number(number.parse().unwrap())))
}

fn literal_text(code: &str) -> IResult<&str, Literal> {
    let (code, text) = delimited(char('"'), take_until("\""), char('"'))(code)?;
    Ok((code, Literal::Text(text)))
}

fn variable(code: &str) -> IResult<&str, Expression> {
    let (code, identifier) = identifier(code)?;
    Ok((code, Expression::Variable(identifier)))
}

fn method(code: &str) -> IResult<&str, Expression> {
    let (code, object) = expression0(code)?;
    let (code, _) = char('.')(code)?;
    let (code, method) = identifier(code)?;
    let (code, arguments) = call_arguments(code)?;
    let method_call = Expression::MethodCall(Box::new(object), method, arguments);
    Ok((code, method_call))
}

fn member(code: &str) -> IResult<&str, Expression> {
    let (code, object) = expression0(code)?;
    let (code, _) = char('.')(code)?;
    let (code, member) = identifier(code)?;
    Ok((code, Expression::Member(Box::new(object), member)))
}

fn index(code: &str) -> IResult<&str, Expression> {
    let (code, array) = expression0(code)?;
    let (code, index) = delimited(char('['), expression, char(']'))(code)?;
    Ok((code, Expression::Index(Box::new(array), Box::new(index))))
}

fn call(code: &str) -> IResult<&str, Expression> {
    let (code, function) = identifier(code)?;
    let (code, arguments) = call_arguments(code)?;
    Ok((code, Expression::Call(function, arguments)))
}

fn call_arguments(code: &str) -> IResult<&str, Vec<Expression>> {
    delimited(char('('), separated_list0(tag(", "), expression), char(')'))(code)
}
