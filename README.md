CF‑B‑tree
# CF-B-tree: Order-Isomorphic Encoding of Continued Fractions for B-Tree Indexing

📄 Read the full paper on Zenodo: https://zenodo.org

=========
A general‑purpose, order‑preserving B‑tree index for arbitrary‑precision real
numbers. CF‑B‑tree encodes each real value using its continued‑fraction
representation and transforms it into a lexicographically sortable binary key
(CFKey). The lexicographic order of CFKey is *mathematically identical* to the
true numerical order of the underlying real number.

This enables:

- O(log N) insertion, lookup, and range queries
- No big‑integer comparison at query time
- Unified indexing of integers, rationals, and floating‑point values
- Correct ordering across heterogeneous precision levels
- Pure bytewise `memcmp` for all comparisons

CF‑B‑tree is suitable for database engines, scientific computing, numerical
libraries, and any system requiring scalable ordering of high‑precision
numerical data.

## Features

- **Order‑preserving**  
  CFKey lexicographic order ≡ real‑number order.

- **Arbitrary precision**  
  Supports integers and rationals of unbounded size.

- **Mixed‑type indexing**  
  Floats, rationals, and integers coexist in the same index.

- **No big‑integer arithmetic**  
  All comparisons reduce to bytewise `memcmp`.

- **Infinity Marker for prefix correctness**  
  Ensures continued‑fraction prefix cases preserve numerical order.

- **Fast**  
  20,000 insertions / lower‑bounds / range queries in a few hundred ms on mobile ARM.

## Example

```rust
use cf_b_tree::{CFIndex, CFInput};

let mut idx = CFIndex::new();

idx.insert_fraction(21, 25);
idx.insert_fraction(23, 25);
idx.insert_float(1.05);

for (_k, (label, val)) in idx.range_query(
    CFInput::Fraction(23,25),
    CFInput::Fraction(28,25),
) {
    println!("{} = {}", label, val);
}

  
