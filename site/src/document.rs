use std::path::Path;

pub trait Document {
    // type Data;

    fn new<P: AsRef<Path>>(path: P) -> Self;
    fn read(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn parse(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn render(&self) -> String;
}

// #[derive(Default, Debug)]
// pub struct Document<P: AsRef<Path> + Renderable> {
//     // title: Option<String>,
//     // pub nodes: Node<Box<dyn Renderable>>, // needs to be a Node
//     path: P,
//     memo: String,
//     // memo: Option<R>,
//     // data: T
// }

// impl<P: AsRef<Path> + Renderable> Document<P> {
//     // pub fn new<P: AsRef<Path>>(path: P) -> Result<Document, Box<dyn std::error::Error>> {
//     //     Ok(Document {
//     //         path: path.as_ref().to_path_buf(),
//     //         data: None,
//     //         // ..Self::default()
//     //     })
//     // }
//     pub fn new(path: P) -> Result<Document<P>, Box<dyn std::error::Error>> {
//         Ok(Document {
//             path,
//             memo: String::new(), // memo: String::new(),
//                                  // ..Self::default()
//         })
//     }

//     pub fn read(&mut self) -> Result<(), Box<dyn std::error::Error>> {
//         self.memo = std::fs::read_to_string(self.path.as_ref())?;
//         Ok(())
//     }
// }

// // impl Parseable for Document {
// //     // read and parse, return renderable (probably the ast of a supported language)
// //     pub fn parse(&self) -> Result<Box<dyn Renderable>, Box<dyn std::error::Error>> {
// //         // for now, assume we have an astryx document
// //         Ok(self.parse())
// //     }
// // }

// impl Renderable for Document<PathBuf> {
//     fn render_html(&self) -> String {
//         self.path.render_html()
//     }
// }

// impl Renderable for PathBuf {
//     fn render_html(&self) -> String {
//         panic!("should never be called");
//     }
// }
