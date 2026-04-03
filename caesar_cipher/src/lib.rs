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

}

