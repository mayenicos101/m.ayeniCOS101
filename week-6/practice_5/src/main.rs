fn main() {
    let fullname = " Pan-Atlantic University ";
    println!();
    println!("Name: {}", fullname);
    println!();
    println!("Before trim ");
    println!("The length is {}", fullname.len());
    println!();
    println!("After trim ");
    println!("The length is {}", fullname.trim().len());
}