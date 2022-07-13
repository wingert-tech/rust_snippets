// cargo run --bin iter
// borrowing / owning of iterations

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    // for this vector, when iter() is called, it created a borrow
    // then filter also creates a borrow
    // leaving the closure type &&x
    //                            &      &   => &&x
    let result: Vec<_> = v.iter().filter(|&&x| {
        x > 2
    }).collect();
    println!("{:?}", result);

    // this example defines the closure type as a borrowed x
    let result: Vec<_> = v.iter().filter(|&x| {
        // * removes a borrow, now ending up with the x standing on its own
        *x > 2
    }).collect();

    println!("{:?}", result);

    let v = vec![1, 2, 3, 4, 5];
    let result: Vec<_> = v.iter().filter(|x| {
        // here you can remove the borrow twice
        **x > 2
    }).collect();

    println!("{:?}", result);

}

