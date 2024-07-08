const BITS_PER_BLOCK: u64 = 512;
const BITS_PER_BYTE: u64 = 8;
const BYTES_PER_BLOCK: u64 = BITS_PER_BLOCK / BITS_PER_BYTE;

pub fn sha256(message: Vec<u8>) -> Vec<u8> {
    // Padding the message.
    let l: u64 = message.len() as u64 * BITS_PER_BYTE;
    let blocks = 1 + l / 512 + if l % 512 >= 448 { 1 } else { 0 };
    let mut bytes: Vec<u8> = vec![0; (blocks * BYTES_PER_BLOCK) as usize];

    // Copy over the message bytes.
    for (i, byte) in message.iter().enumerate() {
        bytes[i] = *byte;
    }

    bytes[message.len()] = 1 << 7;  // Append the '1' bit after the message.

    // Add the length of the message to the last 64 bits;
    for i in 0..8 {
        let mask: u64 = 0b11111111;
        let index = bytes.len() - 1 - i;
        bytes[index] = ((l & mask) >> (i * 8)) as u8;
    }

    let mut i = 0;
    for byte in bytes {
        println!("{} : {:08b}", i, byte);
        i += 1;
    }

    Vec::new()
}