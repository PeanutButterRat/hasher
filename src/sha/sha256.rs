use crate::sha::*;

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

pub fn hash(message_bytes: Vec<u8>) -> Vec<u8> {
    let padded_message = pad_message::<u32>(message_bytes);
    let parsed_blocks = parse_blocks(padded_message, 512);
    transform(parsed_blocks)
}

#[allow(non_snake_case)]
fn transform(blocks: Vec<Vec<u32>>) -> Vec<u8> {
    let mut H: [u32; 8] = [
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

    let mut result: Vec<u8> = vec![0; H.len() * 4];

    for (i, word) in H.iter().enumerate() {
        result[i * 4] = (word >> 24) as u8;
        result[i * 4 + 1] = ((word >> 16) & 0xff_u32) as u8;
        result[i * 4 + 2] = ((word >> 8) & 0xff_u32) as u8;
        result[i * 4 + 3] = (word & 0xff_u32) as u8;
    }

    result
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

// Output was tested against the SHA tool from https://emn178.github.io/online-tools/sha256.html.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abc() {
        let message = "abc".as_bytes().to_vec();
        let expected = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
        let actual = hash(message);

        assert_eq!(hex::encode(actual), expected)
    }

    #[test]
    fn alphabet() {
        let message = "abcedefghijklmnopqrstuvwxyz".as_bytes().to_vec();
        let expected = "c4fdd9f0589d29bdbf2603e01901380cc3bc9c101dd62ea8da558bc9c516352e";
        let actual = hash(message);

        assert_eq!(hex::encode(actual), expected)
    }

    #[test]
    fn random() {
        let message = "6d1e72ad03ddeb5de891e572e2396f8da015d899ef0e79503152d6010a3fe6916d1e72a\
            d03ddeb5de891e572e2396f8da015d899ef0e79503152d6010a3fe6916d1e72ad03ddeb5de891e572e2396f8\
            da015d899ef0e79503152d6010a3fe691".as_bytes().to_vec();
        let expected = "870b0b44c38b67c4f184fd699677c2a5f0b18f95c1360c95fe2b946695f45c04";
        let actual = hash(message);

        assert_eq!(hex::encode(actual), expected)
    }
}