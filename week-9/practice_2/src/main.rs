use std::io::Read;
use std::io::Write;

fn main() {
    let status = "A course where you're bound to fail";

    let mut source = std::fs::File::create("welcome_message.txt").expect("create failed");
    source.write_all("Welcome to COS-101 Class!\n".as_bytes()).expect("create failed");
    source.write_all(status.as_bytes()).expect("create failed");
    println!("Data written to file");

    println!("Extracting contents now");
    let mut file = std::fs::File::open("welcome_message.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}