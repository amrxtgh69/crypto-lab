pub mod frequency_analysis;

pub fn encrypt(plaintext: &str, shift: u8) -> String {
    plaintext
        .chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                let base = b'A';
                (((c as u8 - base + shift) % 26) + base) as char
            } else if c.is_ascii_lowercase() {
                let base = b'a';
                (((c as u8 - base + shift) % 26) + base) as char
            } else {
                c
            }
        })
        .collect()
}

pub fn decrypt(ciphertext: &str, shift: u8) -> String {
    encrypt(ciphertext, 26 - (shift % 26))
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_encrypt_decrypt() {
    //     let plaintext = "Hello Caesar";
    //     let shift = 3;
    //     let ciphertext = encrypt(plaintext, shift);
    //     let decrypted = decrypt(&ciphertext, shift);
    //     assert_eq!(decrypted, plaintext);
    // }

    #[test]
    fn test_break_caesar() {
        let plaintext = "Hello Caesar";
        let shift = 3;
        let ciphertext = encrypt(plaintext, shift);

        let (found_shift, broken) = frequency_analysis::break_caesar(&ciphertext);

        assert_eq!(found_shift, shift);
        assert_eq!(broken, plaintext);
    }
}
