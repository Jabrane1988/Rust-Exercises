pub fn collatz(n: u64) -> Option<u64> {

    if n == 0{
        return None;
    }
    
    let mut step:u64 = 0;
    let mut number = n.clone();

    while number != 1 {
        step += 1;
        if number%2 == 0 {
            number /= 2;
        } else if number%2 == 1 {
            number = number.checked_mul(3)?.checked_add(1)?;
        }
    }
    Some(step)
}
