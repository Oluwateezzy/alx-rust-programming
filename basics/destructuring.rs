// we can use let to extrapolate date from a tuple, list to a separate variable

// fn main() {
//     let (x, y) = (1, 2);
//     x += 2;

//     assert_eq!(x, 3);
//     assert_eq!(y, 2);

//     println!("success!");
// }

fn main() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    main2();

    println!("success!");
}

// destructuring assignments

fn main2() {
    let (x, y);

    (x, ..) = (3, 4);
    [.., y] = [6, 3];

    assert_eq!([x, y], [3, 3]);
}