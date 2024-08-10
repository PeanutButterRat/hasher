use crate::sha::*;

const INITIAL_HASH: [u64; 8] = [
    0xcbbb9d5dc1059ed8, 0x629a292a367cd507, 0x9159015a3070dd17, 0x152fecd8f70e5939,
    0x67332667ffc00b31, 0x8eb44a8768581511, 0xdb0c2e0d64f98fa7, 0x47b5481dbefa4fa4,
];

pub fn hash(message_bytes: Vec<u8>) -> Vec<u8> {
    let padded_message = pad_message::<u64>(message_bytes);
    let parsed_blocks: Vec<Vec<u64>> = parse_blocks(padded_message);
    let mut bytes = transform(parsed_blocks, INITIAL_HASH, 80);
    bytes.resize(384 / BITS_PER_BYTE, 0);
    bytes
}

// Output was tested against the SHA tool from https://emn178.github.io/online-tools/sha256.html.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abc() {
        let message = "abc".as_bytes().to_vec();
        let expected = "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7";
        let actual = hex::encode(hash(message));

        assert_eq!(actual, expected)
    }

    #[test]
    fn alphabet() {
        let message = "abcedefghijklmnopqrstuvwxyz".as_bytes().to_vec();
        let expected = "c3de3368cca0f9b93bb33fb9951c215672278a0dd4f761d662e9bfb08c1c341017efbb59cd07deaa217fa106dd929a20";
        let actual = hex::encode(hash(message));

        assert_eq!(actual, expected)
    }

    #[test]
    fn random() {
        let message = "6d1e72ad03ddeb5de891e572e2396f8da015d899ef0e79503152d6010a3fe6916d1e72ad03ddeb5de891e572e2396f8da015d899ef0e79503152d6010a3fe6916d1e72ad03ddeb5de891e572e2396f8da015d899ef0e79503152d6010a3fe691".as_bytes().to_vec();
        let expected = "0abb43cf90db999605dff67b896dd94cd0bbe158242c35fc6f2a99fa203bb2460b7e6173a4ec3b5f571aaf256bee1f93";
        let actual = hex::encode(hash(message));

        assert_eq!(actual, expected)
    }
}