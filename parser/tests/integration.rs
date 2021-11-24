fn assert_statement_eq(test: &str, result: &str) {
    assert_eq!(
        format!("{:?}", parser::parse(test).unwrap()),
        result.to_string()
    );
}

#[test]
fn test_parser() {
    assert_statement_eq("text", "Text(\"text\")");
    assert_statement_eq("hello I am dog", "Text(\"hello I am dog\")");
}
