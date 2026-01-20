#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_must_use,
    unused_mut,
    clippy::useless_vec
)]

use std::{
    collections::HashMap,
    io::{self, Bytes, Read},
    mem::swap,
};

use hex::{decode, encode};

pub fn double_numbers() -> u8 {
    let input = io::stdin();
    println!("Enter a digit between 0-9");
    let mut num = String::new();
    input.read_line(&mut num).expect("Error reading input");

    let num = num.trim().parse::<u8>().expect("Error passing input");

    let double_digit = num * 2;

    if double_digit >= 10 {
        1 + double_digit % 10
    } else {
        double_digit
    }
}

pub fn second_largest(arr: &[i32]) -> i32 {
    if arr.len() < 2 {
        return -1;
    }

    let mut largest = -1;
    let mut second_largest = -1;

    for num in arr {
        if *num > largest {
            second_largest = largest;
            largest = *num;
        } else if *num < largest && *num > second_largest {
            second_largest = *num;
        }
    }

    // for i in 0..arr.len() {
    //     let a = arr[i];
    //     if a > largest {
    //         second_largest = largest;
    //         largest = a;
    //     } else if a < largest && a > second_largest {
    //         second_largest = a;
    //     }
    // }

    second_largest
}

pub fn move_zeroes(arr: &mut [i32]) {
    let mut non_zero_count = 0;
    for i in 0..arr.len() {
        if arr[i] != 0 {
            // arr.swap(i, non_zero_count);
            let temp = arr[i];
            arr[i] = arr[non_zero_count];
            arr[non_zero_count] = temp;

            non_zero_count += 1;
        }
    }
}

pub fn reverse_array(arr: &mut [i32]) {
    let arr_length = arr.len();

    if arr_length == 0 || arr_length == 1 {
        return;
    }

    if arr_length == 2 {
        let temp = arr[0];
        arr[0] = arr[1];
        arr[1] = temp;
        // arr.swap(0, 1);
        return;
    }

    let n = arr_length / 2;
    for i in 0..n {
        let temp = arr[i];
        arr[i] = arr[arr_length - 1 - i];
        arr[arr_length - 1 - i] = temp;
    }
}

pub fn array_left_rotation(arr: &mut [i32], rot: i32) {
    let len = arr.len();
    if len == 0 || len == 1 {
        return;
    }

    let rotation = rot % len as i32;
    rotate(arr, 0, rotation - 1);
    rotate(arr, rotation, len as i32 - 1);
    rotate(arr, 0, len as i32 - 1);
}

fn rotate(arr: &mut [i32], mut start: i32, mut end: i32) {
    while start < end {
        arr.swap(start as usize, end as usize);
        start += 1;
        end -= 1;
    }
}

pub fn mode(arr: &[i32]) -> i32 {
    //let mut arr = [0, 64, 0, 4, 25, 12, 11, 22, 11, 11];

    // let mut mode = 1;
    // let mut key = 1;

    let mut hash = HashMap::<&i32, i32>::new();
    for i in arr {
        hash.entry(i)
            .and_modify(|e| {
                *e += 1;
            })
            .or_insert(1);

        // *hash.entry(i).or_insert(0) += 1;
    }

    // for i in arr {
    //    if hash.contains_key(i) {
    //        println!("{}", hash[i]);
    //        hash.insert(i, hash[i] + 1);
    //    }else {
    //        hash.insert(i, 1);
    //    }
    // }

    // for (k, v) in  hash.iter(){
    //   println!("key: {} | value: {}", k, v);
    //     if *v > mode {
    //       println!("key: {} | value: {} : mode: {}", k, v, mode);
    //         mode = *v;
    //         key = **k;
    //     }
    // }

    let max_by_key = hash
        .into_iter()
        .max_by_key(|&(k, v)| v)
        .map(|(k, v)| *k)
        .unwrap();

    println!("Mode is now: {}", max_by_key);

    max_by_key
}

pub fn multi_array_ex() {
    let arr1 = [2; 4];

    // let multi_arr = [[0; 4]; 5];

    // for i in 0..multi_arr.len() {
    //     for j in 0..multi_arr[i].len() {
    //         println!("[{i}][{j}]")
    //     }
    // }

    let multi_arr = [[[0; 4]; 4]; 5];
    for i in 0..multi_arr.len() {
        for j in 0..multi_arr[i].len() {
            for k in 0..multi_arr[i][j].len() {
                println!("[{i}][{j}][{k}] | value: {}", multi_arr[i][j][k]);
            }
        }
    }

    let flatten_array = multi_arr.iter().flatten().flatten();
    for val in flatten_array {
        println!("{val}")
    }
}

pub fn next_permutation() {
    // let a = [1, 2, 3, 6, 5, 4];
    // let a = [1, 2, 3, 4];
    let a = [4, 3, 2, 1];
    let n = a.len();
    if n <= 1 {
        return;
    }
    let mut index = 0;
    for i in (0..a.len()).rev() {
        println!("i:{} - value: {}", i, a[i]);
        if a[i] > a[i - 1] {
            println!("true: {}", a[i - 1]);
            index = i - 1;
            break;
        }
    }

    println!("Pivot index: {}", index)
}

pub fn xor_vectors(a: &str, b: &str) -> Vec<u8> {
    let byte_one = decode(a).unwrap();
    let byte_two = decode(b).unwrap();
    byte_one
        .iter()
        .zip(byte_two.iter()) // Pair elements from both slices
        .map(|(&x1, &x2)| x1 ^ x2) // Apply bitwise XOR to each pair
        .collect() // Collect the results into a new Vec<u8>
}

pub fn decode_hex_string(hex_string: &str) {
    let byte_one = decode(hex_string).unwrap();
    println!("{byte_one:?} with length: {}", byte_one.len());

    let value = i64::from_le_bytes(byte_one.try_into().unwrap());

    //1000_000_000
    println!("value: {}", value);
}

pub fn decode_hex_string_(hex_string: &str) {
    let byte_one = decode(hex_string).unwrap();
    println!("{byte_one:?} with length: {}", byte_one.len());

    let mut buf = [0; 8];

    byte_one.as_slice().read_exact(&mut buf);

    let value = i64::from_le_bytes(buf);

    //1000_000_000
    println!("value: {}", value);
}

pub fn compact_size(hex_string: &str) {
    let mut byte_one = decode(hex_string).unwrap();
    println!("{byte_one:?} with length: {}", byte_one.len());

    let mut buf = [0u8; 1];
    let mut bytes = byte_one.as_slice();
    bytes.read_exact(&mut buf);
    let n = buf[0];
    println!("n: {}", n);

    match n {
        0xFF => {
            let mut buf = [0; 8];
            bytes.read_exact(&mut buf);
            let value = u64::from_le_bytes(buf);
            println!("u64 value: {}", value);
        },
        0xFE => {
            let mut buf = [0; 4];
            bytes.read_exact(&mut buf);
            let value = u32::from_le_bytes(buf);
            println!("u32 value: {}", value);
        },
        0xFD => {
            let mut buf = [0; 2];
            bytes.read_exact(&mut buf);
            let value= u16::from_le_bytes(buf);
            println!("u16 value: {}", value);
        },
        n => {
            println!("u8 value: {}", n);
        }
    }
}
