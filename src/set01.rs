extern crate hex;

use utils;
use std::collections::BTreeMap;
use std::str;

pub fn challenge3() -> String {
    let cipher =
        hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
            .unwrap();

    // score: (key, plaintext)
    let mut scores = Vec::new();
    for key in 0..255 {
        let plaintext = utils::xor(&cipher, &vec![key; cipher.len()]);
        let plaintext = match str::from_utf8(&plaintext) {
            Ok(text) => String::from(text),
            Err(_) => continue,
        };
        let score = utils::score(plaintext);
        scores.push((score, key, plaintext));
    }

    scores.sort_by_key(|item| item.0);
    let plaintext = scores.last().unwrap().clone().2;
    String::from(plaintext)
}
