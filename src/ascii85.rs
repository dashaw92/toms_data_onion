
pub struct ASCII85 {
    encoded: Vec<char>,
}

impl ASCII85 {
    pub fn new<S: AsRef<str>>(input: S) -> Self {
        let mut input = input.as_ref();
        if input.starts_with("<~") {
            input = input.strip_prefix("<~").unwrap();
        }

        if input.ends_with("~>") {
            input = input.strip_suffix("~>").unwrap();
        }

        let encoded = input.chars().collect();
        Self { encoded }
    }

    /// Takes an input slice ([char; 5]) and decodes it into a 32 bit integer
    fn decode_slice(slice: &[char]) -> u32 {
        slice.into_iter()
            //Convert a char to u32, then remove 33 (counter of encoding)
            .map(|&ch| ch as u32 - 33)
            //The index will be used for powers of 85
            .enumerate()
            //Fold each 32 bit value into a single bit pattern
            .fold(0, |acc, (idx, value)| {
                acc + (85_u32.pow(4 - idx as u32) * value)
            })
    }

    /// Converts a bit pattern into an array of decoded ASCII chars.
    fn bits_to_ascii(bits: u32) -> [char; 4] {
        let mut out = ['\0'; 4];
        
        //Every 8 bits represents a single decoded ASCII char
        out[0] = (bits >> 24 & 0xFF) as u8 as char;
        out[1] = (bits >> 16 & 0xFF) as u8 as char;
        out[2] = (bits >> 8  & 0xFF) as u8 as char;
        out[3] = (bits >> 0  & 0xFF) as u8 as char;
        out
    }

    pub fn decode(mut self) -> String {        
        //Input length must be divisible by 5
        let needed = 5 - self.encoded.len() % 5 + 1;
        while self.encoded.len() % 5 != 0 {
            self.encoded.push('u');
        }

        //Split the input into chunks of 5
        let mut out: String = self.encoded.chunks(5)
            .map(ASCII85::decode_slice)
            .map(ASCII85::bits_to_ascii)
            .flat_map(|chars| chars.into_iter())
            .collect();

        //If padding was added at the start, skip that many bytes from the end
        let _ = out.split_off(out.len() - needed);
        out
    }
}


#[cfg(test)]
mod tests {
    use crate::ASCII85;

    #[test]
    fn test_wikipedia() {
        let test_in: Vec<&str> = include_str!("../test_inputs/ascii85.txt").lines().collect();
        let encoded: String = test_in.iter().take_while(|&&line| line != "---")
            .flat_map(|line| line.chars())
            .filter(|ch| !ch.is_whitespace())
            .collect();

        let expected: String = test_in.iter().skip_while(|&&line| line != "---")
            .skip(1)
            .flat_map(|line| line.chars())
            .collect();

        // dbg!(&expected);
        // dbg!(&encoded);

        // let (encoded, expected) = test_in.lines().
        let asc = ASCII85::new(encoded);
        let out = asc.decode();
        assert_eq!(expected, out);
    }
}
