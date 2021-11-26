fn assert_statement_eq(test: &str, result: &str) {
    assert_eq!(format!("{:?}", parser::parse(test)), result.to_string());
}

#[test]
fn test_parser() {
    //assert_statement_eq("text", "Text(\"text\")");
    //assert_statement_eq("hello I am dog", "Text(\"hello I am dog\")");
    assert_statement_eq(
        "# hello I am dog",
        "Ok([Heading(Heading { level: 1, content: \"hello I am dog\" })])",
    );
    assert_statement_eq(
        "## Doggomaster\nParagraph",
        "Ok([Heading(Heading { level: 2, content: \"Doggomaster\" })])",
    );
}
