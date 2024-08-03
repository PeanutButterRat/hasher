// Developed using the specification published by NIST.
// https://csrc.nist.gov/files/pubs/fips/180-2/final/docs/fips180-2.pdf

pub mod sha256;
pub mod sha512;

const BITS_PER_BYTE: usize = 8;

fn pad_message(message_bytes: Vec<u8>, block_size_bits: usize, number_of_bits_reserved_for_message_size: usize) -> Vec<u8> {
    // Calculate the space required for the padded message.
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
