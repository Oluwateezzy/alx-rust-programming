// a scope is the range in a program in which an item is valid
// fn main() {
//     let x: i32 = 5;

//     {
//         let y = 6;
//         println!("{} {}", x, y);
//     }
//     println!("{} {}", x, y);
// }

fn main() {
    let x: i32 = 5;
    let y = 6;

    {
        println!("{} {}", x, y);
    }
    println!("{} {}", x, y);
}

