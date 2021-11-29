use parser::{MarkdownParser, AST};

pub(crate) fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let mut site = site::Site::<AST, MarkdownParser>::new(MarkdownParser {});
    site.read()?;

    // let site = site::Site::<parser::AST>::read("./examples/index.md")?;
    server::start(site);
    Ok(())
}
