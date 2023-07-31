static HEADER: &str = "==[ Layer 4/6: ";
static _KNOWN: &str = "==[ Payload ]===============================================";

pub fn process(input: String) -> String {
    let bytes = input.as_bytes();

    let partial: Vec<u8> = HEADER.as_bytes().into_iter()
        .enumerate()
        .map(|(idx, known)| known ^ bytes[idx])
        .collect();


    try_decode(&partial, &input)

}

fn try_decode(key: &[u8], payload: &str) -> String {
    payload.as_bytes().into_iter().zip(key.into_iter().cycle())
        .map(|(enc, key)| enc ^ key)
        .map(|int| int as char)
        .collect()
}