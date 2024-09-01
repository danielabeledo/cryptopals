# Single-byte XOR cipher

The hex encoded string:

```
1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
```

... has been XOR'd against a single character. Find the key, decrypt the message.

You can do this by hand. But don't: write code to do it for you.

How? Devise some method for "scoring" a piece of English plaintext. Character frequency is a good metric. Evaluate each output and choose the one with the best score.

# Solution

Scoring is based on the frequency of the different letters in english - SPACE plus ETAOINSHRDLU

http://www.macfreek.nl/memory/Letter_Distribution

Scoring is normalised and we only use the 12 most used characters

```
cargo run 1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
```

will result in

```
Solution is 'X' with score 94
Output is "Cooking MC\'s like a pound of bacon"
```
