use std::io;

fn add(a:i32, b:i32) {
    let sum = a + b;

    println!("The sum of a and b is {}", sum);
}
fn main () {

    let mut input1 = String::new();
    println!("Enter input for parameter a:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:i32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter input for parameter b:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:i32 = input2.trim().parse().expect("Invalid input");

    add (a, b);
}