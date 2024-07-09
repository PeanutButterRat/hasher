const BITS_PER_BLOCK: usize = 512;
const BITS_PER_BYTE: usize = 8;
const BYTES_PER_BLOCK: usize = BITS_PER_BLOCK / BITS_PER_BYTE;
const WORDS_PER_BLOCK: usize = BITS_PER_BLOCK / (BITS_PER_BYTE * 4);

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
];

pub fn sha256(message: Vec<u8>) -> [u32; 8] {
    let bytes = pad(message);
    let blocks = parse(bytes);
    hash(blocks)
}

fn pad(message: Vec<u8>) -> Vec<u8> {
    let l: usize = message.len() * BITS_PER_BYTE;
    let blocks = 1 + l / 512 + if l % 512 >= 448 { 1 } else { 0 };
    let mut bytes: Vec<u8> = vec![0; blocks * BYTES_PER_BLOCK];

    // Copy over the message bytes.
    let mut i: usize = 0;
    for byte in message {
        bytes[i] = byte;
        i += 1;
    }

    bytes[i] = 1 << 7;  // Append the '1' bit after the message.

    // Add the length of the message to the last 64 bits.
    i = bytes.len() - 8;
    for j in (0..8).rev() {
        let mask: usize = 0xff << j * 8;
        let byte = (l & mask) >> j * 8;
        bytes[i] = byte as u8;
        i += 1;
    }

    bytes
}

#[allow(non_snake_case)]
fn parse(bytes: Vec<u8>) -> Vec<Vec<u32>> {
    let N = bytes.len() / BYTES_PER_BLOCK;
    let mut blocks = vec![vec![0; WORDS_PER_BLOCK]; N];
    let mut i: usize = 0;

    for block in 0..N {
        for word in 0..WORDS_PER_BLOCK {
            blocks[block][word] = join_word(bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]);
            i += 4;
        }
    }

    blocks
}

#[allow(non_snake_case)]
fn hash(blocks: Vec<Vec<u32>>) -> [u32; 8] {
    let mut H: [u32; 8] = [  // 5.3.3
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19
    ];
    let N = blocks.len();

    for i in 0..N {
        // 1.
        let mut W: Vec<u32> = vec![0; 64];
        for t in 0..16 {
            W[t] = blocks[i][t];
        }
        for t in 16..64 {
            W[t] = sigma1(W[t - 2])
                .wrapping_add(W[t - 7])
                .wrapping_add(sigma0(W[t - 15]))
                .wrapping_add(W[t - 16]);
        }

        // 2.
        let mut a = H[0];
        let mut b = H[1];
        let mut c = H[2];
        let mut d = H[3];
        let mut e = H[4];
        let mut f = H[5];
        let mut g = H[6];
        let mut h = H[7];

        // 3.
        for t in 0..64 {
            let T1 = h
                .wrapping_add(epsilon1(e))
                .wrapping_add(ch(e, f, g))
                .wrapping_add(K[t])
                .wrapping_add(W[t]);

            let T2 = epsilon0(a).wrapping_add(maj(a, b, c));

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(T1);
            d = c;
            c = b;
            b = a;
            a = T1.wrapping_add(T2);
        }

        // 4.
        H[0] = a.wrapping_add(H[0]);
        H[1] = b.wrapping_add(H[1]);
        H[2] = c.wrapping_add(H[2]);
        H[3] = d.wrapping_add(H[3]);
        H[4] = e.wrapping_add(H[4]);
        H[5] = f.wrapping_add(H[5]);
        H[6] = g.wrapping_add(H[6]);
        H[7] = h.wrapping_add(H[7]);
    }

    H
}

fn join_word(b1: u8, b2: u8, b3: u8, b4: u8) -> u32 {
    ((b1 as u32) << 24) | ((b2 as u32) << 16) | ((b3 as u32) << 8) | (b4 as u32)
}

fn rotr(x: u32, n: u32) -> u32 {
    (x >> n) | (x << (32 - n))
}

fn shr(x: u32, n: u32) -> u32 {
    x >> n
}

fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
}

fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

#[allow(non_snake_case)]
fn epsilon0(x: u32) -> u32 {
    rotr(x, 2) ^ rotr(x, 13) ^ rotr(x, 22)
}

#[allow(non_snake_case)]
fn epsilon1(x: u32) -> u32 {
    rotr(x, 6) ^ rotr(x, 11) ^ rotr(x, 25)
}

fn sigma0(x: u32) -> u32 {
    rotr(x, 7) ^ rotr(x, 18) ^ shr(x, 3)
}

fn sigma1(x: u32) -> u32 {
    rotr(x, 17) ^ rotr(x, 19) ^ shr(x, 10)
}