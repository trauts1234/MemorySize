# Memory Size

`MemorySize` is a Rust library for representing a number of bits or bytes. It provides utilities for working with memory sizes in bytes and bits, as well as performing arithmetic operations, comparisons, and formatting.

## Features

- Create memory sizes from bytes or bits.
- Perform arithmetic operations (`+`, `-`, `+=`, `-=`) on memory sizes.
- Compare memory sizes using `PartialOrd` and `Ord`.
- Pretty-print memory sizes in human-readable formats (e.g. `10 kB`, `1 GB`).
- Clamp, find the maximum, or find the minimum of memory sizes.
- Sum memory sizes from iterators.
- Align memory offsets

### Example

```rust
use memory_size::MemorySize;

fn main() {
    let size1 = MemorySize::from_bytes(1024); // 1 KB
    let size2 = MemorySize::from_bytes(2048); // 2 KB

    let total = size1 + size2;
    println!("Total size: {}", total); // Output: Total size: 3 kB
}
```

## Testing

To run the testing scripts, use:

```bash
cargo test
```