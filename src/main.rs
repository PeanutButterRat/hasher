mod sha256;

fn main() {
    for h in sha256::sha256("abc".as_bytes().to_vec()) {
        print!("{:08x}", h);
    }
}