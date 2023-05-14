use crate::ast::{BinaryOperator, Expression, Literal, Program, Statement};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, char, digit1, newline};
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;

pub fn parse(code: &str) -> Program {
    let (code, program) = program(code).unwrap();
    assert_eq!(code, "");
    program
}

fn program(code: &str) -> IResult<&str, Program> {
    let (code, statements) = separated_list0(newline, statement)(code)?;
    Ok((code, Program { statements }))
}

fn statement(code: &str) -> IResult<&str, Statement> {
    alt((assign,))(code)
}

fn assign(code: &str) -> IResult<&str, Statement> {
    let (code, identifier) = identifier(code)?;
    let (code, _) = tag(" = ")(code)?;
    let (code, expression) = expression(code)?;
    Ok((code, Statement::Assign(identifier, expression)))
}

fn identifier(code: &str) -> IResult<&str, &str> {
    alpha1(code)
}

fn expression(code: &str) -> IResult<&str, Expression> {
    expression2(code)
}

fn expression2(code: &str) -> IResult<&str, Expression> {
    alt((add, expression1))(code)
}

fn expression1(code: &str) -> IResult<&str, Expression> {
    alt((member, expression0))(code)
}

fn expression0(code: &str) -> IResult<&str, Expression> {
    alt((literal, variable))(code)
}

fn add(code: &str) -> IResult<&str, Expression> {
    let (code, lhs) = expression1(code)?;
    let (code, op) = delimited(char(' '), arithmetic_operator, char(' '))(code)?;
    let (code, rhs) = expression1(code)?;
    Ok((
        code,
        Expression::BinaryOperator(Box::new(lhs), op, Box::new(rhs)),
    ))
}

fn arithmetic_operator(code: &str) -> IResult<&str, BinaryOperator> {
    let (code, op) = alt((char('+'), char('-'), char('*'), char('/')))(code)?;
    let op = match op {
        '+' => BinaryOperator::Add,
        '-' => BinaryOperator::Subtract,
        '*' => BinaryOperator::Multiply,
        '/' => BinaryOperator::Divide,
        _ => unreachable!(),
    };
    Ok((code, op))
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

fn member(code: &str) -> IResult<&str, Expression> {
    let (code, object) = expression0(code)?;
    let (code, _) = char('.')(code)?;
    let (code, member) = identifier(code)?;
    Ok((code, Expression::Member(Box::new(object), member)))
}
