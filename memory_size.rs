mod tests;

use std::{fmt::Display, iter::Sum, ops::{Add, AddAssign, Sub, SubAssign}};

use humansize::{format_size, BaseUnit, FormatSizeOptions, Kilo};

const BITS_IN_BYTE: u64 = 8;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct MemorySize {
    size_bits: u64
}

impl MemorySize {
    pub const fn new() -> Self {
        MemorySize { size_bits: 0 }
    }
    /**
     * Construct a MemorySize from a number of bytes
     * if the number of bytes is greater than u64::MAX bits, then it is limited to u64::MAX bits
     */
    pub const fn from_bytes(size_bytes: u64) -> MemorySize{
        MemorySize{
            size_bits: size_bytes.saturating_mul(BITS_IN_BYTE)
        }
    }
    /**
     * Construct a MemorySize from number of bits
     * assuming BITS_IN_BYTE bit bytes
     */
    pub const fn from_bits(size_bits: u64) -> MemorySize {
        MemorySize {
            size_bits
        }
    }

    /**
     * Constructs a MemorySize from a number of bits
     * The resultant MemorySize represents a whole number of bytes, at least enough bytes to store all the bits
     */
    pub const fn from_bits_ceil(bits: u64) -> MemorySize {
        MemorySize {
            size_bits: bits.div_ceil(BITS_IN_BYTE) * BITS_IN_BYTE
        }
    }

    /**
     * Calculate the size suggested by this MemorySize in bytes
     * panics if the number of bits represented is not a multiple of BITS_IN_BYTE
     */
    pub fn size_bytes(&self) -> u64 {
        assert!(self.size_bits % BITS_IN_BYTE == 0);
        self.size_bits / BITS_IN_BYTE
    }

    /**
     * Calculate the size suggested by this MemorySize in bits
     * if the size represented is bigger than usize::MAX bits, then None is returned
     */
    pub fn size_bits(&self) -> u64 {
        self.size_bits
    }

}

impl Add for MemorySize {
    type Output = MemorySize;

    /**
     * adds the size in bytes under the hood
     * so may panic!() on overflow only on debug builds
     */
    fn add(self, rhs: MemorySize) -> MemorySize {
        
        MemorySize::from_bits(
            self.size_bits.checked_add(rhs.size_bits).expect("addition of MemorySize overflowed")
        )
    }
}

impl AddAssign for MemorySize {
    /**
     * finds the total size of self and rhs
     * panics if:
     * size_bits + rhs.size_bits > u64::MAX
     */
    fn add_assign(&mut self, rhs: MemorySize) {
        self.size_bits = self.size_bits.checked_add(rhs.size_bits).expect("addition-assignment of MemorySize overflowed");
    }
}

impl Sub for MemorySize {
    type Output = MemorySize;

    /**
     * finds the differences in size represented by self and rhs
     * panics if:
     * size_bits - rhs.size_bits < 0
     */
    fn sub(self, rhs: MemorySize) -> MemorySize {
        MemorySize::from_bits(
            self.size_bits.checked_sub(rhs.size_bits).expect("subtraction of MemorySize underflowed")
        )
    }
}

impl SubAssign for MemorySize {
    /**
     * finds the differences in size represented by self and rhs
     * panics if:
     * size_bits - rhs.size_bits < 0
     */
    fn sub_assign(&mut self, rhs: MemorySize) {
        self.size_bits = self.size_bits.checked_sub(rhs.size_bits).expect("subtraction-assignment of MemorySize underflowed");
    }
}
impl Sum for MemorySize {
    /**
     * finds the total size represented by the MemorySizes in the iterator
     * panics:
     * if the sum overflows
     */
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(MemorySize::new(), |acc, x| acc + x)
    }
}

impl PartialOrd for MemorySize {
    /**
     * compares the size of each memory layout's size in bytes
     */
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.size_bits.partial_cmp(&other.size_bits)
    }
}
impl Ord for MemorySize {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.size_bits.cmp(&other.size_bits)
    }
    
    /**
     * returns the largest size represented by the two MemorySizes
     */
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::max_by(self, other, Ord::cmp)
    }
    
    /**
     * returns the smallest size represented by the two MemorySizes
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
        MemorySize {
            size_bits: self.size_bits.clamp(min.size_bits, max.size_bits)
        }
    }
}

impl Display for MemorySize {
    /**
     * pretty-prints the MemorySize
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