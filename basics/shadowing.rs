// defining a new variable with the same as a previous variable, here we can say the first is a shadow of the second

// fn main() {
//     let x = 5;

//     {
//         let x = 12;
//         assert_eq!(x, 12);
//     }

//     assert_eq!(x, 12);

//     let x = 42;
//     println!("{}", x);
// }


fn main() {
    let x = 5;

    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x);
}