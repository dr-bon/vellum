use std::path::Path;
use vellum_app::vellum_core::document::DocumentBuffer;
fn main() {
    let fp = Path::new("vellum-tui/src/hello_world.txt");
    let doc = DocumentBuffer::from_file(fp).expect("Failed to read document.");
    println!("Document was read:\n{}", doc.get_contents());
}
