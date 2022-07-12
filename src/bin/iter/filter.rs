// cargo run --bin iter_filter
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let result: Vec<_> = v.iter().filter(|&x| {
        *x > 2
    }).collect();
    println!("{:?}", result)
}