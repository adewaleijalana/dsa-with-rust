#![allow(dead_code, unused_imports, unused_variables, unused_must_use, unused_mut, clippy::useless_vec)]

use data_structure_lib::arrays::{
    merge_sorted_arrays::merge_sorted_arrays, reverse_str::reverse_str_2,
};
use std::fs::File;
use std::{cell::RefCell, thread};

use data_structure_lib::arrays::custom_array::CustomArray;
use data_structure_lib::custom_iterator::{
    custom_into_iterator, custom_iterator, custom_pixel_into_iterator,
};
use data_structure_lib::linked_list::custom_linked_list::create_custom_link_list;
use data_structure_lib::sorting_alg::selection_sort::selection_sort;
use data_structure_lib::functions::{second_largest, double_numbers, move_zeroes};

use std::collections::HashMap;
use std::process;

fn main() {
    // let file = match File::open("text.txtx") {
    //     Ok(file) => file,
    //     Err(e) => {
    //         eprintln!("Error opening file: {:?}", e);
    //         process::exit(1)
    //     }
    // };

    // let value = map.get(&one).cloned().unwrap_or("Not Found".to_string());

    // map.entry("key".to_string()).or_insert("default".to_string());
    // let mut arr = CustomArray::new();
    // arr.add(1);
    // arr.add(2);
    // arr.add(3);

    // println!("Array length: {}", arr.length());
    // println!("Element at index 1: {:?}", arr.get(1));
    // println!("Element at index 3: {:?}", arr.get(3));

    // let str = "Hello, I am Rose!";
    // // let rose = "";
    // // let reversed = arrays::reverse_str::reverse_str(str);
    // let reversed = arrays::reverse_str::reverse_str_2(str);
    // println!("Original: {} | Reversed: {}", str, reversed);

    // let arr1 = [1, 3, 5, 7];
    // let arr2 = [2, 4];
    // let merged = arrays::merge_sorted_arrays::merge_sorted_arrays(&arr1, &arr2);
    // println!("Merged sorted array: {:?}", merged);

    // custom_iterator();
    // custom_into_iterator();
    // custom_pixel_into_iterator();

    // let mut vec_1 = vec![4, 5, 6, 9, 8];
    // for i in vec_1.iter_mut() { // fix this line by making a call to relevant function
    //     *i = *i * 2;
    // }
    // println!("{:?}", vec_1);

    // let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // println!("{:?}", numbers.pop());
    // let mut result = 0;

    // /* The code in the for loop needs to be replaced */
    // for &num in &numbers {
    //     if num % 2 != 0 {
    //         let squared_num = num * num;
    //         result += squared_num;
    //     }
    // }

    // println!("Result without combinators: {}", result);

    // let result2: i32 = numbers
    // .into_iter()
    // .filter(|&num| num % 2 != 0)
    // .map(|num| num * num)
    // .sum();

    // println!("Result with combinators: {}", result2);

    // test_main();

    // parity();

    // create_custom_link_list();
    // box_test();
    // take_name()
    // sum_thread();

    let mut arr = [0, 64, 0, 4, 025, 12, 0, 22, 11];
    // // selection_sort(&mut arr);
    // // println!("Sorted array: {:?}", arr);
    println!("arrays before moving zeros: {arr:?}");
    move_zeroes(&mut arr);

    println!("arrays after moving zeros: {arr:?}");

    // println!("Second largest element: {}", second_largest(&arr));
    // println!("Double number: {}", double_numbers())
    
}


fn test_main() {
    let multiplier = 2;

    let adder = 5;

    let transform = |x: i32| -> i32 { x * multiplier + adder };

    let result = transform(10);

    println!("Result: {}", result);
}

fn parity() {
    let input = vec![1, 2, 3];

    let parity = input.iter().map(|x| x % 2);

    for p in parity {
        print!("{}", p);
    }
}

fn box_test() {
    let mut stack_var = 4;
    let heap_var = Box::new(stack_var);

    stack_var = 5;
    println!(
        "The value of stack_var = {} and heap_var = {}",
        stack_var, heap_var
    );
}

fn take_name() {
    let mut name = String::from("Alice");
    let taken_name = std::mem::take(&mut name);
    println!("Taken name: {}", taken_name);
    println!("Remaining name: {}", name);
}

// fn ref_borrow() {
//     let count = RefCell::new(0);
//     let borrowed_count = count.borrow();
//     *borrowed_count += 1;
//     println!("Count: {}", borrowed_count);
// }

fn test_thread() {
    let mut thread_vec = vec![];
    for i in 0..10 {
        thread_vec.push(thread::spawn(|| {
            println!("Hi from thread");
        }));
    }

    // The code below will make sure that all the threads go to completion
    for i in thread_vec {
        i.join();
    }
}

fn sum_thread() {
    let handle_1 = thread::spawn(|| {
        let mut sum = 0;
        let range = 0..=1_000;
        for num in range {
            sum += num;
        }
        sum
    }); // Note: The thread spawn returns a joinhandle type. If there is anything returned from
    // closure inside the thread, it will be inside the joinhandle type. In this case, it will be Joinhandle<i32>.
    // You can access the returned i32 value by calling .unwrap() on join.

    // Todo!: Insert a code for creating another thread which will compute the summation from 1001 - 2000
    let handle_2 = thread::spawn(|| {
        let mut sum = 0;
        let range = 1_001..=2_000;
        for num in range {
            sum += num;
        }
        sum
    });

    // Todo!: Insert a code for creating another thread which will compute the summation from 2001 - 3000
    let handle_3 = thread::spawn(|| {
        let mut sum = 0;
        let range = 2_001..=3_000;
        for num in range {
            sum += num;
        }
        sum
    });

    let mut sum = 0;

    // Todo!: Insert code to make sure that the summation is computed correctly.
    // Summation will be computed correctly, if all the threads go to completion.

    sum += handle_1.join().unwrap();
    sum += handle_2.join().unwrap();
    sum += handle_3.join().unwrap();

    println!("Final Summation Result {sum}");

    let v = vec![1, 2, 3];
    let y = v.clone();
    let x = 5;
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", y);
        println!("Here's a variable : {:?}", x);
    });

    println!("The variable x is still alive {}", x);
    println!("The variable v is currenlty not alive {:?}", v);
    println!("Make approperiate changes so that it remains alive on this line");
    handle.join();
}
