use parser::Renderable;
use std::path::Path;

pub(crate) fn build<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
    // let mut page = Page::new(path.as_ref());
    // page.read()?;
    // page.parse()?;
    // println!("{}", page.render_html());
    // let site = site::Site::<parser::AST>::read("./examples/index.md")?;
    // let page = site.request_html("/");

    let file = std::fs::read_to_string(&path)?;
    let ast = parser::parse(&*file)?;

    println!("{}", ast.render_html());

    Ok(())
}
