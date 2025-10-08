fn main() {
	//Stating the given parameters as inputs
	let p:f64 = 510000.0;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	//Calculating amount after 3 years
	let a = p * (((1.0)-(r / 100.0)).powf(n));
	println!("The amount is {}", a);

}