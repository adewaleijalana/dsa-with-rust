use std::{io::{self}, mem::swap};

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

pub fn move_zeroes(arr: &mut [i32]){
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