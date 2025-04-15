mod tests;

use std::{fmt::Display, iter::Sum, ops::{Add, AddAssign, Sub, SubAssign}};

use humansize::{format_size, BaseUnit, FormatSizeOptions, Kilo};

const BITS_IN_BYTE: usize = 8;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MemoryLayout {
    size_bytes: usize
}

impl MemoryLayout {
    /**
     * Creates a MemoryLayout with a size of 0
     */
    pub const fn new() -> MemoryLayout {
        MemoryLayout{
            size_bytes: 0
        }
    }
    /**
     * Construct a MemoryLayout from a number of bytes
     */
    pub const fn from_bytes(size_bytes: usize) -> MemoryLayout{
        MemoryLayout{
            size_bytes
        }
    }
    /**
     * Construct a MemoryLayout from number of bits
     * assuming 8 bit bytes
     * returns None if the number of bits does not exactly fit a number of bytes, else an owned MemoryLayout representing the size
     */
    pub const fn from_bits(bits: usize) -> Option<MemoryLayout> {
        if bits % BITS_IN_BYTE == 0 {
            Some(MemoryLayout {
                size_bytes: bits/BITS_IN_BYTE
            })
        } else {
            None
        }
    }

    /**
     * Calculate the size suggested by this MemoryLayout in bytes
     */
    pub fn size_bytes(&self) -> usize{
        self.size_bytes
    }

    /**
     * Calculate the size suggested by this MemoryLayout in bits
     * if the size represented is bigger than usize::MAX bits, then None is returned
     */
    pub fn size_bits(&self) -> Option<usize> {
        const MAX_BYTES_STILL_FIT_IN_BITS: usize = usize::MAX / BITS_IN_BYTE;
        if self.size_bytes <= MAX_BYTES_STILL_FIT_IN_BITS {
            Some(self.size_bytes * 8)
        } else {
            None
        }
    }

}

impl Add for MemoryLayout {
    type Output = MemoryLayout;

    /**
     * adds the size in bytes under the hood
     * so may panic!() on overflow only on debug builds
     */
    fn add(self, rhs: MemoryLayout) -> MemoryLayout {
        
        MemoryLayout::from_bytes(
            self.size_bytes.checked_add(rhs.size_bytes).expect("addition of MemoryLayout overflowed")
        )
    }
}

impl AddAssign for MemoryLayout {
    /**
     * adds the size in bytes under the hood
     * so may panic!() on overflow only on debug builds
     */
    fn add_assign(&mut self, rhs: MemoryLayout) {
        self.size_bytes = self.size_bytes.checked_add(rhs.size_bytes).expect("addition-assignment of MemoryLayout overflowed");
    }
}

impl Sub for MemoryLayout {
    type Output = MemoryLayout;

    /**
     * subtracts the size in bytes under the hood
     * so may panic!() on underflow only on debug builds
     */
    fn sub(self, rhs: MemoryLayout) -> MemoryLayout {
        MemoryLayout::from_bytes(
            self.size_bytes.checked_sub(rhs.size_bytes).expect("subtraction of MemoryLayout underflowed")
        )
    }
}

impl SubAssign for MemoryLayout {
    /**
     * subtracts the size in bytes under the hood
     * so may panic!() on underflow only on debug builds
     */
    fn sub_assign(&mut self, rhs: MemoryLayout) {
        self.size_bytes = self.size_bytes.checked_sub(rhs.size_bytes).expect("subtraction-assignment of MemoryLayout underflowed");
    }
}
impl Sum for MemoryLayout {
    /**
     * finds the total size represented by the MemoryLayouts in the iterator
     * panics:
     * if the sum overflows
     */
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(MemoryLayout::new(), |acc, x| acc + x)
    }
}

impl PartialOrd for MemoryLayout {
    /**
     * compares the size of each memory layout's size in bytes
     */
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.size_bytes.partial_cmp(&other.size_bytes)
    }
}
impl Ord for MemoryLayout {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.size_bytes.cmp(&other.size_bytes)
    }
    
    /**
     * returns the largest size represented by the two MemoryLayouts
     */
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::max_by(self, other, Ord::cmp)
    }
    
    /**
     * returns the smallest size represented by the two MemoryLayouts
     */
    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::min_by(self, other, Ord::cmp)
    }
    
    /**
     * clamps the size in bytes to between min.size_bytes() and max.size_bytes()
     * panics:
     * if min > max
     */
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        assert!(min <= max);
        MemoryLayout {
            size_bytes: self.size_bytes.clamp(min.size_bytes, max.size_bytes)
        }
    }
}

impl Display for MemoryLayout {
    /**
     * pretty-prints the MemoryLayout
     * uses human-readable formats, like 10GB, 100B
     */
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let options = FormatSizeOptions::default()
            .base_unit(BaseUnit::Byte)
            .kilo(Kilo::Binary)
            .decimal_places(2)
            .space_after_value(true);
        write!(f, "{}", format_size(self.size_bytes(), options))
    }
}