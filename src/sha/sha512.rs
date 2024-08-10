use crate::sha::*;

pub fn hash(message_bytes: Vec<u8>) -> Vec<u8> {
    let padded_message = pad_message::<u64>(message_bytes);
    let parsed_blocks: Vec<Vec<u64>> = parse_blocks(padded_message);
    transform(parsed_blocks, 80)
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