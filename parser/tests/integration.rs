use parser::{parse, Renderable};

fn assert_statement_eq(test: &str, debug: &str, html: &str) {
    assert_eq!(&format!("{:?}", parser::parse(test)), debug);
    assert_eq!(&parser::parse(test).unwrap().render_html(), html);
}

#[test]
fn test_parser() {
    //assert_statement_eq("text", "Text(\"text\")");
    //assert_statement_eq("hello I am dog", "Text(\"hello I am dog\")");
    assert_statement_eq(
        "# hello I am dog",
        "Ok([Heading(Heading { level: 1, content: \"hello I am dog\" })])",
        "<h1>hello I am dog</h1>",
    );
    assert_statement_eq(
        "## Doggomaster\nParagraph",
        "Ok([Heading(Heading { level: 2, content: \"Doggomaster\" })])",
        "<h2>Doggomaster</h1><p>Paragraph</p>",
    );
}
