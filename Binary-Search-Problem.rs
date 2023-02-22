pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let length = array.len();
    if length == 0 {
        None
    }

    else if length == 1 {
        match array[0] == key {
            true => Some(0),
            false => None,
        }
    }
    else{
        let (left, right) = array.split_at(length/2);
        if key >= array[length/2] {
            let found = find(right, key);
            match found {
                None => None,
                Some(n) => Some(length/2 + n)
            }
        }
        else {
            find(left, key)
        }
    }
    
    
}
