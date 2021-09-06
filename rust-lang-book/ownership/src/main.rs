fn main() {
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // println!("{} and {}", r1, r2);
    // // r1 and r2 are no longer used after this point

    // let r3 = &mut s; // no problem
    // println!("{}", r3)

    // let reference_to_nothing = dangle();

    // let mut s = String::from("hello world");

    // // let hello = &s[0..5]; // &s[..5]
    // // let world = &s[6..11]; // &s[6..]
    // let word = first_word(&s);

    // s.clear();

    // println!("the first word is: {}", word);

    // let my_string = String::from("hello world");

    // let word = first_word(&my_string[..]);

    // let my_string_literal = "hello world";

    // let word = first_word(&my_string_literal[..]);

    // let word = first_word(my_string_literal);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }
