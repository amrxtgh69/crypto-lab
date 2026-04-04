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

use crate::decrypt;
use std::usize;

pub fn break_caesar(ciphertext: &str) -> (u8, String) {
    let shift = find_shift(ciphertext);
    let plaintext = decrypt(ciphertext, shift);
    (shift, plaintext)
}


fn find_shift(ciphertext: &str) -> u8 {
    let counts = count_letters(ciphertext);
    let most_freq = most_frequent_letter(&counts);
    
}

fn count_letters(text: &str) -> [u32; 26] {
    let mut counts = [0; 26];
~/crypto-lab/caesar_cipher git:(main) ❯ tree
.

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let idx = (c.to_ascii_uppercase() as u8 - b'A') as usize;
            counts[idx] += 1;
        }
    }
    counts
}


