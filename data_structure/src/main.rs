#![allow(dead_code)]

mod arrays;

use arrays::custom_array::CustomArray;

fn main() {
    let mut arr = CustomArray::new();
    arr.add(1);
    arr.add(2);
    arr.add(3);

    println!("Array length: {}", arr.length());
    println!("Element at index 1: {:?}", arr.get(1));
    println!("Element at index 3: {:?}", arr.get(3));
}
