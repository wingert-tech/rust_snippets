
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let valid = v.iter().any(|&x| {
        x > 2
    });

    println!("Vec is valid : {:?}", valid); // Vec is valid : true

    let not_valid = v.iter().any(|&x| {
        x > 14
    });

    println!("Vec is valid : {:?}", not_valid); // Vec is valid : false
}
