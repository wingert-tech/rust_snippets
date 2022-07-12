// cargo run --bin iter_all
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let valid = v.iter().all(|&x| {
        x > 0
    });

    println!("Vec is valid : {:?}", valid); // Vec is valid : true

    let not_valid = v.iter().all(|&x| {
        x > 3
    });

    println!("Vec is valid : {:?}", not_valid); // Vec is valid : false
}
