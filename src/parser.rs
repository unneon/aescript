use crate::ast::{BinaryOperator, Expression, Literal, Program, Statement};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, char, digit1, newline};
use nom::error::Error;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::{IResult, Parser};

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
    expression4(code)
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
    alt((member, expression0))(code)
}

fn expression0(code: &str) -> IResult<&str, Expression> {
    alt((literal, variable))(code)
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

fn comparison(code: &str) -> IResult<&str, Expression> {
    binary_expression(expression3, comparison_operator, code)
}

fn multiplication(code: &str) -> IResult<&str, Expression> {
    binary_expression(expression2, multiplicative_operator, code)
}

fn addition(code: &str) -> IResult<&str, Expression> {
    binary_expression(expression1, additive_operator, code)
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
