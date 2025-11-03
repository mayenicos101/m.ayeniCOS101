use std::io;

fn main() {
    println!("Welcome to our store!");
    println!("To buy something input the code and quantity give from the list below");
    println!("Code     Item    Price(₦)
        \nl     Laptop      550_000
        \nm     Monitor     120_000
        \nk     Keyboard    15_000
        \nh     Headset     25_000");
    let l:f64 = 550_000.0;
    let m:f64 = 120_000.0;
    let k:f64 = 15_000.0;
    let h:f64 = 25_000.0;
    
    let mut code_input = String::new();
    println!("Input the code of what you want to purchase:");
    io::stdin().read_line(&mut code_input).expect("Not a valid code");

    let mut price:f32 = 0.0;

    if code_input == "l" {
        price = 550_000.0;
    } else if code_input == "m" {
        price = 120_000.0;
    } else if code_input == "k" {
        price = 15_000.0;
    } else if code_input == "h" {
        price = 25_000.0;
    } else {
        println!("The code is not concurrent with any of the items above.");
    }

    let mut quantity = String::new();
    println!("Input the quantity of what you want to buy");
    io::stdin().read_line(&mut quantity).expect("Not a valid input");
    let quantity:f32 = quantity.trim().parse().expect("Your amount must be a number");

    let mut total_cost = price * quantity;
    println!("The total amount is {}", total_cost);
}


