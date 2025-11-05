#![cfg(test)]

use crate::MemorySize;

#[test]
fn new() {
    let x = MemorySize::new();
    assert_eq!(x.size_bits(), 0);
    assert_eq!(x.size_bytes(), 0);

    const Y_CONST: MemorySize = MemorySize::new();
    assert_eq!(Y_CONST.size_bits(), 0);
    assert_eq!(Y_CONST.size_bytes(), 0);
}

#[test]
fn from_bytes() {
    let x = MemorySize::from_bytes(0);
    assert_eq!(x.size_bits(), 0);
    assert_eq!(x.size_bytes(), 0);

    let y = MemorySize::from_bytes(u64::MAX/8);
    assert_eq!(y.size_bytes(), u64::MAX/8);

    const Z: MemorySize = MemorySize::from_bytes(10);
    assert_eq!(Z.size_bits(), 80);
    assert_eq!(Z.size_bytes(), 10);
}

#[test]
fn from_bits() {
    let x = MemorySize::from_bits(0);
    assert_eq!(x.size_bits(), 0);
    assert_eq!(x.size_bytes(), 0);

    let large_bits = (u64::MAX/8) & !0b0111u64;//small enough to fit, and a multiple of 8
    let y = MemorySize::from_bits(large_bits);
    assert_eq!(y.size_bits(), large_bits);
    assert_eq!(y.size_bytes(), large_bits / 8);

    const Z: MemorySize = MemorySize::from_bits(24);
    assert_eq!(Z.size_bits(), 24);
    assert_eq!(Z.size_bytes(), 3);
}
#[test]
fn add_layouts() {
    let x = MemorySize::from_bytes(5);
    let y = MemorySize::from_bytes(10);
    let z = x + y;
    assert_eq!(z.size_bits(), 120);
    assert_eq!(z.size_bytes(), 15);

    let large_x = MemorySize::from_bits(u64::MAX / 2);
    let large_y = MemorySize::from_bits(u64::MAX / 2 + 1);//ceiling division, so that they sum to u64::max
    let nearly_overflow = large_x + large_y;
    assert_eq!(nearly_overflow.size_bits(), u64::MAX);
    assert_eq!(nearly_overflow.size_bits(), u64::MAX);
}

#[test]
#[should_panic]
fn add_panic() {
    let large_x = MemorySize::from_bits(u64::MAX / 2);
    let large_y = MemorySize::from_bits(u64::MAX - 10);

    let _z = large_x + large_y;
}

#[test]
fn subtract_layouts() {
    let x = MemorySize::from_bytes(10);
    let y = MemorySize::from_bytes(5);
    let z = x - y;
    assert_eq!(z.size_bits(), 40);
    assert_eq!(z.size_bytes(), 5);

    let nearly_underflow = x - y - y;
    assert_eq!(nearly_underflow.size_bits(), 0);
    assert_eq!(nearly_underflow.size_bytes(), 0);
}

#[test]
#[should_panic]
fn subtract_panic() {
    let x = MemorySize::from_bytes(2);
    let large_y = MemorySize::from_bits(u64::MAX - 10);

    let _z = x - large_y;
}

#[test]
fn equality_checks() {
    let x = MemorySize::from_bytes(10);
    let y = MemorySize::from_bytes(10);
    let z = MemorySize::from_bytes(5);

    assert_eq!(x, y);
    assert_ne!(x, z);

    assert_eq!(x, x);//requirement for Ord
}

#[test]
fn comparison_checks() {
    let x = MemorySize::from_bytes(10);
    let y = MemorySize::from_bytes(20);
    let z = MemorySize::from_bytes(10);

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
    let x = MemorySize::from_bytes(10);
    let y = MemorySize::from_bytes(20);

    let max_layout = x.max(y);
    assert_eq!(max_layout.size_bytes(), 20);

    let max_layout_reverse = y.max(x);
    assert_eq!(max_layout_reverse.size_bytes(), 20);
}

#[test]
fn min_checks() {
    let x = MemorySize::from_bytes(10);
    let y = MemorySize::from_bytes(20);

    let min_layout = x.min(y);
    assert_eq!(min_layout.size_bytes(), 10);

    let min_layout_reverse = y.min(x);
    assert_eq!(min_layout_reverse.size_bytes(), 10);
}

#[test]
fn clamp_checks() {
    let x = MemorySize::from_bytes(15);
    let min = MemorySize::from_bytes(10);
    let max = MemorySize::from_bytes(20);

    let clamped = x.clamp(min, max);
    assert_eq!(clamped.size_bytes(), 15);

    let clamped_below = MemorySize::from_bytes(5).clamp(min, max);
    assert_eq!(clamped_below.size_bytes(), 10);

    let clamped_above = MemorySize::from_bytes(25).clamp(min, max);
    assert_eq!(clamped_above.size_bytes(), 20);
}

#[test]
#[should_panic]
fn clamp_panic() {
    let min = MemorySize::from_bytes(20);
    let max = MemorySize::from_bytes(10);

    let _ = MemorySize::from_bytes(15).clamp(min, max);
}

