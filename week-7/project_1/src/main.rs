use std::io;

fn read_number(prompt: &str) -> f64 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim().parse::<f64>() {
            Ok(n) => return n,
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}
fn get_pi() -> f64 {
    22.0 / 7.0
}

fn area_trapezium() -> f64 {
    let height = read_number("Enter height:");
    let base1 = read_number("Enter base 1:");
    let base2 = read_number("Enter base 2:");
    (height / 2.0) * (base1 + base2)
}

fn area_rhombus() -> f64 {
    let d1 = read_number("Enter diagonal 1:");
    let d2 = read_number("Enter diagonal 2:");
    0.5 * d1 * d2
}

fn area_parallelogram() -> f64 {
    let base = read_number("Enter the base:");
    let altitude = read_number("Enter the altitude:");
    base * altitude
}

fn area_cube() -> f64 {
    let side = read_number("Enter the length of a side:");
    6.0 * side.powf(2.0)
}

fn volume_cylinder() -> f64 {
    let radius = read_number("Enter the radius:");
    let height = read_number("Enter the height:");
    get_pi() * radius.powf(2.0) * height
}

fn main() {
    loop {    
        let result = loop {
            let mut choice = String::new();
            println!("Select the formula you want to calculate");
            println!("A - Area of Trapezium");
            println!("B - Area of Rhombus");
            println!("C - Area of Parallelogram");
            println!("D - Area of Cube");
            println!("E - Volume of Cylinder");

            io::stdin().read_line(&mut choice).expect("Failed to read input");
            let choice = choice.trim().to_uppercase();

            match choice.as_str() {
                "A" => break area_trapezium(),
                "B" => break area_rhombus(),
                "C" => break area_parallelogram(),
                "D" => break area_cube(),
                "E" => break volume_cylinder(),
                _ => {
                    println!("Invalid choice. Please pick between A, B, C, D, or E!");
                    continue;
                }
            }
        };
        println!("The result is: {:.2}", result);
        loop {
            println!("Do you want to compute another shape?(y/n)");
            let mut continue_input = String::new();
            io::stdin().read_line(&mut continue_input).expect("Failed to read input");
            let continue_input = continue_input.trim().to_lowercase();

            if continue_input == "y" {
                break;
            } else if continue_input == "n"{
                println!("Goodbye!");
                return;
            } else {
                println!("Please input only y or n!");
            }
        }    
    }    
}