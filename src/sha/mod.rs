// Developed using the specification published by NIST.
// https://csrc.nist.gov/files/pubs/fips/180-2/final/docs/fips180-2.pdf

use std::fmt::Debug;
use std::mem;
use std::ops::*;

pub mod sha256;
pub mod sha512;
pub mod sha384;

const BITS_PER_BYTE: usize = 8;
const WORDS_PER_BLOCK: usize = 16;

const CONSTANTS: [u64; 80] = [
    0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc,
    0x3956c25bf348b538, 0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118,
    0xd807aa98a3030242, 0x12835b0145706fbe, 0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235, 0xc19bf174cf692694,
    0xe49b69c19ef14ad2, 0xefbe4786384f25e3, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65,
    0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5,
    0x983e5152ee66dfab, 0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4,
    0xc6e00bf33da88fc2, 0xd5a79147930aa725, 0x06ca6351e003826f, 0x142929670a0e6e70,
    0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 0x53380d139d95b3df,
    0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b,
    0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30,
    0xd192e819d6ef5218, 0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8,
    0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f7763e373, 0x682e6ff3d6b2b8a3,
    0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec,
    0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b,
    0xca273eceea26619c, 0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178,
    0x06f067aa72176fba, 0x0a637dc5a2c898a6, 0x113f9804bef90dae, 0x1b710b35131c471b,
    0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc, 0x431d67c49c100d4c,
    0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817
];

trait Word: Sized + Copy + Default + BitOrAssign + Shl<usize, Output = Self> +
    BitAnd<Output = Self> + Not<Output = Self> + BitXor<Output = Self> + Debug {
    fn from_byte(byte: u8, shift: usize) -> Self;
    fn into_bytes(self) -> Vec<u8>;
    fn from_u64(value: u64) -> Self;
    fn wrapping_add(self, other: Self) -> Self;
    fn upper_sigma_0(self) -> Self;
    fn upper_sigma_1(self) -> Self;
    fn lower_sigma_0(self) -> Self;
    fn lower_sigma_1(self) -> Self;
}

impl Word for u32 {
    fn from_byte(byte: u8, shift: usize) -> Self {
        (byte as u32) << shift
    }

    fn into_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(4);

        for i in 1..=4 {
            let shift = 32 - BITS_PER_BYTE * i;
            let byte = (self >> shift & 0xff_u32) as u8;
            bytes.push(byte);
        }

        bytes
    }

    fn from_u64(value: u64) -> Self {
        (value >> 32) as u32
    }

    fn wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }

    fn upper_sigma_0(self) -> Self {
        self.rotate_right(2) ^ self.rotate_right(13) ^ self.rotate_right(22)
    }

    fn upper_sigma_1(self) -> Self {
        self.rotate_right(6) ^ self.rotate_right(11) ^ self.rotate_right(25)
    }

    fn lower_sigma_0(self) -> Self {
        self.rotate_right(7) ^ self.rotate_right(18) ^ self.shr(3)
    }

    fn lower_sigma_1(self) -> Self {
        self.rotate_right(17) ^ self.rotate_right(19) ^ self.shr(10)
    }
}

impl Word for u64 {
    fn from_byte(byte: u8, shift: usize) -> Self {
        (byte as u64) << shift
    }

    fn into_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8);

        for i in 1..=8 {
            let shift = 64 - BITS_PER_BYTE * i;
            let byte = (self >> shift & 0xff_u64) as u8;
            bytes.push(byte);
        }

        bytes
    }

    fn from_u64(value: u64) -> Self {
        value
    }

    fn wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }

    fn upper_sigma_0(self) -> Self {
        self.rotate_right(28) ^ self.rotate_right(34) ^ self.rotate_right(39)
    }

    fn upper_sigma_1(self) -> Self {
        self.rotate_right(14) ^ self.rotate_right(18) ^ self.rotate_right(41)
    }

    fn lower_sigma_0(self) -> Self {
        self.rotate_right(1) ^ self.rotate_right(8) ^ self.shr(7)
    }

    fn lower_sigma_1(self) -> Self {
        self.rotate_right(19) ^ self.rotate_right(61) ^ self.shr(6)
    }
}

