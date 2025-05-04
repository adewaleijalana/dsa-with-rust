#![allow(dead_code, unused_imports)]

mod arrays;

use arrays::custom_array::CustomArray;

fn main() {
    // let mut arr = CustomArray::new();
    // arr.add(1);
    // arr.add(2);
    // arr.add(3);

    // println!("Array length: {}", arr.length());
    // println!("Element at index 1: {:?}", arr.get(1));
    // println!("Element at index 3: {:?}", arr.get(3));

    let str = "Hello, I am Rose!";
    // let rose = "";
    // let reversed = arrays::reverse_str::reverse_str(str);
    let reversed = arrays::reverse_str::reverse_str_2(str);
    println!("Original: {} | Reversed: {}", str, reversed);
}
