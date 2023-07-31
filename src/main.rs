use std::{fs::{read_to_string, File}, io::Write};

mod ascii85;
mod layer1;

use ascii85::ASCII85;

fn main() {
    println!("\t*** Toms Data Onion - dashaw92's solver ***");
    println!();
    println!("- Layer 0...");
    layer0();
    println!("- Layer 1...");
    layer1();
}

fn layer0() {
    let layer0 = read_to_string("base.txt").expect("Missing base.txt!");
    let encoded: String = find_payload(layer0);
    let ascii85 = ASCII85::new(encoded);
    let out = ascii85.decode();
    let mut layer1 = File::create("layer1.txt").unwrap();
    layer1.write_all(out.as_bytes()).unwrap();
}

fn layer1() {
    let layer1 = read_to_string("layer1.txt").expect("Missing layer1.txt!");
    let encoded = find_payload(layer1);
    let ascii85: String = ASCII85::new(encoded).decode();
    let solved = layer1::process(ascii85);
    let mut layer2 = File::create("layer2.txt").unwrap();
    layer2.write_all(solved.as_bytes()).unwrap();
}

fn find_payload(input: String) -> String {
    input
        .lines()
        .skip_while(|line| !line.contains("Payload"))
        .skip(2)
        .flat_map(|line| line.chars())
        .collect()
}