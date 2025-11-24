use std::fs;
use std::io::Write;
use std::io::Read;

fn main() {
    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("dummy text.".as_bytes()).expect("create failed");
    println!("Data written to file");

    println!("Extracting contents now");
    let mut open = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();
    open.read_to_string(&mut contents).unwrap();
    println!("{}", contents);

    println!("Deleting file...");
    fs::remove_file("data.txt").expect("could not remove_file");
    println!("file is removed");
}