use std::{fs::{read_to_string, File}, io::Write};

mod ascii85;

use ascii85::ASCII85;

fn main() {
    println!("\t*** Toms Data Onion - dashaw92's solver ***");
    println!();
    println!("- Layer 0...");
    layer0();
}

fn layer0() {
    let layer0 = read_to_string("base.txt").expect("Missing base.txt!");
    let encoded: String = find_payload(layer0);
    let ascii85 = ASCII85::new(encoded);
    let out = ascii85.decode();
    let mut layer1 = File::create("layer1.txt").expect("to be able to make files??");
    layer1.write_all(out.as_bytes()).expect("to write to files");
}

fn find_payload(input: String) -> String {
    input
        .lines()
        .skip_while(|line| !line.contains("Payload"))
        .skip(2)
        .flat_map(|line| line.chars())
        .collect()
}