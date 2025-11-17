fn main() {
    let v:Vec<i64> = Vec::new();
    println!("\nThe lenght of Vec::new is: {}", v.len());

    let v = vec!["Grace","Effiong","Basil","Kareem","Susan"];
    println!("\nThe length of vec macro is: { }", v.len());
}