use crate::interpreter::run;
use crate::parser::parse;

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
