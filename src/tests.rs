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

#[test]
fn equal_number() {
    let ast = parse("a = 2 == 2\nb = 2 == 3\nc = 2 != 2\nd = 2 != 3");
    let state = run(&ast);
    assert_eq!(state["a"], true);
    assert_eq!(state["b"], false);
    assert_eq!(state["c"], false);
    assert_eq!(state["d"], true);
}

#[test]
fn equal_text() {
    let ast =
        parse("a = \"a\" == \"a\"\nb = \"a\" == \"b\"\nc = \"a\" != \"a\"\nd = \"a\" != \"b\"");
    let state = run(&ast);
    assert_eq!(state["a"], true);
    assert_eq!(state["b"], false);
    assert_eq!(state["c"], false);
    assert_eq!(state["d"], true);
}

#[test]
fn and() {
    let ast =
        parse("a = false and false\nb = false and true\nc = true and false\nd = true and true");
    let state = run(&ast);
    assert_eq!(state["a"], false);
    assert_eq!(state["b"], false);
    assert_eq!(state["c"], false);
    assert_eq!(state["d"], true);
}

#[test]
fn or() {
    let ast = parse("a = false or false\nb = false or true\nc = true or false\nd = true or true");
    let state = run(&ast);
    assert_eq!(state["a"], false);
    assert_eq!(state["b"], true);
    assert_eq!(state["c"], true);
    assert_eq!(state["d"], true);
}

#[test]
fn array() {
    let ast = parse("a = [2, \"test\"]");
    let state = run(&ast);
    assert_eq!(state["a"][0], 2);
    assert_eq!(state["a"][1], "test");
}

#[test]
fn array_length() {
    let ast = parse("a = [2, \"test\"].length");
    let state = run(&ast);
    assert_eq!(state["a"], 2);
}

#[test]
fn array_index() {
    let ast = parse("a = [2, \"test\"]\nb = a[0]\nc = a[1]");
    let state = run(&ast);
    assert_eq!(state["b"], 2);
    assert_eq!(state["c"], "test");
}

#[test]
fn func_return() {
    let ast = parse("func f()\n    return 2\na = f()");
    let state = run(&ast);
    assert_eq!(state["a"], 2);
}

#[test]
fn while_simple() {
    let ast = parse("i = 0\nwhile i != 5\n    i = i + 1");
    let state = run(&ast);
    assert_eq!(state["i"], 5);
}

#[test]
fn if_simple() {
    let ast = parse("i = 0\nif i == 0\n    i = 1\nif i == 0\n    i = 2");
    let state = run(&ast);
    assert_eq!(state["i"], 1);
}

#[test]
fn text_starts_with() {
    let ast = parse("a = \"hello\"\nb = a.starts_with(\"he\")\nc = a.starts_with(\"ha\")");
    let state = run(&ast);
    assert_eq!(state["b"], true);
    assert_eq!(state["c"], false);
}
