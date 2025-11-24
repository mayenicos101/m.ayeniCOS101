use std::fs::OpenOptions;
use std::io::Write;
use std::io::Read;

fn main() {
    let announce = "Week 9 - Rust File Input & Output\n";
    let dept = "Department of Computer Science";

    let mut source = std::fs::File::create("data.txt").expect("create failed");
    source.write_all("Welcome to Rust Programming\n".as_bytes()).expect("write failed");
    source.write_all(announce.as_bytes()).expect("write failed");
    source.write_all(dept.as_bytes()).expect("write failed");
    println!("\nData written to file.");

    let mut file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");
    file.write_all("\nHello Class".as_bytes()).expect("write failed");
    file.write_all("\nThis is the appendage to the document.".as_bytes()).expect("write failed");
    println!("file append success");

    println!("Extracting contents now");
    let mut code = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();
    code.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}