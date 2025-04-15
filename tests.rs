#![cfg(test)]

use std::usize;

use crate::MemoryLayout;

#[test]
fn new() {
    let x = MemoryLayout::new();
    assert_eq!(x.size_bits(), Some(0));
    assert_eq!(x.size_bytes(), 0);

    const Y_CONST: MemoryLayout = MemoryLayout::new();
    assert_eq!(Y_CONST.size_bits(), Some(0));
    assert_eq!(Y_CONST.size_bytes(), 0);
}

#[test]
fn from_bytes() {
    let x = MemoryLayout::from_bytes(0);
    assert_eq!(x.size_bits(), Some(0));
    assert_eq!(x.size_bytes(), 0);

    let y = MemoryLayout::from_bytes(usize::MAX);
    assert_eq!(y.size_bits(), None);
    assert_eq!(y.size_bytes(), usize::MAX);

    const Z: MemoryLayout = MemoryLayout::from_bytes(10);
    assert_eq!(Z.size_bits(), Some(80));
    assert_eq!(Z.size_bytes(), 10);
}

#[test]
fn from_bits() {
    let x = MemoryLayout::from_bits(0).unwrap();
    assert_eq!(x.size_bits(), Some(0));
    assert_eq!(x.size_bytes(), 0);

    let large_bits = (usize::MAX/8) & !0b0111usize;//small enough to fit, and a multiple of 8
    let y = MemoryLayout::from_bits(large_bits).unwrap();
    assert_eq!(y.size_bits(), Some(large_bits));
    assert_eq!(y.size_bytes(), large_bits / 8);

    const Z: MemoryLayout = MemoryLayout::from_bits(24).unwrap();
    assert_eq!(Z.size_bits(), Some(24));
    assert_eq!(Z.size_bytes(), 3);
}