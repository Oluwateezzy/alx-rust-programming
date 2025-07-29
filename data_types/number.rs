use std::ops::{Range, RangeInclusive};

// fn main() {
//     let x: i32 = 5;
//     let mut y: u32 = 5;

//     x = y;

//     let z = 10;

//     println!("success!")
// }

fn main() {
    let x: i32 = 5;
    let mut y = 5;

    y = x;

    let z = 10;

    mainB();

    println!("success!")
}

// fn main2() {
//     let v: u16 = 38_u8 as __;

//     println!("success!");
// }

fn main2() {
    let v: u16 = 38_u8 as u16;

    println!("success!");
}

// fn main3() {
//     let x = 5;
//     assert_eq!("u32".to_string(), type_of(&x));

//     println!("success!");
// }

// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>());
// }

fn main3() {
    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));

    println!("success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}


fn main4() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
    println!("success!");
}

fn main5() {
    let v1:i16 = 251_i16 + 8_i16;
    let v2 = i16::checked_add(251, 8).unwrap();
    println!("{} {}",v1,v2);
}

fn main6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);
    println!("success!");
}

fn main7() {
    let x: f64 = 1_000.000_1;
    let y: f32 = 0.12;
    let z: f64 = 0.01_f64;

    assert_eq!("f64".to_string(), type_of(&x));
    println!("success!");
}

fn main8() {
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);
    println!("success!");
}

fn main9() {
    assert!(0.1 as f32 + 0.2_f32 == 0.3_f32);
    println!("success!");
}

fn mainA() {
    let mut sum:i32 = 0;

    for i in -3..2 {
        sum += i
    }
    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}", c as u8);
    }
}

fn mainB() {
    assert_eq!((1..5), Range{start: 1, end: 5});
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("success!");
}

// Fill the blanks and fix the errors
fn main() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);

    assert!(9.6f32 / 3.2f32 == 3.0f32); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5); // 0000 0001 -> 0010 0000
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2); // 0x80 = 128 (8 x 16) -> 1000 0000 -> 0010 0000
                                                   // -> 32 -> 0x20
}