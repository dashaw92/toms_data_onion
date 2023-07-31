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
    let solved = decode_ascii85("base.txt");
    write_output("layer1.txt", solved);
}

fn layer1() {
    let ascii85: String = decode_ascii85("layer1.txt");
    let solved = layer1::process(ascii85);
    write_output("layer2.txt", solved);
}

fn find_payload(input: String) -> String {
    input
        .lines()
        .skip_while(|line| !line.contains("Payload"))
        .skip(2)
        .flat_map(|line| line.chars())
        .collect()
}

fn decode_ascii85(path: &str) -> String {
    let file = read_to_string(path).expect("Missing file");
    let encoded: String = find_payload(file);
    ASCII85::new(encoded).decode()
}

fn write_output(path: &str, contents: String) {
    let mut file = File::create(path).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}