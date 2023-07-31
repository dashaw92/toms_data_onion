pub fn process(input: String) -> String {
    input.chars()
        .map(|ch| ch as u8)
        .map(flip_bits)
        .map(rotate_bits)
        .map(|int| int as char)
        .collect()
}

fn flip_bits(int: u8) -> u8 {
    int ^ 0b01010101
}

fn rotate_bits(int: u8) -> u8 {
    let rhb = int & 1;
    let shifted = int >> 1;
    shifted | (rhb << 7)
}