fn pad_message<T: Word>(message_bytes: Vec<u8>) -> Vec<u8> {
    // Calculate the space required for the padded message.
    let bytes_per_word: usize = mem::size_of::<T>();
    let block_size_bits = bytes_per_word * WORDS_PER_BLOCK * BITS_PER_BYTE;
    let number_of_bits_reserved_for_message_size = bytes_per_word * BITS_PER_BYTE * 2;
    let message_length_bits = message_bytes.len() * BITS_PER_BYTE;
    let last_block_size = block_size_bits - number_of_bits_reserved_for_message_size;
    let number_of_blocks = if message_length_bits % block_size_bits >= last_block_size {
        2 + message_length_bits / block_size_bits
    } else {
        1 + message_length_bits / block_size_bits
    };
    let bytes_per_block = block_size_bits / BITS_PER_BYTE;
    let mut padded_message: Vec<u8> = vec![0; number_of_blocks * bytes_per_block];

    // Copy over the message bytes.
    let mut index: usize = 0;
    for byte in message_bytes {
        padded_message[index] = byte;
        index += 1;
    }

    padded_message[index] = 0b10000000;  // Append the '1' bit after the message bytes.

    // Set the last bits to the message size.
    let number_of_bytes_reserved_for_message_size = number_of_bits_reserved_for_message_size / BITS_PER_BYTE;
    index = padded_message.len() - number_of_bytes_reserved_for_message_size;

    for size_byte in (0..number_of_bytes_reserved_for_message_size).rev() {
        let mask: u128 = 0xff << size_byte * 8;
        let byte = (message_length_bits as u128 & mask) >> size_byte * 8;
        padded_message[index] = byte as u8;
        index += 1;
    }

    padded_message
}

fn parse_blocks<T: Word>(padded_message: Vec<u8>) -> Vec<Vec<T>> {
    let bytes_per_word = mem::size_of::<T>();
    let block_size_bits = WORDS_PER_BLOCK * bytes_per_word * BITS_PER_BYTE;
    let bytes_per_block = block_size_bits / BITS_PER_BYTE;
    let words_per_block = bytes_per_block / bytes_per_word;
    let number_of_blocks = padded_message.len() / bytes_per_block;
    let mut blocks: Vec<Vec<T>> = Vec::with_capacity(number_of_blocks);
    let mut byte_index: usize = 0;

    for _ in 0..number_of_blocks {
        let mut block: Vec<T> = Vec::with_capacity(words_per_block);

        for _ in 0..words_per_block {
            let mut word: T = T::default();

            for byte in (0..bytes_per_word).rev() {
                let shift_amount = byte * 8;
                let shifted_byte: T = T::from_byte(padded_message[byte_index], shift_amount);
                word |= shifted_byte;
                byte_index += 1;
            }
            block.push(word);
        }
        blocks.push(block);
    }

    blocks
}

fn transform<T: Word>(blocks: Vec<Vec<T>>, initial_hash: [T; 8], message_schedule_size: usize) -> Vec<u8> {
    let mut hash = Vec::with_capacity(initial_hash.len());

    for i in 0..initial_hash.len() {
        hash.push(initial_hash[i])
    }

    for i in 0..blocks.len() {
        // Prepare the message schedule.
        let mut message_schedule = Vec::with_capacity(message_schedule_size);

        for t in 0..16 {
            message_schedule.push(blocks[i][t]);
        }
        for t in 16..message_schedule_size {
            message_schedule.push(
                message_schedule[t - 2].lower_sigma_1()
                .wrapping_add(message_schedule[t - 7])
                .wrapping_add(message_schedule[t - 15].lower_sigma_0())
                .wrapping_add(message_schedule[t - 16])
            );
        }

        // Initialize the working variables.
        let mut a = hash[0];
        let mut b = hash[1];
        let mut c = hash[2];
        let mut d = hash[3];
        let mut e = hash[4];
        let mut f = hash[5];
        let mut g = hash[6];
        let mut h = hash[7];

        // Perform the computation.
        for t in 0..message_schedule_size {
            let t1 = h
                .wrapping_add(e.upper_sigma_1())
                .wrapping_add(ch(e, f, g))
                .wrapping_add(T::from_u64(CONSTANTS[t]))
                .wrapping_add(message_schedule[t]);

            let t2 = a.upper_sigma_0().wrapping_add(maj(a, b, c));

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }

        // Compute the intermediate hash value.
        hash[0] = a.wrapping_add(hash[0]);
        hash[1] = b.wrapping_add(hash[1]);
        hash[2] = c.wrapping_add(hash[2]);
        hash[3] = d.wrapping_add(hash[3]);
        hash[4] = e.wrapping_add(hash[4]);
        hash[5] = f.wrapping_add(hash[5]);
        hash[6] = g.wrapping_add(hash[6]);
        hash[7] = h.wrapping_add(hash[7]);
    }

    let mut bytes: Vec<u8> = Vec::new();

    for word in hash.iter() {
        bytes.extend(word.into_bytes());
    }

    bytes
}

fn ch<T: Word>(x: T, y: T, z: T) -> T {
    (x & y) ^ (!x & z)
}

fn maj<T: Word>(x: T, y: T, z: T) -> T {
    (x & y) ^ (x & z) ^ (y & z)
}
