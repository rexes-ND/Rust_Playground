struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let sum: Vec<_> = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .collect();
    println!("{:?}", sum);
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    // for val in v1_iter {
    //     println!("Got: {}", val);
    // }
    // let total: i32 = v1_iter.sum();
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    let in_my_size = shoes_in_size(shoes, 10);

    println!("{:?}", in_my_size);
}
#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
