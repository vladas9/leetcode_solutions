use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut char_map: HashMap<char, usize> = HashMap::new();
    let mut max_length = 0;
    let mut start = 0;

    for (end, c) in s.chars().enumerate() {
        if let Some(&prev_index) = char_map.get(&c) {
            start = start.max(prev_index + 1);
        }
        char_map.insert(c, end);
        max_length = max_length.max(end - start + 1);
    }

    max_length as i32
}
