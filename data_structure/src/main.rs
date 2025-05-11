#![allow(dead_code, unused_imports)]

mod arrays;
mod custom_iterator;
mod hash_tables;

use arrays::custom_array::CustomArray;
use custom_iterator::{custom_iterator, custom_into_iterator, custom_pixel_into_iterator};

fn main() {
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
    custom_pixel_into_iterator();
}
