#![allow(dead_code, unused_imports)]

use std::time::Instant;

mod arrays;

use arrays::log_all_pairs_in_array;

fn main() {
    // let names = vec!("nemo", "titi", "toto", "nemo", "tata", "nemo");
    // let time = Instant::now();
    // find_nemo(&names);
    // let time_elapsed = time.elapsed();

    // println!("Time elapsed: {:?}", time_elapsed);
    // println!("Time elapsed: {:?}", time_elapsed.as_millis());

    let arrs = vec![1, 2, 3, 4, 5];

    log_all_pairs_in_array(&arrs);
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