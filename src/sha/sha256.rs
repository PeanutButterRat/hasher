use crate::sha::*;

const INITIAL_HASH: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
    0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19
];

pub fn hash(message_bytes: Vec<u8>) -> Vec<u8> {
    let padded_message = pad_message::<u32>(message_bytes);
    let parsed_blocks: Vec<Vec<u32>> = parse_blocks(padded_message);
    transform(parsed_blocks, INITIAL_HASH, 64)
}

// Output was tested against the SHA tool from https://emn178.github.io/online-tools/sha256.html.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abc() {
        let message = "abc".as_bytes().to_vec();
        let expected = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
        let actual = hex::encode(hash(message));

        assert_eq!(actual, expected)
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