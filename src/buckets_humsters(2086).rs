pub fn minimum_buckets(hamsters: String) -> i32 {
    let mut hamsters: Vec<char> = hamsters.chars().collect();
    let mut buckets_count = 0;
    for i in 0..hamsters.len() {
        if hamsters[i] == 'H' {
            if i > 0 && hamsters[i - 1] == 'B' || i < hamsters.len() - 1 && hamsters[i + 1] == 'B' {
                continue;
            } else {
                if i < hamsters.len() - 1 && hamsters[i + 1] == '.' {
                    hamsters[i + 1] = 'B';
                    buckets_count += 1;
                } else if i > 0 && hamsters[i - 1] == '.' {
                    hamsters[i - 1] = 'B';
                    buckets_count += 1;
                } else {
                    return -1;
                }
            }
        }
    }
    buckets_count
}
