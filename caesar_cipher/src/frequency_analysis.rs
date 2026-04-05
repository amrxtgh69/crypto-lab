/*
 * What is caesar cipher:
 * C = (P + K) mod 26
 *
 * vigenere cipher: Ci = (Pi + Ki) mod 26
 *  suppose plain text is       : HELLO WORLD
 *  we know this is             : KEYKEYKEYKE
 *  and key is                  : KEY
 * key has length of 3 ciphertext will be :
 * c0 c1 c2 c3 c4 c5 c6 ........
 *
 * now we split into groups to make the caesar cipher:
 *
 * group 0: c0, c3 , c6, c9 .... (same shift K[0]  look at the e.g H -> K and L -> K)
 * group 1: c1, c4, c7, c10 .....
 * group 2: c2, c5, c8, c11 .......
 *
 * so the main attack is find key length:
 * split into group and break each group using frequency analysis
 * combine shift -> get key
 * 
 * here i am implementing just caesar cipher attack brute force: 
 */


//  `ciphertext
//    ↓
// try shift k
//    ↓
// decrypted text
//    ↓
// count letters
//    ↓
// compare with English frequency`

use crate::decrypt;
use core::f64;
use std::usize;

// The frequency of letters in English text.
const ENGLISH_FREQ: [f64; 26] = [
    8.15, 1.44, 2.76, 3.79, 13.11, 2.92, 1.99,
    5.26, 6.35, 0.13, 0.42, 3.39, 2.54,
    7.10, 8.00, 1.98, 0.12, 6.83,
    6.10, 10.47, 2.46, 0.92, 1.54,
    0.17, 1.98, 0.08
];

pub fn break_caesar(ciphertext: &str) -> (u8, String) {
    let shift = find_shift(ciphertext);
    let plaintext = decrypt(ciphertext, shift);
    (shift, plaintext)
}


fn find_shift(ciphertext: &str) -> u8 {
    let mut best_shift = 0;
    let mut best_score = f64::INFINITY;

    for shift in 0..26 {
        let decrypted = decrypt(ciphertext, shift);

        let counts = count_letters(&decrypted);
        let freq = normalize(&counts);

        let s = score(&freq);

        if s < best_score {
            best_score = s;
            best_shift = shift;
        }
    }
    best_shift
}

fn normalize(counts: &[u32; 26]) -> [f64; 26] {
    let total: u32 = counts.iter().sum();
    let mut freq = [0.0; 26];

    for i in 0..26 {
        let freq[i]: f64 = counts[i] as f64 * 100.0 / total as f64;
    }
    freq
}
fn score(freq: &[f64; 26]) -> f64 {
    let mut s = 0.0;
    
    for i in 0..26 {
        let diff = freq[i] - ENGLISH_FREQ[i];
        s += diff * diff;
    }
    s
}
fn count_letters(text: &str) -> [u32; 26] {
    let mut counts = [0; 26];
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let idx = (c.to_ascii_uppercase() as u8 - b'A') as usize;
            counts[idx] += 1;
        }
    }
    counts
}