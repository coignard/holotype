# Holotype

Generate reproducible names for musical projects and patches.

## Install
```bash
cargo install holotype
```

Or build from source:
```bash
git clone https://github.com/coignard/holotype
cd holotype
cargo build --release
sudo cp target/release/holotype /usr/local/bin/
```

## Usage
```bash
# Generate name for today, number 1
holotype 1

# Generate for specific date
holotype 5 --date 2026-01-15

# Generate with type
holotype 1 --type patch

# Decode name back to date and number
holotype --extract "Cyanokinesum insularis"
```

Example output:
```
$ holotype 1
Cyanokinesum insularis

$ holotype --extract "Monojejunnx sylvaticus" --type patch
Monojejunnx sylvaticus
[patch] No. 1, dated 15.1.2000 (9486 days ago)
```

## How it works

Holotype uses a Feistel network to create a bijective mapping between (date, number, type) tuples and biological binomial names, guaranteeing no collisions.

Let $d$ be a date (2000-2099), $n$ a number (1-99), and $t$ a type string. First, encode:

$$E(d, n) = (y - 2000) \times 10^7 + m \times 10^6 + D \times 10^4 + n$$

where $y$, $m$, $D$ are year, month, day respectively.

Then apply a 4-round Feistel network with type-dependent salt:

$$s = \text{hash}(t), \quad F(x, k) = \text{MixHash}(x + k)$$

$$L_0 = E \gg 32, \quad R_0 = E \land 2^{32}-1$$

For rounds $i \in [1,4]$:

$$L_i = R_{i-1}, \quad R_i = L_{i-1} \oplus F(R_{i-1}, s \times i)$$

Final permuted value: $P = (L_4 \ll 32) | R_4$

The permuted value $P$ deterministically selects morphemes:

$$\text{prefix} = P \bmod |\text{Prefixes}|$$
$$\text{root} = (P \gg 8) \bmod |\text{Roots}|$$  
$$\text{suffix} = (P \gg 16) \bmod |\text{Suffixes}|$$

## Performance

Space complexity: $O(1)$ for generation, $O(n)$ for morpheme tables.

## License

GPL-3.0-or-later
