use std::time::Instant;

fn main() {
    let names = vec!("nemo", "titi", "toto", "nemo", "tata", "nemo");
    let time = Instant::now();
    find_nemo(&names);
    let time_elapsed = time.elapsed();

    println!("Time elapsed: {:?}", time_elapsed);
    println!("Time elapsed: {:?}", time_elapsed.as_millis());
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