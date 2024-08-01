fn rotr(x: u64, n: u64) -> u64 {
    (x >> n) | (x << (32 - n))
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