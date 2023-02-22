pub fn is_valid(code: &str) -> bool {
  let code: String = code.chars().filter(|c: &char| !c.is_whitespace()).collect();
  if code.len() < 2 || code.chars().any(|c: char| !c.is_ascii_digit()) {
    return false;
  }
  let handle_digit = |(i, c): (usize, char)| {
    let mut d: u32 = c.to_digit(10).unwrap();
    if (code.len() - i) % 2 == 0 {
      d *= 2;
      if d > 9 {
        d -= 9;
      }
    }
    d
  };
  let sum: u32 = code.chars().enumerate().map(handle_digit).sum();
  sum % 10 == 0
}
