use std::io;
use num_complex::Complex;

fn main() {
    println!("Make sure your quadratic equation in the form of ax² + bx + c");

    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter coefficent a: ");
    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a:f64 = a.trim().parse().expect("Not a valid number");
    
    println!("Enter coefficent b: ");
    io::stdin().read_line(&mut b).expect("Failed to read input");
    let b:f64 = b.trim().parse().expect("Not a valid number");

    println!("Enter coefficent c: ");
    io::stdin().read_line(&mut c).expect("Failed to read input");
    let c:f64 = c.trim().parse().expect("Not a valid number");

    if a == 0.0 {
        println!("This is not a quadratic equation");
        return;
    }
    let d_real = b.powf(2.0) - 4.0 * a * c;
    let discriminant:Complex<f64> = Complex::new(d_real, 0.0);
    println!("\nDiscriminant (D) = {}", discriminant);

    let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
    let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Roots of the quadratic equation are {:.3} and {:.3}", root1, root2);

    if d_real > 0.0 {
            println!("\nThe equation has two real and distinct roots.");
        }
        else if d_real == 0.0 {
            println!("\nThe equation has real and equal roots.");
        }
    else {
        println!("\nThe equation has complex conjugate roots.");
    }
}