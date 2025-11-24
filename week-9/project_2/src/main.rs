use std::io::{Write, Read};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let students = vec![
        ("Oluchi Mordi","ACC10211111","Accounting",300),
        ("Adams Aliyu","ECO10110101","Economics",100),
        ("Shania Bolade","CSC10328828","Computer",200),
        ("Adekunle Gold","EEE11020202","Electrical",200),
        ("Blanca Edemoh","MEE10202001","Mechanical",100)
    ];

    println!("Writing data to file...");
    let mut file = File::create("students.txt")?;

    writeln!(file, "Student Name | Matric. Number | Department | Level")?;
    writeln!(file, "--------------------------------------------------")?;

    for student in &students {
        writeln!(file, "{} | {} | {} | {}", student.0, student.1, student.2, student.3)?;
    }

    println!("Data has been successfully written to students.txt");

    let mut source = File::open("students.txt")?;
    let mut contents = String::new();
    source.read_to_string(&mut contents)?;
    println!("Contents of students.txt:");
    println!("{}", contents);

    Ok(())
}