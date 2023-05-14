use crate::interpreter::run;
use crate::parser::parse;

#[test]
fn literal_bool() {
    let ast = parse("a = true\nb = false");
    let state = run(&ast);
    assert_eq!(state["a"], true);
    assert_eq!(state["b"], false);
}

#[test]
fn literal_number() {
    let ast = parse("a = 42");
    let state = run(&ast);
    assert_eq!(state["a"], 42);
}

#[test]
fn literal_text() {
    let ast = parse("a = \"Hello, world!\"");
    let state = run(&ast);
    assert_eq!(state["a"], "Hello, world!");
}

#[test]
fn expression_variable() {
    let ast = parse("a = 42\nb = a");
    let state = run(&ast);
    assert_eq!(state["b"], 42);
}

#[test]
fn text_length() {
    let ast = parse("a = \"hello\".length");
    let state = run(&ast);
    assert_eq!(state["a"], 5);
}

#[test]
fn add_number() {
    let ast = parse("a = 2 + 2");
    let state = run(&ast);
    assert_eq!(state["a"], 4);
}

#[test]
fn add_text() {
    let ast = parse("a = \"a\" + \"b\"");
    let state = run(&ast);
    assert_eq!(state["a"], "ab");
}

#[test]
fn sub_number() {
    let ast = parse("a = 13 - 8");
    let state = run(&ast);
    assert_eq!(state["a"], 5);
}

#[test]
fn mul_number() {
    let ast = parse("a = 2 * 3");
    let state = run(&ast);
    assert_eq!(state["a"], 6);
}

#[test]
fn div_number() {
    let ast = parse("a = 6 / 2");
    let state = run(&ast);
    assert_eq!(state["a"], 3);
}
