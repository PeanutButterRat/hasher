// Developed using the specification published by NIST.
// https://csrc.nist.gov/files/pubs/fips/180-2/final/docs/fips180-2.pdf

use std::mem;
use std::ops::{BitOrAssign, Shl};

pub mod sha256;
pub mod sha512;

const BITS_PER_BYTE: usize = 8;
const WORDS_PER_BLOCK: usize = 16;

trait Word: Sized + Copy + Default + BitOrAssign + Shl<usize, Output = Self> {
    fn from_byte(byte: u8, shift: usize) -> Self;
}

impl Word for u32 {
    fn from_byte(byte: u8, shift: usize) -> Self {
        (byte as u32) << shift
    }
}

impl Word for u64 {
    fn from_byte(byte: u8, shift: usize) -> Self {
        (byte as u64) << shift
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

fn parse_blocks<T: Word>(padded_message: Vec<u8>, block_size_bits: usize) -> Vec<Vec<T>> {
    let bytes_per_block = block_size_bits / BITS_PER_BYTE;
    let bytes_per_word = mem::size_of::<T>();
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