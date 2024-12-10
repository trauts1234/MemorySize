use std::ops::{Add, AddAssign};


#[derive(Debug, PartialEq, Clone)]
pub struct MemorySize {
    size_bits: usize
}

impl MemorySize {
    /**
     * Creates a MemorySize with a size of 0
     */
    pub fn new() -> MemorySize {
        MemorySize{
            size_bits: 0
        }
    }
    /**
     * Construct a MemorySize from a number of bytes
     */
    pub fn from_bytes(bytes: usize) -> MemorySize{
        MemorySize{
            size_bits: bytes*8
        }
    }
    /**
     * Construct a MemorySize from number of bits
     */
    pub fn from_bits(bits: usize) -> MemorySize{
        MemorySize{
            size_bits:bits
        }
    }

    /**
     * Calculate the size suggested by this MemorySize in bytes
     */
    pub fn size_bytes(&self) -> usize{
        let rounded_down_ans = self.size_bits/8;
        let remaining_bits = self.size_bits%8;

        //add one if bits are left over, so that there are enough bytes to store all the bits
        if remaining_bits > 0{
            rounded_down_ans + 1
        } else{
            rounded_down_ans
        }
    }

    /**
     * Calculate the size suggested by this MemorySize in bits
     */
    pub fn size_bits(&self) -> usize{
        self.size_bits
    }

}

impl AddAssign for MemorySize{
    fn add_assign(&mut self, rhs: Self) {
        self.size_bits += rhs.size_bits;
    }
}
impl Add for MemorySize{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self{
            size_bits: self.size_bits+rhs.size_bits,
        }
    }
}