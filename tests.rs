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

    let fail_from_bits = MemoryLayout::from_bits(1025);
    assert_eq!(fail_from_bits, None);
}
#[test]
fn add_layouts() {
    let x = MemoryLayout::from_bytes(5);
    let y = MemoryLayout::from_bytes(10);
    let z = x + y;
    assert_eq!(z.size_bits(), Some(120));
    assert_eq!(z.size_bytes(), 15);

    let large_x = MemoryLayout::from_bytes(usize::MAX / 2);
    let large_y = MemoryLayout::from_bytes(usize::MAX / 2 + 1);//ceiling division, so that they sum to usize::max
    let nearly_overflow = large_x + large_y;
    assert_eq!(nearly_overflow.size_bits(), None);
    assert_eq!(nearly_overflow.size_bytes(), usize::MAX);
}

#[test]
#[should_panic]
fn add_panic() {
    let large_x = MemoryLayout::from_bytes(usize::MAX / 2);
    let large_y = MemoryLayout::from_bytes(usize::MAX - 10);

    let _z = large_x + large_y;
}

#[test]
fn subtract_layouts() {
    let x = MemoryLayout::from_bytes(10);
    let y = MemoryLayout::from_bytes(5);
    let z = x - y;
    assert_eq!(z.size_bits(), Some(40));
    assert_eq!(z.size_bytes(), 5);

    let nearly_underflow = x - y - y;
    assert_eq!(nearly_underflow.size_bits(), Some(0));
    assert_eq!(nearly_underflow.size_bytes(), 0);
}

#[test]
#[should_panic]
fn subtract_panic() {
    let x = MemoryLayout::from_bytes(2);
    let large_y = MemoryLayout::from_bytes(usize::MAX - 10);

    let _z = x - large_y;
}

#[test]
fn equality_checks() {
    let x = MemoryLayout::from_bytes(10);
    let y = MemoryLayout::from_bytes(10);
    let z = MemoryLayout::from_bytes(5);

    assert_eq!(x, y);
    assert_ne!(x, z);

    assert_eq!(x, x);//requirement for Ord
}

#[test]
fn comparison_checks() {
    let x = MemoryLayout::from_bytes(10);
    let y = MemoryLayout::from_bytes(20);
    let z = MemoryLayout::from_bytes(10);

    // PartialOrd checks
    assert!(x < y);
    assert!(y > x);
    assert!(x <= z);
    assert!(x >= z);

    // Ord checks
    assert_eq!(x.cmp(&y), std::cmp::Ordering::Less);
    assert_eq!(y.cmp(&x), std::cmp::Ordering::Greater);
    assert_eq!(x.cmp(&z), std::cmp::Ordering::Equal);
}

#[test]
fn max_checks() {
    let x = MemoryLayout::from_bytes(10);
    let y = MemoryLayout::from_bytes(20);

    let max_layout = x.max(y);
    assert_eq!(max_layout.size_bytes(), 20);

    let max_layout_reverse = y.max(x);
    assert_eq!(max_layout_reverse.size_bytes(), 20);
}

#[test]
fn min_checks() {
    let x = MemoryLayout::from_bytes(10);
    let y = MemoryLayout::from_bytes(20);

    let min_layout = x.min(y);
    assert_eq!(min_layout.size_bytes(), 10);

    let min_layout_reverse = y.min(x);
    assert_eq!(min_layout_reverse.size_bytes(), 10);
}

#[test]
fn clamp_checks() {
    let x = MemoryLayout::from_bytes(15);
    let min = MemoryLayout::from_bytes(10);
    let max = MemoryLayout::from_bytes(20);

    let clamped = x.clamp(min, max);
    assert_eq!(clamped.size_bytes(), 15);

    let clamped_below = MemoryLayout::from_bytes(5).clamp(min, max);
    assert_eq!(clamped_below.size_bytes(), 10);

    let clamped_above = MemoryLayout::from_bytes(25).clamp(min, max);
    assert_eq!(clamped_above.size_bytes(), 20);
}

#[test]
#[should_panic]
fn clamp_panic() {
    let min = MemoryLayout::from_bytes(20);
    let max = MemoryLayout::from_bytes(10);

    let _ = MemoryLayout::from_bytes(15).clamp(min, max);
}

