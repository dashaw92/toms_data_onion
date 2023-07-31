use std::{fs::{read_to_string, File}, io::Write};

mod ascii85;
mod layer1;
mod layer2;
mod layer3;

use ascii85::ASCII85;

fn main() {
    println!("\t*** Toms Data Onion - dashaw92's solver ***");
    println!();
    println!("- Layer 0...");
    layer0();
    println!("- Layer 1...");
    layer1();
    println!("- Layer 2...");
    layer2();
    println!("- Layer 3...");
    layer3();
}

/// ASCII85
fn layer0() {
    let solved = decode_ascii85("base.txt");
    write_output("layer1.txt", solved);
}

/// Bit flip and rotation
fn layer1() {
    let ascii85 = decode_ascii85("layer1.txt");
    let solved = layer1::process(ascii85);
    write_output("layer2.txt", solved);
}

/// Parity bits
fn layer2() {
    let ascii85 = decode_ascii85("layer2.txt");
    let solved = layer2::process(ascii85);
    write_output("layer3.txt", solved);
}

/// XOR encryption
fn layer3() {
    let payload = decode_ascii85("layer3.txt");
    let solved = layer3::process(payload);
    write_output("layer4.txt", solved);
}

fn find_payload(input: String) -> String {
    input
        .lines()
        .skip_while(|line| !line.contains("Payload"))
        .skip(2)
        .flat_map(|line| line.chars())
        .filter(|ch| !ch.is_whitespace())
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