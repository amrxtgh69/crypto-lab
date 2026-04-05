use caesar_cipher::{encrypt, decrypt};

fn main() {
    let plaintext = "The writer claimed by a momentary expression, \
    a twitch of a muscle or a glance of an eye, to fathom a man’s inmost thoughts. \
    His conclusions were as infallible as so many propositions of Euclid. \
    So startling would his results appear to the uninitiated that until they learned the \
    processes by which he had arrived at them they might well consider him as a necromancer";

    let shift = 5;

    let ciphertext = encrypt(plaintext, shift);
    println!("Ciphertext: {}", ciphertext);

    let decrypted = decrypt(&ciphertext, shift);
    println!("Decrypted: {}", decrypted);
}
