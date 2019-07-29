use std::collections::btree_map::BTreeMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let s_bytes = s.as_bytes();
    let mut max_length_last = 0;
    let mut max_length_curr = 0;
    let mut temp = Vec::new();
    let mut btreemap = BTreeMap::new();
    let mut index = 0;
    let mut last_same_index = 0;
    for item in s_bytes {
        if temp.contains(item) {
            if max_length_curr > max_length_last {
                max_length_last = max_length_curr
            }
            let index_temp = btreemap.get(item).unwrap();
            if *index_temp > last_same_index {
                last_same_index = *index_temp;
            }
            max_length_curr = index - last_same_index;
            btreemap.remove(item);
        } else {
            max_length_curr += 1;
        }
        temp.push(*item);
        btreemap.insert(*item, index);
        index += 1;
    }
    if max_length_last < max_length_curr {
        max_length_last = max_length_curr;
    }
    return max_length_last;
}

pub fn length_of_longest_substring_test() {
    let res = length_of_longest_substring("abcabcbb".to_string());
    println!("length_of_longest_substring: {}", res);


    let res = length_of_longest_substring("dvdf".to_string());
    println!("length_of_longest_substring: {}", res);

    let res = length_of_longest_substring("abba".to_string());
    println!("length_of_longest_substring: {}", res);
}