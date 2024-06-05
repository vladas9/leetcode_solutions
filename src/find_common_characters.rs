pub fn common_chars(words: Vec<String>) -> Vec<String> {
    if words.is_empty() {
        return vec![];
    }
    let mut words = words;
    let mut common_c: Vec<String> = words[0].chars().map(|c| c.to_string()).collect();

    for word in words.iter_mut().skip(1) {
        let mut i = 0;
        while i < common_c.len() {
            if !word.contains(&common_c[i]) {
                common_c.remove(i);
            } else {
                let index = word.find(&common_c[i]);
                if let Some(pos) = index {
                    word.remove(pos);
                }
                i += 1;
            }
        }
    }

    common_c
}
