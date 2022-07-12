// cargo run --bin iter_enumerate
fn main() {
    let v = vec!["red", "white", "blue"];
    for (index, value) in v.iter().enumerate() {
        println!("{}: {}", index, value);
    }
}