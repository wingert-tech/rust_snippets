// cargo run --bin iter_eq
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let equals = vec![1, 2, 3, 4, 5];
    let not_equals = vec![1, 2, 3];

    let should_be_equal = v.iter().eq(equals.iter());
    println!("{:?} is equal to {:?} : {:?}", v, equals, should_be_equal);
    let should_not_be_equal = v.iter().eq(not_equals.iter());
    println!("{:?} is equal to {:?} : {:?}", v, not_equals, should_not_be_equal);

}