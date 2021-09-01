// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

// fn main() {
//     // let guess: u32 = "42".parse().expect("Not a number!");
//     // println!("{}", guess);
//     // let num = 5 / 2;
//     // println!("{}", num);
//     // let tup = (500, 6.4, 1);

//     // let (x, y, z) = tup;

//     // println!("The value of y is: {}", y);
//     let a = [1, 2, 3, 4, 5];

//     let first = a[0];
//     let second = a[1];
//     println!("{}", first);
// }

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
