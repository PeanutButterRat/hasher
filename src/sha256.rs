const BITS_PER_BLOCK: usize = 512;
const BITS_PER_BYTE: usize = 8;
const BYTES_PER_BLOCK: usize = BITS_PER_BLOCK / BITS_PER_BYTE;
const WORDS_PER_BLOCK: usize = BITS_PER_BLOCK / (BITS_PER_BYTE * 4);

pub fn sha256(message: Vec<u8>) -> Vec<u8> {
    let bytes = pad(message);
    let blocks = parse(bytes);

    for i in 0..blocks.len() {
        println!("Block: {}", i);
        for (j, word) in blocks[i].iter().enumerate() {
            println!("Word: {}: {:032b}", j, word);
        }
    }

    Vec::new()
}

fn pad(message: Vec<u8>) -> Vec<u8> {
    let l: usize = message.len() * BITS_PER_BYTE;
    let blocks = 1 + l / 512 + if l % 512 >= 448 { 1 } else { 0 };
    let mut bytes: Vec<u8> = vec![0; blocks * BYTES_PER_BLOCK];

    // Copy over the message bytes.
    for (i, byte) in message.iter().enumerate() {
        bytes[i] = *byte;
    }

    bytes[message.len()] = 1 << 7;  // Append the '1' bit after the message.

    // Add the length of the message to the last 64 bits;
    for byte in 0..8 {
        let mask: u8 = !0;
        let index = bytes.len() - 1 - byte;
        bytes[index] = ((l & mask as usize) >> (byte * 8)) as u8;
    }

    bytes
}

#[allow(non_snake_case)]
fn parse(bytes: Vec<u8>) -> Vec<Vec<u32>> {
    let N = bytes.len() / BYTES_PER_BLOCK;
    let mut blocks = vec![vec![0; WORDS_PER_BLOCK]];
    let mut i: usize = 0;

    for block in 0..N {
        for word in 0..WORDS_PER_BLOCK {
            blocks[block][word] = join_word(bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]);
            i += 4;
        }
    }

    blocks
}

fn join_word(b1: u8, b2: u8, b3: u8, b4: u8) -> u32 {
    ((b1 as u32) << 24) | ((b2 as u32) << 16) | ((b3 as u32) << 8) | (b4 as u32)
}