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

    main3();

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
    let x: i32 = 5;
    assert_eq!("i32".to_string(), type_of(x));

    println!("success!");
}

fn type_of<T>(_: T) -> String {
    format!("{}", std::any::type_name::<T>())
}