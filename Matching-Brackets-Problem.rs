pub fn brackets_are_balanced(string: &str) -> bool {
    let mut Brackets: Vec<char> = Vec::new();
    
    for c in string.chars() {
        match c {
            '(' | '{' | '[' => Brackets.push(c),
            ')' => if Brackets.pop() != Some('(') {return false},
            '}' => if Brackets.pop() != Some('{') {return false},
            ']' => if Brackets.pop() != Some('[') {return false},
            _ => ()
        }
    }
    Brackets.is_empty()
}
