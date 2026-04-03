
# Information Theory and Cryptography: Key Concepts

Information theory, founded by **Claude Shannon**, provides the mathematical backbone for understanding secrecy and security in cryptography.

---

## Entropy (H)

**Entropy** is the measure of uncertainty.

### Meaning of Uncertainty

Imagine a simple game:
- I flip a coin.
- You guess the result.

- If the coin is fair: You have no clue → **maximum uncertainty**. 
- If the coin is rigged (always heads): You already know the result → **zero uncertainty**. 

**Uncertainty**: How surprised are you when you see the outcome?

### How to Measure Uncertainty

We need a way to measure surprise mathematically.

- If something happens with probability 1 (certain) → no surprise → value = 0
- If something happens with probability very small (rare) → very surprising → large value

A function that behaves like this is:

$$\text{surprise} = -\log(p)$$

In cryptography, **high entropy** means a key or plaintext is hard to guess. For a truly random n-bit key, $H = n$ bits.

We say:
- "truly random n-bit key"
- This means: Every key is equally likely
- So: $p_i = \frac{1}{2^n}$ for all $i$

### Mathematical Derivation

For a truly random n-bit key:

1. **Entropy Formula**:
   $$H = -\sum_{i=1}^{2^n} p_i \log_2(p_i)$$

2. **Substitute $p_i$**:
   $$H = -\sum_{i=1}^{2^n} \frac{1}{2^n} \log_2\left(\frac{1}{2^n}\right)$$

3. **Simplify the Logarithm**:
   $$\log_2\left(\frac{1}{2^n}\right) = -n$$

4. **Substitute Back**:
   $$H = -\sum_{i=1}^{2^n} \frac{1}{2^n} \cdot (-n)$$

5. **Factor Out Constants**:
   $$H = -\left( (-n) \cdot \sum_{i=1}^{2^n} \frac{1}{2^n} \right)$$

6. **Evaluate the Sum**:
   $$\sum_{i=1}^{2^n} \frac{1}{2^n} = 1$$

7. **Final Result**:
   $$H = -(-n \cdot 1) = n$$

### Interpretation

- There are $2^n$ equally likely possibilities.
- Entropy becomes: $H = \log_2(2^n) = n$
- **Meaning**: Identifying the correct one requires $n$ bits of information.

---

## Perfect Secrecy and One-Time Pad

### Definitions

- **Plaintext (P)**: The secret message
- **Key (K)**: Secret randomness
- **Ciphertext (C)**: What the attacker sees
- **Encryption**: $C = \text{Enc}(P, K)$

### Demand of Perfect Secrecy

We use entropy:
- Before seeing ciphertext → uncertainty about message = $H(P)$
- After seeing ciphertext → uncertainty becomes $H(P|C)$

**Perfect secrecy** says:
$$H(P|C) = H(P)$$

### One-Time Pad (Vernam Cipher)

- Achieves perfect secrecy if the key is truly random, as long as the plaintext, and used once.

---

## Conditional Entropy

### Definition

- **$H(X)$**: Uncertainty of a random variable $X$
- **$H(X|Y)$**: Uncertainty of $X$ after knowing $Y$

### Formal Definition

$$H(X|Y) = -\sum_{x,y} p(x,y) \log p(x|y)$$

---

## Key Equivocation

### Definition

- **$H(K)$**: Uncertainty about the key before seeing anything
- **$H(K|C)$**: Uncertainty about the key after seeing ciphertext

### Interpretation

- **Key equivocation**: How many possible keys still look valid after seeing the ciphertext

---

## Mutual Information ($I(P; C)$)

### Definition

$$I(P; C) = H(P) - H(P|C)$$

### Interpretation

- **Mutual information**: How much information ciphertext gives about plaintext

---

## Summary Table

| Concept                | Definition                                                                 | Interpretation                                                                 |
|-------------------------|----------------------------------------------------------------------------|--------------------------------------------------------------------------------|
| Entropy ($H(X)$)        | $H(X) = -\sum p(x) \log p(x)$                                           | Measures uncertainty of a random variable $X$                                |
| Conditional Entropy    | $H(X|Y) = -\sum_{x,y} p(x,y) \log p(x|y)$                               | Measures uncertainty of $X$ after knowing $Y$                               |
| Key Equivocation        | $H(K|C)$                                                                   | Measures uncertainty about the key after seeing ciphertext                   |
| Mutual Information      | $I(P; C) = H(P) - H(P|C)$                                                 | Measures information ciphertext gives about plaintext                        |
| Perfect Secrecy        | $H(P|C) = H(P)$                                                            | Ciphertext reveals no information about the plaintext                        |

