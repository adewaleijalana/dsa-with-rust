#![allow(dead_code, unused_imports)]

use std::time::Instant;

mod arrays;

use arrays::{log_all_pairs_in_array, contains_common_items, all_pairs_in_array_set};

fn main() {
    // let names = vec!("nemo", "titi", "toto", "nemo", "tata", "nemo");
    // let time = Instant::now();
    // find_nemo(&names);
    // let time_elapsed = time.elapsed();

    // println!("Time elapsed: {:?}", time_elapsed);
    // println!("Time elapsed: {:?}", time_elapsed.as_millis());

    let arrs = vec![1, 2, 3, 4, 5];

    // log_all_pairs_in_array(&arrs);

    // let arrs1 = vec!['a', 'b', 'c', 'f', 'g'];

    // let arrs2 = vec!['w', 'i', 'z', 't', 'u'];

    // let is_contains = contains_common_items(&arrs1, &arrs2);
    // if is_contains {
    //     println!("Contains common items");
    // } else {
    //     println!("Does not contain common items");
    // }

    let found_pairs = all_pairs_in_array_set(&arrs, 5);
    if found_pairs {
        println!("Contains pairs");
    } else {
        println!("Does not contain pairs");
    }

    
}

fn find_nemo(names: &Vec<&str>){
    for name in names {
        if *name == "nemo" {
            println!("Found Nemo!");
            return;
        }
    }
    println!("Nemo not found");
}