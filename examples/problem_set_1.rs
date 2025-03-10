// :dep hex
use base64::prelude::*; // Import Base64 encoding utilities
use hex;
use std::collections::HashMap;

// The variable `hex_string` represents a string of hexadecimal (base 16) numbers;
// each pair represents one byte.
// 1) First, `hex_string` is converted in a vector of raw bytes with the `0x`
// 2) Take 3 bytes (24 bits) at a time.
// 3) Split them into 4 groups of 6 bits each.
// 4) Map each 6-bit value to a Base64 character.

fn convert_hex_to_base64() {
    // Define the input hex string
    let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

    // Decode hex string into raw bytes (Vec<u8>)
    let bs = hex::decode(hex_string).unwrap();
    // println!("{:?} \n Length: {:?}", bs, bs.len()); // Print raw bytes and length

    // Use the base64 crate to encode the original byte array by grouping the raw bytes by three.
    // and converts them into 4 base64 characters.
    let bb = BASE64_STANDARD.encode(bs);
    println!("Solution: {:?}", bb); // Print the final Base64 encoded string

    let solution = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert!(solution == bb);
}

// ===================================================================================

// Challange 2
fn fixed_xor() {
    // Define the input hex string
    let hex_string = "1c0111001f010100061a024b53535009181c";

    // Decode hex string into raw bytes (Vec<u8>)
    let bs = hex::decode(hex_string).unwrap();
    // println!("{:?} \n Length: {:?}", bs, bs.len()); // Print raw bytes and length

    let aa = "686974207468652062756c6c277320657965";
    let aa = hex::decode(aa).unwrap();
    // Bitwise xor and recollect back in a vector.
    let bb: Vec<u8> = bs
        .iter()
        .zip(aa.iter()) // Pair elements from both byte strings
        .map(|(x, y)| x ^ y) // XOR each pair
        .collect(); // Collect results into a Vec<u8>

    let xored_result = hex::encode(bb);
    println!("Solution: {:?}", xored_result); // Print the final Base64 encoded string

    let solution = "746865206b696420646f6e277420706c6179";
    assert!(solution == xored_result);
}

// ===================================================================================

// Challange 3
// // Shorter version for `max_score_for_ascii` function.
fn max_score_for_ascii(sentence: &str) -> usize {
    let mut score = 0;
    let eng_alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ ";
    for c in sentence.chars() {
        if eng_alphabet.contains(c) {
            score += 1;
        }
    }
    score
}

// Find the character with the best score for the single 60-character string
// Find the character with the best score for the single 60-character string
fn single_byte_xor_cipher(hex_string: &str) -> Result<(u8, usize, String), std::io::Error> {
    // Decode the hex string to bytes
    let bs = hex::decode(hex_string).unwrap();
    let mut max_score = 0;
    let mut best_sentence = String::new();
    let mut best_char: u8 = 0;
    // Try to XOR with all ASCII characters
    for x in 0..=255 {
        let ir: Vec<u8> = bs.clone().into_iter().map(|a| a ^ x).collect();

        let res = String::from_utf8(ir);

        // Get max score
        if let Ok(decoded) = res {
            let score = max_score_for_ascii(&decoded);

            if score > max_score {
                max_score = score;
                best_sentence = decoded;
                best_char = x;
            }
        }
    }

    Ok((best_char, max_score, best_sentence))
}


// ===================================================================================

// Challange 4
fn single_byte_xor_cipher_for_file() {
    let fp = std::path::Path::new(r"./data/set1chall4.txt");
    assert!(fp.exists());

    let mut scores: HashMap<(u8, usize), String> = HashMap::new();

    let file_content = std::fs::read_to_string(fp);

    if let Ok(fico) = file_content {
        for line in fico.lines() {
            let res = single_byte_xor_cipher(line);
            if let Ok(bs) = res {
                scores.insert((bs.0, bs.1), bs.2);
                // println!("{:?}", bs);
            }
        }

        // println!("{:?}", scores);
        let key_with_max_value = scores.iter().max_by_key(|entry| entry.0.1).unwrap();
        println!("Solution: {:?}", key_with_max_value);
    }
}

// ===================================================================================

fn main() {
    // Challange 1
    println!("Challange 1");
    convert_hex_to_base64();

    // Challange 2
    println!("Challange 2");
    fixed_xor();

    // Challange 3
    println!("Challange 3");
    let sc3 = single_byte_xor_cipher("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    println!("Soution: {:?}", sc3);

    // Challange 4
    println!("Challange 4");
    single_byte_xor_cipher_for_file();
}