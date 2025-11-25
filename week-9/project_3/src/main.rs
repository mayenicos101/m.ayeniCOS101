use std::fs::File;
use std::io::{Write, Result};

fn main() -> Result<()> {
    // Separate datasets
    let commissioners = vec![
        "Aiqboqun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieve",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // Create output text
    let mut output = String::new();
    output.push_str("S/N | Commissioner Name                | Ministry         | Geopolitical Zone\n");
    output.push_str("----+----------------------------------+------------------+------------------\n");

    for i in 0..commissioners.len() {
        output.push_str(&format!(
            "{:<3} | {:<32} | {:<16} | {}\n",
            i + 1,
            commissioners[i],
            ministries[i],
            zones[i]
        ));
    }

    // Write to file
    let mut file = File::create("merged_output.txt")?;
    file.write_all(output.as_bytes())?;

    println!("merged_output.txt created successfully!");

    Ok(())
}
