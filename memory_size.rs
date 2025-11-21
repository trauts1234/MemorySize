
//! This crate provides the `MemorySize` type, a wrapper around a memory size represented in bits.
//! It supports basic arithmetic operations and pretty-printing in human-readable formats.
//!
//! ## Usage Example
//!
//! ```rust
//! use memory_size::MemorySize;
//!
//! // Create a MemorySize from bytes
//! let size = MemorySize::from_bytes(1024);
//! println!("Size: {}", size); // prints "1.00 KB" (depending on the chosen format)
//! ```

mod tests;

use std::fmt::Display;

use derive_more::{Add, Sub, Sum, AddAssign, SubAssign};
const BITS_IN_BYTE: u64 = 8;

///This struct represents the size of an area of memory
/// The maximum size possible to be represented is u64::MAX bits (approximately 2.3 exabytes)
#[derive(
    PartialEq, PartialOrd, Ord, Eq,
    Clone, Copy, Hash, Debug, Default,
    Add, Sub, Sum, AddAssign, SubAssign
)]
pub struct MemorySize {
    size_bits: u64
}

impl MemorySize {
    /// Generates a new `MemorySize` representing a 0 bits sized object
    /// # Examples
    ///
    /// ```
    /// use memory_size::MemorySize;
    ///
    /// let zero = MemorySize::new();
    /// assert_eq!(zero.size_bits(), 0);
    /// ```
    pub const fn new() -> Self {
        MemorySize { size_bits: 0 }
    }
    
    ///Construct a `MemorySize`` from a number of bytes
    /// 
    /// # Panics
    /// If the multiplication of bytes by 8 would overflow `u64`
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size::MemorySize;
    ///
    /// let size = MemorySize::from_bytes(128);
    /// assert_eq!(size.size_bits(), 1024);
    /// ```
    pub const fn from_bytes(size_bytes: u64) -> MemorySize{
        MemorySize{
            size_bits: size_bytes.checked_mul(BITS_IN_BYTE).unwrap()
        }
    }
    
    /// Constructs a `MemorySize` directly from a number of bits.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size::MemorySize;
    ///
    /// let size = MemorySize::from_bits(512);
    /// assert_eq!(size.size_bits(), 512);
    /// ```
    pub const fn from_bits(size_bits: u64) -> MemorySize {
        MemorySize {
            size_bits
        }
    }

    
    /// Constructs a `MemorySize` from a number of bits by rounding up to the next whole byte.
    ///
    /// This ensures the internal representation always corresponds to whole bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size::MemorySize;
    ///
    /// // 9 bits rounded up to the nearest byte becomes 16 bits (2 bytes)
    /// let size = MemorySize::from_bits_ceil(9);
    /// assert_eq!(size.size_bits(), 16);
    /// ```
    pub const fn from_bits_ceil(bits: u64) -> MemorySize {
        MemorySize {
            size_bits: bits.div_ceil(BITS_IN_BYTE) * BITS_IN_BYTE
        }
    }

    
    /// Returns the size in bytes.
    ///
    /// # Panics
    ///
    /// if `size_bits` is not a whole number of bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size::MemorySize;
    ///
    /// let size = MemorySize::from_bytes(64);
    /// assert_eq!(size.size_bytes(), 64);
    /// ```
    pub fn size_bytes(&self) -> u64 {
        assert!(self.size_bits % BITS_IN_BYTE == 0);
        self.size_bits / BITS_IN_BYTE
    }

    
    /// Returns the size in bits.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size::MemorySize;
    ///
    /// let size = MemorySize::from_bits(256);
    /// assert_eq!(size.size_bits(), 256);
    /// ```
    pub fn size_bits(&self) -> u64 {
        self.size_bits
    }

    /// Calculates the size in numbers of bits and bytes
    /// 
    /// # Examples
    ///
    /// ```
    /// use memory_size::MemorySize;
    ///
    /// let size = MemorySize::from_bits(10);
    /// assert_eq!(size.size_bits_bytes(), (2,1));
    /// ```
    pub fn size_bits_bytes(&self) -> (u64, u64) {
        (self.size_bits % BITS_IN_BYTE, self.size_bits / BITS_IN_BYTE)
    }

    /// Calculates the memory size above or equal to `self` that is aligned to `alignment`
    /// 
    /// i.e returned value is a multiple of alignment
    /// 
    /// # Examples
    ///
    /// ```
    /// use memory_size::MemorySize;
    ///
    /// let size = MemorySize::from_bytes(25);
    /// assert_eq!(size.align_up(MemorySize::from_bytes(4)), MemorySize::from_bytes(28));
    /// ```
    pub fn align_up(&self, alignment: MemorySize) -> MemorySize {
        //address 0 is aligned to everything
        if self.size_bits == 0 {return self.clone();}
        // alignment 0 = no alignment
        if alignment.size_bits == 0 {return self.clone();}

        let too_much = self.size_bits + alignment.size_bits - 1;//go above self
        let size_bits = (too_much / alignment.size_bits) * alignment.size_bits;//round down

        Self { size_bits }
    }

    /// Calculates the minimum number of bytes that can store `&self`
    /// 
    /// under the hood, this calls `self.align_up(MemorySize::from_bytes(1))`
    /// 
    /// # Examples 
    /// ```
    /// use memory_size::MemorySize;
    ///
    /// let size = MemorySize::from_bits(12);
    /// assert_eq!(size.round_up_byte(), MemorySize::from_bytes(2));
    /// ```
    pub fn round_up_byte(&self) -> MemorySize {
        self.align_up(MemorySize::from_bytes(1))
    }

}

impl Display for MemorySize {
    
    /// Formats the `MemorySize` in a human-readable way.
    ///
    /// Uses the `humansize` crate to format the size to two decimal places.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size::MemorySize;
    ///
    /// let size = MemorySize::from_bits(64);
    /// println!("{}", size); // e.g. "64bit"
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}bit", self.size_bits())
    }
}