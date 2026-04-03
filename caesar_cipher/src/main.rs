use caesar_cipher::{encrypt, decrypt};

fn main() {
    let plaintext = "Hello World! This is a Caesar Cipher.";
    let shift = 5;

    let ciphertext = encrypt(plaintext, shift);
    println!("Ciphertext: {}", ciphertext);

    let decrypted = decrypt(&ciphertext, shift);
    println!("Decrypted: {}", decrypted);
}
