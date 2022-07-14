// cargo run --bin loop_assign
fn main() {
    let mut count = 0;
    // loop resulting in a string value
    let result = loop {
        // incrementing the count variable by one each iteration
        count += 1;
        println!("Counting {:?}", count);
        if count == 10 {
            // end the loop and return this formatted string
            break format!("Count has reached {:?}", 10);
        }
    };
    println!("{}", result);
}
