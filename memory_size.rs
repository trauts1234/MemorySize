
pub struct MemorySize {
    size_bits: i32
}

impl MemorySize {
    /**
     * construct a memory size from a number of bytes
     */
    pub fn from_bytes(bytes: i32) -> MemorySize{
        MemorySize{
            size_bits: bytes*8
        }
    }
    /**
     * construct a memory size from number of bits
     */
    pub fn from_bits(bits: i32) -> MemorySize{
        MemorySize{
            size_bits:bits
        }
    }

    /**
     * calculate the size suggested by this MemorySize in bytes
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
     * calculate the size suggested by this MemorySize in bits
     */
    pub fn size_bits(&self) -> usize{
        self.size_bits
    }

}