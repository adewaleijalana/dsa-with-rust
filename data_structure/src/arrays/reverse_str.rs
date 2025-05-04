use core::str;

pub fn reverse_str(str: &str) -> String {
    let chars: Vec<char> = str.chars().collect();
    let mut result: Vec<char> = Vec::new();
    for i in (0..chars.len()).rev() {
        result.push(chars[i]);
    }
    result.into_iter().collect()
}

pub fn reverse_str_2(str: &str) -> String {
    str.chars().rev().collect()
}
