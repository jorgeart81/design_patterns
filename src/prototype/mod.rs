use crate::prototype::document::Document;

mod document;

pub fn run_prototype() {
    let doc = Document::new("Computer quote", "$2800", "Apple");

    let mut doc2 = doc.clone();
    doc2.author = String::from("Lenovo");

    println!("Document 1: {:?}", doc);
    println!("Document 2: {:?}", doc2);
}
