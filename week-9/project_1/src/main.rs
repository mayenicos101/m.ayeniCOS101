use std::io::Write;
use std::io::Read;

fn main() {
    let mut file = std::fs::File::create("drinks.txt").expect("create failed");

    file.write_all("Lager:\n".as_bytes()).expect("write failed");
    file.write_all("33 Export\n".as_bytes()).expect("write failed");
    file.write_all("Desperados\n".as_bytes()).expect("write failed");
    file.write_all("Goldberg\n".as_bytes()).expect("write failed");
    file.write_all("Gulder\n".as_bytes()).expect("write failed");
    file.write_all("Heineken\n".as_bytes()).expect("write failed");
    file.write_all("Star\n\n".as_bytes()).expect("write failed");

    file.write_all("Stout:\n".as_bytes()).expect("write failed");
    file.write_all("Legend\n".as_bytes()).expect("write failed");
    file.write_all("Turbo King\n".as_bytes()).expect("write failed");
    file.write_all("Williams\n\n".as_bytes()).expect("write failed");

    file.write_all("Non-Alcoholic:\n".as_bytes()).expect("write failed");
    file.write_all("Maltina\n".as_bytes()).expect("write failed");
    file.write_all("Amstel Malta\n".as_bytes()).expect("write failed");
    file.write_all("Malta Gold\n".as_bytes()).expect("write failed");
    file.write_all("Fayrouz\n".as_bytes()).expect("write failed");

    println!("Data written to file.");

    let mut source = std::fs::File::open("drinks.txt").expect("create failed");
    let mut contents = String::new();
    source.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}