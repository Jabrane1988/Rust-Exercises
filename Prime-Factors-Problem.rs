pub fn factors(n: u64) -> Vec<u64> {

    let mut value = n.clone();
    
    let mut result = vec![];
    for i in 2..value+1 {
        if value == 1 {
            break;
        }
        while value%i == 0{
            result.push(i);
            value/=i;
        }
    }
    result
    
}
