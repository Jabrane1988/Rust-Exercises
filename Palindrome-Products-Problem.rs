#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    pub fn new(value: u64) -> Option<Palindrome> {
        if is_palindrome(value) {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
       self.0
    }
}

pub fn is_palindrome(value: u64) -> bool {
    if value % 10 == 0 {
        return false;
    }
    let mut reverse = 0;
    let mut result = value;
    while result > 0 {
        reverse = reverse * 10 + result % 10;
        result /= 10;
    }
    value == reverse
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_palindrome = None;
    let mut max_palindrome = None;
    for i in min..=max {
        if i%10 == 0 { continue };
        for j in min..=max{
            if j%10 == 0 { continue };
            let k = i*j;
            if is_palindrome(k){
                if k < min_palindrome.unwrap_or(u64::MAX) {
                    min_palindrome = Some(k);
                }
                if k > max_palindrome.unwrap_or(u64::MIN) {
                    max_palindrome = Some(k)
                } 
            }
        }
    }

    match(min_palindrome, max_palindrome) {
        (Some(min), Some(max)) => Some((Palindrome::new(min).unwrap(), Palindrome::new(max).unwrap())),
        _ => None
    }
    
}
