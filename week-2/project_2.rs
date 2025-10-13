fn main() {
	//Amount of each product
	let amounts = [450_000.0, 1_000_000.0, 750_000.0, 3_000_000.0, 250_000.0];
	//Quantity of each product sold
	let quantity = [2.0, 1.0, 3.0, 4.0, 2.0];

	//Variable to store the total sales amounts
	let mut total = 0.0;

	//Loop throught all products (index 0 to 4)
	for i in 0..quantity.len() {
		//Multiply quantity by price for each product and add the result to the running total
		total += quantity[i] * amounts[i];

	}

//Calculate the average sales amount per product. Converting the qty.len() which gives the number as products as 5 to f64 so it matches the data type of total
let average = total / 12.0 as f64;

//Print the total sales amount
println!("Total Sales Amount = {}", total);

//Print the average sales amount
println!("Average Sales Amount = {}", average);


}