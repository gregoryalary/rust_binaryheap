# Binary heap

This repository contains a simple implementation of the binary heap structure in Rust.

## Example

```rust
let mut heap = build_heap(&[-1, 6, 9, 0]);
heap.add(3);
heap.add(11);
while heap.count() > 0 {
    if let Some(x) = heap.remove() {
        println!("{}", x);
    }
} // will print 11, 9, 6, 3, 0, -1
```
