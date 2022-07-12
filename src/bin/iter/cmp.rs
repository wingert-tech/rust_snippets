// cargo run --bin iter_cmp
use std::cmp::Ordering;

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    //let v2 = vec![1, 2, 3, 4, 5]; // equal
    //let v2 = vec![1, 2, 3, 4, 5, 6]; // greater
    let v2 = vec![1, 2, 3, 4]; // less

    match  v.iter().cmp(v2.iter()) {
        Ordering::Less => println!("An ordering where a compared value is less than another."),
        Ordering::Equal => println!("An ordering where a compared value is equal to another."),
        Ordering::Greater => println!("An ordering where a compared value is greater than another."),
    }
}