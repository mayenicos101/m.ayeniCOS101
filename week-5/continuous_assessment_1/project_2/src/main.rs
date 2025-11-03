use std::io;

fn main() {
    loop {
    let mut p = String::new();
    let mut r = String::new();
    let mut t = String::new();

    println!("Input your deposit amount:");
    io::stdin().read_line(&mut p).expect("Not a valid string");
    let p:f32 = p.trim().parse().expect("Deposit amount is invalid");

    println!("Input your bank's annual rate:");
    io::stdin().read_line(&mut r).expect("Not a valid string");
    let r:f32 = r.trim().parse().expect("Deposit amount is invalid");

    println!("Input the time (in years) the money is being saved:");
    io::stdin().read_line(&mut t).expect("Not a valid string");
    let t:f32 = t.trim().parse().expect("Deposit amount is invalid");

        let amount:f32 = p * (1.0 + r / 100.0).powf(t);
        let compound_interest:f32 = amount - p;

        println!("The Compound Interest is {} and the Total Amount is {}" ,compound_interest, amount);

    println!("Do you want to continue? (y/n)");
    let mut continue_input = String::new();
    io::stdin().read_line(&mut continue_input).expect("Failed to read input");
    let continue_input = continue_input.trim();

    if continue_input == "n" {
            break;
        } else if continue_input == "y" {
            continue;
        } else {
            println!("Input ONLY y/n");
            return;
        }

    }
}