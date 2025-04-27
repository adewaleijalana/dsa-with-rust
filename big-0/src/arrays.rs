use std::fmt::Display;

fn log_all_pairs_array(arrs: &[&[i32]]) {
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