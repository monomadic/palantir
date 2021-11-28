use parser::Renderable;

fn assert_statement_eq(test: &str, html: &str, debug: &str) {
    assert_eq!(&format!("{:?}", parser::parse(test)), debug);
    assert_eq!(&parser::parse(test).unwrap().render_html(), html);
}

fn assert_eq(test: &str, html: &str) {
    assert_eq!(&parser::parse(test).unwrap().render_html(), html);
}

#[test]
fn test_statements() {
    //assert_statement_eq("text", "Text(\"text\")");
    //assert_statement_eq("hello I am dog", "Text(\"hello I am dog\")");
    assert_statement_eq(
        "# hello I am dog\n",
        "<h1>hello I am dog</h1>",
        "Ok(AST { nodes: [Heading((1, [Text(\"hello I am dog\")]))] })",
    );
    assert_statement_eq(
        "## Doggomaster\nParagraph\n",
        "<h2>Doggomaster</h2><p>Paragraph</p>",
        "Ok(AST { nodes: [Heading((2, [Text(\"Doggomaster\")])), Paragraph([Text(\"Paragraph\")])] })",
    );
    assert_statement_eq(
        "basic text bro\n",
        "<p>basic text bro</p>",
        "Ok(AST { nodes: [Paragraph([Text(\"basic text bro\")])] })",
    );
}

#[test]
fn test_expressions() {
    assert_statement_eq(
        "**bold text**\n",
        "<p><strong>bold text</strong></p>",
        "Ok(AST { nodes: [Paragraph([BoldText([Text(\"bold text\")])])] })",
    );
}
