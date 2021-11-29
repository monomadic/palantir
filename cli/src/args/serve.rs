pub(crate) fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let site = site::Site::<parser::AST>::read("./examples/index.md")?;
    server::start(site);
    Ok(())
}
