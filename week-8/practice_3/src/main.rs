fn main() {
    let v = vec!['R','U','S','T','A','C','I','A','N'];

    loop {
        println!("Enter an index value between 0 and 8");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let index:usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        match v.get(index) {
            Some(ch) => {
                println!("Element of vector: {}", ch);
                break;
            } None => {
                println!("Index out of range! Choose between 0 and 8");
            }
        }
    }
}