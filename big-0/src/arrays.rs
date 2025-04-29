use std::{collections::HashMap, fmt::Display, hash::Hash};

pub fn log_all_pairs_array(arrs: &[&[i32]]) {
    for i in 0..arrs.len() {
        for j in 0..arrs.len() {
            println!("Pair: ({}, {})", arrs[i][0], arrs[j][0]);
        }
    }
    
}

pub fn log_all_pairs_in_array<T : Display>(arrs: &[T]) {
    for i in 0..arrs.len() {
        for j in 0..arrs.len() {
            println!("Pair: ({}, {})", arrs[i], arrs[j]);
        }
    }
    
}

pub fn contains_common_items<T : Eq + Hash>(arr1: &[T], arr2: &[T]) -> bool{
  let mut map = HashMap::new();
    for item1 in arr1 {
        map.insert(item1, true);
    }

    for item in arr2 {
        if map.contains_key(item) {
            return true;
        }
    }
    false
}