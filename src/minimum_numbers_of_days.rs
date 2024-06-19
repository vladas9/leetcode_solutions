pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
    if bloom_day.len() < (m * k) as usize {
        return -1;
    }
    let (mut left, mut right) = (1, *bloom_day.iter().max().unwrap());
    while left < right {
        let mid = left + (right - left) / 2;
        if can_make_bouquets(&bloom_day, m, k, mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}

fn can_make_bouquets(bloom_day: &[i32], m: i32, k: i32, day: i32) -> bool {
    let mut bouquets = 0;
    let mut flowers = 0;
    for &bloom in bloom_day {
        if bloom > day {
            flowers = 0;
            continue;
        }
        flowers += 1;
        if flowers != k {
            continue;
        }
        flowers = 0;
        bouquets += 1;
        if bouquets == m {
            return true;
        }
    }
    false
}
