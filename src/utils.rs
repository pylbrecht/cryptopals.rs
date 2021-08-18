extern crate base64;
extern crate hex;

pub fn hex_to_base64(hex_string: &str) -> Result<String, hex::FromHexError> {
    hex::decode(hex_string).and_then(|result| Ok(base64::encode(result)))
}

pub fn xor<'a>(a: &'a [u8], b: &'a [u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(&x, &y)| x ^ y).collect()
}

pub fn score(plaintext: &str) -> usize {
    plaintext
        .chars()
        .filter(|&c| "ETAOIN SHRDLU".contains(c))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_convert_hex_to_base64() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(hex_to_base64(hex), Ok(String::from(expected)));
    }

    #[test]
    fn test_xor() {
        let a = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
        let b = hex::decode("686974207468652062756c6c277320657965").unwrap();
        let result = hex::decode("746865206b696420646f6e277420706c6179").unwrap();
        assert_eq!(xor(&a, &b), result);
    }

    #[test]
    fn test_score() {
        assert_eq!(score("ETAOIN SHRDLU"), 13);
        assert_eq!(score("X"), 0);
    }
}
