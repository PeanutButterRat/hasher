mod sha256;

fn main() {
    sha256::sha256("abc".as_bytes().to_vec());
}