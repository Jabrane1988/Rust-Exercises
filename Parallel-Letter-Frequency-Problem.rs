use std::collections::HashMap;
use std::cmp::min;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut answer = HashMap::<char, usize>::new();
    if input.is_empty(){
        return answer;
    }
    let input = input.join("");
    if input.len() == 0 {
        return answer;
    }

    let mut chern = input.chars();

    let real_worker_count = min(input.len(), worker_count);
    let mut thread_pool = Vec::with_capacity(real_worker_count);
    let mut work_length = (input.len()/real_worker_count).max(1);

    if work_length * real_worker_count < input.len() {
        work_length += 1;
    }

    for _ in 0..real_worker_count {
        let chunk = chern.by_ref().take(work_length).collect::<String>();
        let active_thread = thread::spawn(move || {
           let mut ans = HashMap::<char, usize>::new();
            chunk.chars().for_each(|c| {
                if c.is_alphabetic() {
                    *ans.entry(c.to_ascii_lowercase()).or_default() += 1;
                }
            });
            ans
        }); 
        thread_pool.push(active_thread);
    }

    for thread_active in thread_pool {
        let ans = thread_active.join().unwrap();
        for (key, val) in ans.iter() {
            *answer.entry(*key).or_default() += val;
        }
    }
    answer
}
