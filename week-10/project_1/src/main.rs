struct Laptop {
    name: String,
    price: u32,
}

impl Laptop {
    fn cost_for(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let hp = Laptop { name: "HP".to_string(), price: 650_000 };
    let ibm = Laptop { name: "IBM".to_string(), price: 755_000 };
    let toshiba = Laptop { name: "Toshiba".to_string(), price: 550_000 };
    let dell = Laptop { name: "Dell".to_string(), price: 850_000 };

    let qty = 3;

    let total =
        hp.cost_for(qty) +
        ibm.cost_for(qty) +
        toshiba.cost_for(qty) +
        dell.cost_for(qty);

    println!("Total cost: {}", total);
}
