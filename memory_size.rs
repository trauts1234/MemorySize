use std::ops::{Add, AddAssign};



#[derive(Debug, PartialEq, Clone)]
pub struct MemoryLayout {
    positive: bool,//whether this is a positive size, or perhaps a negative offset
    size_bits: isize
}

impl MemoryLayout {
    pub fn safe_cast_to_unsigned(x: isize) -> usize {
        if x.is_negative() {
            panic!("tried to cast negative number to unsigned")
        } else{
            x as usize
        }
    }
    pub fn safe_cast_to_signed(x: usize) -> isize{
        if x > isize::MAX.try_into().unwrap() {
            panic!("tried to cast too large number to signed")
        } else {
            x as isize
        }
    }
    /**
     * Creates a MemorySize with a size of 0
     */
    pub fn new() -> MemoryLayout {
        MemoryLayout{
            positive: true,
            size_bits: 0
        }
    }
    /**
     * Construct a MemorySize from a number of bytes
     */
    pub fn from_bytes(bytes: isize) -> MemoryLayout{
        MemoryLayout{
            positive: true,
            size_bits: bytes*8
        }
    }
    /**
     * Construct a MemorySize from number of bits
     */
    pub fn from_bits(bits: isize) -> MemoryLayout{
        MemoryLayout{
            positive: true,
            size_bits:bits
        }
    }

    /**
     * Calculate the size suggested by this MemorySize in bytes
     */
    pub fn size_bytes(&self) -> isize{
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
    pub fn size_bits(&self) -> isize{
        self.size_bits
    }

}

impl Add for MemoryLayout{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self{
            positive: true,
            size_bits: self.size_bits+rhs.size_bits,
        }
    }
}

impl AddAssign for MemoryLayout{
    fn add_assign(&mut self, rhs: Self) {
        self.size_bits = self.size_bits + rhs.size_bits;
    }
}