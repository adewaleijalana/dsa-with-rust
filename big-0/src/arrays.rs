use std::{collections::{HashMap, HashSet}, fmt::Display, hash::Hash};

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

pub fn all_pairs_in_array(arrs: &[i32], sum: i32) {
    for i in 0..arrs.len() {
        for j in 0..arrs.len() {
            if arrs[i] + arrs[j] == sum {
                println!("Pair: ({}, {})", arrs[i], arrs[j]);
            }
        }
    }
    
}

pub fn all_pairs_in_array_set(arrs: &[i32], sum: i32) -> bool{
    let mut pair_set = HashSet::new();
    for i in 0..arrs.len() {
        // if pair_set.contains(&(sum - arrs[i])) {
        if pair_set.contains(&arrs[i]) {
            println!("Pair: ({}, {})", arrs[i], sum - arrs[i]);
            return true;
        } else {
            // pair_set.insert(arrs[i]);
            pair_set.insert(sum - arrs[i]);
            
        }
    }
    false
    
}