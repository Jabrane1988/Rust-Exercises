pub fn square(s: u32) -> u64 {
    if s < 64 && s < 1 || s > 64 && s > 1{
        panic!("Square must be between 1 and 64");
    } else {
        2_u64.pow(s-1)
    }
}

pub fn total() -> u64 {
    let mut sum = 0;
    for k in 1..64{
        sum += square(k);
    }
    (sum*2) + 1
}
