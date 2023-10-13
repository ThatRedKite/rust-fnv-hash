use std::collections::HashSet;
use std::num::Wrapping;
use rand::{self, SeedableRng, RngCore};
use rand::rngs::StdRng;


const FNV32_PRIME: Wrapping<u32> = Wrapping(16777619u32);
const FNV32_OFFSET: Wrapping<u32> =  Wrapping(2166136261u32);

const FNV64_PRIME: Wrapping<u64> = Wrapping(1099511628211u64);
const FNV64_OFFSET: Wrapping<u64> =  Wrapping(14695981039346656037u64);

const EMPTY_BYTES: [u8; 1024] = [0u8; 1024];



fn FNV32_A(hash: Wrapping<u32>, byte: &u8) -> Wrapping<u32> {
    let hash = hash ^ Wrapping(*byte as u32);
    hash * FNV32_PRIME

}


fn FNV32(hash: Wrapping<u32>, byte: &u8) -> Wrapping<u32> {
    let hash = hash * FNV32_PRIME;
    hash ^ Wrapping(*byte as u32)
}


fn FNV64_A(hash: Wrapping<u64>, byte: &u8) -> Wrapping<u64> {
    let hash = hash ^ Wrapping(*byte as u64);
    hash * FNV64_PRIME

}

fn FNV64(hash: Wrapping<u64>, byte: &u8) -> Wrapping<u64> {
    let hash = hash * FNV64_PRIME;
    hash ^ Wrapping(*byte as u64)
}

fn main() {
    
    let a = [0xFAFFu32, 0x0F01u32];
    for byte in a {
        let mut flags_u32 = a[1];
        //println!("{:08X}", flags_u32);
        flags_u32 = flags_u32 | (a[0] << 16);
    
        //println!("{:08X}", flags_u32);
        println!("{:04X}", flags_u32 & 0x0000FFFF);
        println!("{:04X}", (flags_u32 & 0xFFFF0000) >> 16);
        
    }

}
