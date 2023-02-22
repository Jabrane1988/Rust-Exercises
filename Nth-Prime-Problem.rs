pub fn nth(n: u32) -> u32 {
    let mut current_prime = 2;

    for _ in 0..n{
        let mut candidate = current_prime + 1;
        while(2..candidate).any(|i| candidate % i == 0){
            candidate += 1;
        }
        current_prime = candidate;
    }
    current_prime
}
