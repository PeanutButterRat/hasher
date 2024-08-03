use crate::sha::*;

const K: [u64; 80]= [
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

pub fn hash(message_bytes: Vec<u8>) -> Vec<u8> {
    let padded_message = pad_message::<u64>(message_bytes);
    let parsed_blocks = parse_blocks(padded_message, 1024);
    transform(parsed_blocks)
}

#[allow(non_snake_case)]
fn transform(blocks: Vec<Vec<u64>>) -> Vec<u8> {
    let mut H: [u64; 8] = [
        0x6a09e667f3bcc908, 0xbb67ae8584caa73b, 0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
        0x510e527fade682d1, 0x9b05688c2b3e6c1f, 0x1f83d9abfb41bd6b, 0x5be0cd19137e2179
    ];
    
    for i in 0..blocks.len() {
        // 1.
        let mut W: Vec<u64> = vec![0; 80];
        for t in 0..16 {
            W[t] = blocks[i][t];
        }
        for t in 16..80 {
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
        for t in 0..80 {
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

    let mut result: Vec<u8> = vec![0; H.len() * 8];

    for (i, word) in H.iter().enumerate() {
        result[i * 8] = (word >> 56) as u8;
        result[i * 8 + 1] = ((word >> 48) & 0xff_u64) as u8;
        result[i * 8 + 2] = ((word >> 40) & 0xff_u64) as u8;
        result[i * 8 + 3] = ((word >> 32) & 0xff_u64) as u8;
        result[i * 8 + 4] = ((word >> 24) & 0xff_u64) as u8;
        result[i * 8 + 5] = ((word >> 16) & 0xff_u64) as u8;
        result[i * 8 + 6] = ((word >> 8) & 0xff_u64) as u8;
        result[i * 8 + 7] = (word & 0xff_u64) as u8;
    }

    result
}

fn rotr(x: u64, n: u64) -> u64 {
    (x >> n) | (x << (64 - n))
}

fn shr(x: u64, n: u64) -> u64 {
    x >> n
}

fn ch(x: u64, y: u64, z: u64) -> u64 {
    (x & y) ^ (!x & z)
}

fn maj(x: u64, y: u64, z: u64) -> u64 {
    (x & y) ^ (x & z) ^ (y & z)
}

#[allow(non_snake_case)]
fn epsilon0(x: u64) -> u64 {
    rotr(x, 28) ^ rotr(x, 34) ^ rotr(x, 39)
}

#[allow(non_snake_case)]
fn epsilon1(x: u64) -> u64 {
    rotr(x, 14) ^ rotr(x, 18) ^ rotr(x, 41)
}

fn sigma0(x: u64) -> u64 {
    rotr(x, 1) ^ rotr(x, 8) ^ shr(x, 7)
}

fn sigma1(x: u64) -> u64 {
    rotr(x, 19) ^ rotr(x, 61) ^ shr(x, 6)
}

// Output was tested against the SHA tool from https://emn178.github.io/online-tools/sha256.html.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abc() {
        let message = "abc".as_bytes().to_vec();
        let expected = "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f";
        let actual = hash(message);

        assert_eq!(hex::encode(actual), expected)
    }

    #[test]
    fn alphabet() {
        let message = "abcedefghijklmnopqrstuvwxyz".as_bytes().to_vec();
        let expected = "30eb6a75f0a781508e380fae61a8dc7040b660e40cc6fb8ab7e706e519ff70b8aa97904d8092c1dd881c26265afd536f4011d0ecd286175a60b0e71bde44d629";
        let actual = hash(message);

        assert_eq!(hex::encode(actual), expected)
    }

    #[test]
    fn random() {
        let message = "6d1e72ad03ddeb5de891e572e2396f8da015d899ef0e79503152d6010a3fe6916d1e72ad03ddeb5de891e572e2396f8da015d899ef0e79503152d6010a3fe6916d1e72ad03ddeb5de891e572e2396f8da015d899ef0e79503152d6010a3fe691".as_bytes().to_vec();
        let expected = "a82213a62f6cc1e41b44fff5fbd6d0be9d5bfd361a595ec70b5a3f13a9522d5584b9e03c987a5a050ab304751c08950326ea988b0d90fe6b9c76f47fc2a0a28a";
        let actual = hash(message);

        assert_eq!(hex::encode(actual), expected)
    }
}