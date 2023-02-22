pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut result: Vec<Vec<u32>> = vec![vec![0; size as usize]; size as usize];

    let mut counter = 1..;
    let loops = ((size as f64)/2.0).ceil() as u32;

    for l in 0..loops {
        let m = size - l -1;
        for j in l..m { result[l as usize][j as usize] = counter.next().unwrap(); }
        for i in l..m { result[i as usize][m as usize] = counter.next().unwrap(); }
        for j in ((l+1)..=m).rev() { result[m as usize][j as usize] = counter.next().unwrap(); }
        for i in ((l+1)..=m).rev() { result[i as usize][l as usize] = counter.next().unwrap(); }    
    }

    if size % 2 == 1{
        result[(size/2) as usize][(size/2) as usize] = size * size;
    }
    result
}
