mod tests;

use std::fmt::Display;

use derive_more::{Add, Sub, Sum, AddAssign, SubAssign};
use humansize::{format_size, BaseUnit, FormatSizeOptions, Kilo};

const BITS_IN_BYTE: u64 = 8;

#[derive(
    PartialEq, PartialOrd, Ord, Eq,
    Clone, Copy, Hash, Debug,
    Add, Sub, Sum, AddAssign, SubAssign
)]
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