#[test]
fn iterator_sum_layouts() {
    let layouts = vec![
        MemorySize::from_bytes(5),
        MemorySize::from_bytes(10),
        MemorySize::from_bytes(15),
    ];

    let total: MemorySize = layouts.iter().cloned().sum();
    assert_eq!(total.size_bits(), 240);
    assert_eq!(total.size_bytes(), 30);

    let empty_layouts: Vec<MemorySize> = vec![];
    let total_empty: MemorySize = empty_layouts.iter().cloned().sum();
    assert_eq!(total_empty.size_bits(), 0);
    assert_eq!(total_empty.size_bytes(), 0);

    let large_layouts = vec![
        MemorySize::from_bits(u64::MAX / 2),//floor division
        MemorySize::from_bits(u64::MAX / 2 + 1),//ceiling division, so the sum is u64::MAX
    ];
    let total_large: MemorySize = large_layouts.iter().cloned().sum();
    assert_eq!(total_large.size_bits(), u64::MAX);
}

#[test]
fn add_assign_layouts() {
    let mut x = MemorySize::from_bytes(5);
    let y = MemorySize::from_bytes(10);
    x += y;
    assert_eq!(x.size_bits(), 120);
    assert_eq!(x.size_bytes(), 15);

    let mut large_x = MemorySize::from_bits(u64::MAX / 2);
    let large_y = MemorySize::from_bits(u64::MAX / 2 + 1); // ceiling division, so that they sum to u64::MAX
    large_x += large_y;
    assert_eq!(large_x.size_bits(), u64::MAX);
}

#[test]
#[should_panic]
fn add_assign_panic() {
    let mut large_x = MemorySize::from_bits(u64::MAX / 2);
    let large_y = MemorySize::from_bits(u64::MAX - 10);

    large_x += large_y;
}

#[test]
fn subtract_assign_layouts() {
    let mut x = MemorySize::from_bytes(10);
    let y = MemorySize::from_bytes(5);
    x -= y;
    assert_eq!(x.size_bits(), 40);
    assert_eq!(x.size_bytes(), 5);

    let mut nearly_underflow = MemorySize::from_bytes(10);
    nearly_underflow -= y;
    nearly_underflow -= y;
    assert_eq!(nearly_underflow.size_bits(), 0);
    assert_eq!(nearly_underflow.size_bytes(), 0);
}

#[test]
#[should_panic]
fn subtract_assign_panic() {
    let mut x = MemorySize::from_bytes(2);
    let large_y = MemorySize::from_bytes(u64::MAX - 10);

    x -= large_y;
}
#[test]
fn display_format() {
    let layout = MemorySize::from_bytes(1024); // 1 KB
    assert_eq!(format!("{}", layout), "1 kB");

    let layout = MemorySize::from_bytes(1048576); // 1 MB
    assert_eq!(format!("{}", layout), "1 MB");

    let layout = MemorySize::from_bytes(1073741824); // 1 GB
    assert_eq!(format!("{}", layout), "1 GB");

    let layout = MemorySize::from_bytes(10); // 10 Bytes
    assert_eq!(format!("{}", layout), "10 B");

    let layout = MemorySize::from_bytes(0); // 0 Bytes
    assert_eq!(format!("{}", layout), "0 B");
}

#[test]
fn debug_format() {
    let layout = MemorySize::from_bytes(1024); // 1 KB
    assert_eq!(format!("{:?}", layout), "MemorySize { size_bits: 8192 }");

    let layout = MemorySize::from_bytes(0); // 0 Bytes
    assert_eq!(format!("{:?}", layout), "MemorySize { size_bits: 0 }");

    let layout = MemorySize::from_bits(u64::MAX); // Maximum size
    assert_eq!(format!("{:?}", layout), format!("MemorySize {{ size_bits: {} }}", u64::MAX));
}

#[test]
fn from_bits_ceil() {
    let x = MemorySize::from_bits_ceil(0);
    assert_eq!(x.size_bytes(), 0);

    let y = MemorySize::from_bits_ceil(1); // 1 bit requires 1 byte
    assert_eq!(y.size_bytes(), 1);

    let z = MemorySize::from_bits_ceil(15); // 15 bits require 2 bytes
    assert_eq!(z.size_bytes(), 2);

    let large_bits = u64::MAX - 7; // Largest number of bits fitting into u64
    let large = MemorySize::from_bits_ceil(large_bits);
    assert_eq!(large.size_bytes(), (large_bits + 7) / 8); // Ceiling division
}

#[test]
fn test_default() {
    let x = MemorySize::default();
    assert_eq!(x.size_bits(), 0);
}

#[test]
fn test_align_up() {
    let edge_cases: Vec<_> = 
        [0,1,8,64,67].into_iter()
        .map(|x| MemorySize::from_bits(x))
        .collect();

    let x = MemorySize::from_bits(67*64);
    for case in edge_cases {
        assert_eq!(MemorySize::new().align_up(case), MemorySize::new());//offset 0 should be aligned to everything

        assert_eq!(case.align_up(MemorySize::new()), case);//everything should be aligned to alignment 0
        assert_eq!(case.align_up(MemorySize::from_bits(1)), case);//and 1 bit aligned

        assert_eq!(case.align_up(case), case);//everything should be aligned to itself

        assert_eq!(x.align_up(case), x);// x is aligned already
    }

    assert_eq!(MemorySize::from_bytes(12).align_up(MemorySize::from_bytes(16)), MemorySize::from_bytes(16));
}

#[test]
fn test_round_up_byte() {
    assert_eq!(MemorySize::new().round_up_byte(), MemorySize::new());

    assert_eq!(MemorySize::from_bytes(3).round_up_byte(), MemorySize::from_bytes(3));
}