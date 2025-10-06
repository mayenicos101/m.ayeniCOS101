fn main() {
	//Stating the given parameters as inputs
	let p:f64 = 520000000.0;
	let r:f64 = 10.0;
	let n:f64 = 5.0;

	//Calculating amount
	let a = p * (((1.0)+(r / 100.0)).powf(n));
	println!("The amount is {}", a);

	//Getting the compound interest
	let ci = a - p;
	println!("The compound interest is {}", ci);
}