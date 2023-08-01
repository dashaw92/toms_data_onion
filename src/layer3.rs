//First 15 bytes of the decrypted layer will be this:
static HEADER: &[u8] = b"==[ Layer 4/6: ";

//Just the first string that stuck out to me in the partially
//decrypted output.
static KNOWN: &[u8] = b"===============";

pub fn process(input: String) -> String {
    //Warning: Do NOT use String#as_bytes. UTF-8 will screw you over (ask me how I know).
    let bytes: Vec<u8> = input.chars().map(|ch| ch as u8).collect();

    let mut key = [0x00; 32];

    //Use the known HEADER to figure out the first 15 bytes of the key.
    HEADER.into_iter()
        .enumerate()
        .map(|(idx, known)| known ^ bytes[idx])
        .enumerate()
        .for_each(|(idx, byte)| key[idx] = byte);

    //Every 32 byte chunk of this output now contains 15 bytes of decrypted payload.
    let partial = try_decode(&key, &bytes);
    
    //Search from the bottom up to find the first instance of `KNOWN`.
    //The decrypted layer output will contain that string on the "Payload" line.
    let known_idx = partial.windows(KNOWN.len())
        .rposition(|slice| slice == KNOWN)
        .expect("Could not find known string in partially decrypted output...");

    //Adjust for the actual starting position, since rposition reports
    //the start of the slice from the above search, not the end.
    let actual_idx = known_idx + KNOWN.len();

    //Only need 17 more bytes of the key. Starting from the first
    //unknown byte to the last of the 32 byte chunk, XOR each byte
    //with '=', since we know the line is all '=' after "==[ Payload ]"
    bytes[actual_idx..actual_idx + 17].into_iter()
        .enumerate()
        .for_each(|(idx, byte)| key[idx + 15] = byte ^ b'=');

    //And decrypt the original again with the full key to solve the layer.
    let decrypted = try_decode(&key, &bytes);
    to_string(&decrypted)
}

//Pair the payload bytes with the infinitely cycled key and decrypt.
//Do not `collect()` to a String; UTF-8 will also screw you over here.
fn try_decode(key: &[u8], payload: &[u8]) -> Vec<u8> {
    payload.into_iter().zip(key.into_iter().cycle())
        .map(|(enc, key)| enc ^ key)
        .collect()
}

//Manually convert a String to a byte array- UTF-8 cannot be trusted!
fn to_string(bytes: &[u8]) -> String {
    bytes.into_iter().map(|byte| *byte as char).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_decode() {
        let key = [0xFE, 0xAA, 0x01, 0x34];
        let payload = "Hello world this is a test".as_bytes();

        let enc = super::try_decode(&key, payload);
        println!("Encoded: \"{enc:?}\"");
        let dec = super::try_decode(&key, &enc);
        println!("Decoded: \"{dec:?}\"");
        assert_eq!(payload, dec);
    }
}