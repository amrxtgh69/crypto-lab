use caesar_cipher::{encrypt, frequency_analysis};
fn main() {
    let plaintext = "The writer claimed by a momentary expression, \
    a twitch of a muscle or a glance of an eye, to fathom a man's inmost thoughts. \
    His conclusions were as infallible as so many propositions of Euclid. \
    So startling would his results appear to the uninitiated that until they learned the \
    processes by which he had arrived at them they might well consider him as a necromancer";

    let shift = 18;
    
    let ciphertext = encrypt(plaintext, shift);
    
    println!("{}", "═".repeat(70));
    println!(" CAESAR CIPER CRACKER - FREQUENCY ANALYSIS ATTACK");
    println!("{}", "═".repeat(70));
    println!();
    println!("  📜 Original text: {}", plaintext);
    println!();
    println!("  Encrypted with shift: {}", shift);
    println!("  📝 Ciphertext: {}", ciphertext);
    println!();
    println!("{}", "─".repeat(70));
    println!("  🔓 CRACKING...");
    println!("{}", "─".repeat(70));
    println!();
    let (found_shift, hacked) = frequency_analysis::break_caesar(&ciphertext);
    println!("   SUCCESS!");
    println!();
    println!("  ┌─ Results ─────────────────────────────────────────────┐");
    println!("  │  Found shift:      {:>35} │", found_shift);
    println!("  │  Original plaintext: {:>32} │", plaintext);
    println!("  │  Hacked plaintext:  {:>32} │", hacked);
    println!("  └──────────────────────────────────────────────────────┘");
    println!();
    println!("   Shift found: {} → {}", found_shift, if found_shift == shift { "✓ CORRECT" } else { "✗ WRONG" });
    println!("{}", "═".repeat(70));
}