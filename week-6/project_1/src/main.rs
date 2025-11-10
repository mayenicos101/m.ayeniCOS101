use std::io;

fn main() {
    
    let mut grand_total = 0;

    println!("Welcome to Bose's Kitchenette!");
    println!("\nMenu                          Price
        \nPoundo Yam/Edinkaiko Soup     N3200
        \nFried Rice & Chicken          N3000
        \nAmala & Ewedu Soup            N2500
        \nEba & Egusi Soup              N2000
        \nWhite Rice & Stew             N2500");

    'main_ordering: loop {
        let item_name;
        let price;

        loop {
            let mut choice = String::new();
            println!("Select what you want to order by typing the first letter of the order.");
            io::stdin().read_line(&mut choice).expect("Not a valid string");
            let choice = choice.trim().to_uppercase();

            if choice == "P" {
                item_name = "Poundo Yam / Edinkaiko Soup";
                price = 3200;
                break;
            } else if choice == "F" {
                item_name = "Fried Rice & Chicken";
                price = 3000;
                break;
            } else if choice == "A" {
                item_name = "Amala & Ewedu Soup";
                price = 2500;
                break;
            } else if choice == "E" {
                item_name = "Eba & Egusi Soup";
                price = 2000;
                break;
            } else if choice == "W" {
                item_name = "White Rice & Stew";
                price = 2500;
                break;
            } else {
                println!("Invalid choice. Pick between P,F,A,E and W.");
                continue;
            }
        }
        println!("You selected {}. Finish with this item if you still want to get anything else.", item_name);
        
        let quantity = loop {
            let mut quantity = String::new();

            println!("Enter quantity:");
            io::stdin().read_line(&mut quantity).expect("Not a valid string");
            match quantity.trim().parse::<u32>() {
                Ok(q) => break q,
                Err(_) => println!("Please enter a valid number!"),
            }
        };

        let cost = price * quantity;
        grand_total += cost;
        println!("Ordered {} x {} = N{}", quantity, item_name, cost);
        println!("Current order cost: N{}", cost);
        println!("Grand total so far: N{}", grand_total);

        loop {
            println!("Do you want to order another item? (Y/N)");
            let mut continue_input = String::new();
            io::stdin().read_line(&mut continue_input).expect("Not a valid string");
            let continue_input = continue_input.trim().to_uppercase();

            if continue_input == "Y" {
                break;
            } else if continue_input == "N" {
                break 'main_ordering;
            } else {
                println!("Only input Y or N!");
                continue;
            } 
        } 
    }
    println!("Your grand total is N{}", grand_total);
    println!("Thank you for ordering at Bose's Kitchenette!");
}