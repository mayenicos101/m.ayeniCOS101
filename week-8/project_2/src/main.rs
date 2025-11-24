use std::io;

fn main() {
    let mut people: Vec<(String, u32)> = Vec::new();

    'interview: loop {
        println!("Enter name:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input");
        let name = name.trim().parse().expect("Not a valid string");

        println!("Enter years of experience:");
        let mut experience = String::new();
        io::stdin().read_line(&mut experience).unwrap();
        let experience:u32 = match experience.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        people.push((name, experience));

        loop {
            println!("Do you want to enter another person? (y/n):");
            let mut answer = String::new();
            io::stdin().read_line(&mut answer).expect("Failed to read input");
            let answer = answer.trim().to_lowercase();
            if answer == "y" {
                break;
            } else if answer == "n" {
                break 'interview;
            } else {
                println!("Only input y or n!");
                continue;
            }
        }
    }

    if people.is_empty() {
        println!("No data entered.");
        return;
    }

    people.sort_by(|a, b| b.1.cmp(&a.1));

    println!("\nRanking based on experience:");
    for (i, (name, exp)) in people.iter().enumerate() {
        println!("{}. {} - {} years", i + 1, name, exp);
    }

    let highest = &people[0];
    println!("\nPerson with the highest experience: {} - {} years", highest.0, highest.1);
}