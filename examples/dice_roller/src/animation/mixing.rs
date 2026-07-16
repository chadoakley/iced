pub fn mix_entropy(bytes: &mut [u8]) {
    let seed: u32 = bytes.iter().fold(0x9E3779B9u32, |acc, &b| {
        acc.wrapping_mul(31).wrapping_add(b as u32)
    });
    for (i, b) in bytes.iter_mut().enumerate() {
        let salt = seed.wrapping_add(i as u32).wrapping_mul(0x85EBCA6B);
        *b = b.wrapping_add((salt >> 16) as u8);
    }
}