#[test]
fn iterator_sum_layouts() {
    let layouts = vec![
        MemoryLayout::from_bytes(5),
        MemoryLayout::from_bytes(10),
        MemoryLayout::from_bytes(15),
    ];

    let total: MemoryLayout = layouts.iter().cloned().sum();
    assert_eq!(total.size_bits(), Some(240));
    assert_eq!(total.size_bytes(), 30);

    let empty_layouts: Vec<MemoryLayout> = vec![];
    let total_empty: MemoryLayout = empty_layouts.iter().cloned().sum();
    assert_eq!(total_empty.size_bits(), Some(0));
    assert_eq!(total_empty.size_bytes(), 0);

    let large_layouts = vec![
        MemoryLayout::from_bytes(usize::MAX / 2),//floor division
        MemoryLayout::from_bytes(usize::MAX / 2 + 1),//ceiling division, so the sum is usize::MAX
    ];
    let total_large: MemoryLayout = large_layouts.iter().cloned().sum();
    assert_eq!(total_large.size_bits(), None);
    assert_eq!(total_large.size_bytes(), usize::MAX);
}

#[test]
fn add_assign_layouts() {
    let mut x = MemoryLayout::from_bytes(5);
    let y = MemoryLayout::from_bytes(10);
    x += y;
    assert_eq!(x.size_bits(), Some(120));
    assert_eq!(x.size_bytes(), 15);

    let mut large_x = MemoryLayout::from_bytes(usize::MAX / 2);
    let large_y = MemoryLayout::from_bytes(usize::MAX / 2 + 1); // ceiling division, so that they sum to usize::MAX
    large_x += large_y;
    assert_eq!(large_x.size_bits(), None);
    assert_eq!(large_x.size_bytes(), usize::MAX);
}

#[test]
#[should_panic]
fn add_assign_panic() {
    let mut large_x = MemoryLayout::from_bytes(usize::MAX / 2);
    let large_y = MemoryLayout::from_bytes(usize::MAX - 10);

    large_x += large_y;
}

#[test]
fn subtract_assign_layouts() {
    let mut x = MemoryLayout::from_bytes(10);
    let y = MemoryLayout::from_bytes(5);
    x -= y;
    assert_eq!(x.size_bits(), Some(40));
    assert_eq!(x.size_bytes(), 5);

    let mut nearly_underflow = MemoryLayout::from_bytes(10);
    nearly_underflow -= y;
    nearly_underflow -= y;
    assert_eq!(nearly_underflow.size_bits(), Some(0));
    assert_eq!(nearly_underflow.size_bytes(), 0);
}

#[test]
#[should_panic]
fn subtract_assign_panic() {
    let mut x = MemoryLayout::from_bytes(2);
    let large_y = MemoryLayout::from_bytes(usize::MAX - 10);

    x -= large_y;
}
#[test]
fn display_format() {
    let layout = MemoryLayout::from_bytes(1024); // 1 KB
    assert_eq!(format!("{}", layout), "1 kB");

    let layout = MemoryLayout::from_bytes(1048576); // 1 MB
    assert_eq!(format!("{}", layout), "1 MB");

    let layout = MemoryLayout::from_bytes(1073741824); // 1 GB
    assert_eq!(format!("{}", layout), "1 GB");

    let layout = MemoryLayout::from_bytes(10); // 10 Bytes
    assert_eq!(format!("{}", layout), "10 B");

    let layout = MemoryLayout::from_bytes(0); // 0 Bytes
    assert_eq!(format!("{}", layout), "0 B");
}

#[test]
fn debug_format() {
    let layout = MemoryLayout::from_bytes(1024); // 1 KB
    assert_eq!(format!("{:?}", layout), "MemoryLayout { size_bytes: 1024 }");

    let layout = MemoryLayout::from_bytes(0); // 0 Bytes
    assert_eq!(format!("{:?}", layout), "MemoryLayout { size_bytes: 0 }");

    let layout = MemoryLayout::from_bytes(usize::MAX); // Maximum size
    assert_eq!(format!("{:?}", layout), format!("MemoryLayout {{ size_bytes: {} }}", usize::MAX));
}
