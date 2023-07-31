pub fn process(input: String) -> String {
    let valid_bits: Vec<u8> = input.chars()
        .map(|ch| ch as u8)
        .filter_map(check_parity)
        .flat_map(deconstruct_bits)
        .collect();

    // TODO: This has a remainder for some reason. `chunks_exact` provides a valid
    // output, but I expected `chunks` to also work? Don't know why we have extra data...
    valid_bits.chunks_exact(8)
        .map(construct_bits)
        .map(|int| int as char)
        .collect()
}

fn check_parity(input: u8) -> Option<u8> {
    let num1s = (input >> 1).count_ones();
    if num1s & 1 == input as u32 & 1 {
        Some(input)
    } else {
        None
    }
}

fn deconstruct_bits(input: u8) -> [u8; 7] {
    let mut out = [0; 7];
    out[0] = (input & 0x80) >> 7;
    out[1] = (input & 0x40) >> 6;
    out[2] = (input & 0x20) >> 5;
    out[3] = (input & 0x10) >> 4;
    out[4] = (input & 0x08) >> 3;
    out[5] = (input & 0x04) >> 2;
    out[6] = (input & 0x02) >> 1;
    out
}

fn construct_bits(slice: &[u8]) -> u8 {
    let mut value = 0;
    value |= slice[0] << 7;
    value |= slice[1] << 6;
    value |= slice[2] << 5;
    value |= slice[3] << 4;
    value |= slice[4] << 3;
    value |= slice[5] << 2;
    value |= slice[6] << 1;
    value |= slice[7];
    value
}