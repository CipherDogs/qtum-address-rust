# qtum-address-rust

![crates.io](https://img.shields.io/crates/v/qtum-address-rust.svg)
![docs.rs](https://docs.rs/qtum-address-rust/badge.svg)

Rust lib for en/decoding address to Qtum/Ethereum format

# Usage
```rust
use qtum_address_rust::*;

fn main() {
    let addr = "qTTH1Yr2eKCuDLqfxUyBLCAjmomQ8pyrBt";
    
    let qtum = QtumAddress { prefix: 0x78 }; // testnet network prefix
    let eth_addr = qtum.gethexaddress(addr).unwrap(); // 6c89a1a6ca2ae7c00b248bb2832d6f480f27da68
    let qtum_addr = qtum.fromhexaddress(&eth_addr).unwrap(); // qTTH1Yr2eKCuDLqfxUyBLCAjmomQ8pyrBt
}
```
