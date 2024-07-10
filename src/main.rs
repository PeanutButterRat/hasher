mod sha256;

fn main() {
    let hash = sha256::hash("abc".as_bytes().to_vec());

    for h in hash {
        print!("{:02x}", h);
    }
}