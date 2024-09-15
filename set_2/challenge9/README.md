# Implement PKCS#7 padding

A block cipher transforms a fixed-sized block (usually 8 or 16 bytes) of plaintext into ciphertext. But 
we almost never want to transform a single block; we encrypt irregularly-sized messages.

One way we account for irregularly-sized messages is by padding, creating a plaintext that is an even 
multiple of the blocksize. The most popular padding scheme is called PKCS#7.

```bash
cargo run "YELLOW SUBMARINE" 20
```