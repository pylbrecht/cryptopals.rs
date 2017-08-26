extern crate base64;

pub fn hex_to_base64(hex: &str) -> Result<&str, base64::DecodeError> {
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_convert_hex_to_base64() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(hex_to_base64(hex).unwrap(), expected);
    }
}
