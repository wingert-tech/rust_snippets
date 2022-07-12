// cargo run --bin iter_chain
// Takes two iterators and creates a new iterator over both in sequence.
fn main() {
    let poultry = vec!["chicken", "quail", "duck"];
    let livestock = vec!["cow", "goat", "pig"];

    // setting the type required for the borrow (farm: Vec<_>)
    // to be used in (for animal in &farm)
    let farm: Vec<_> = poultry.iter().chain(livestock.iter()).collect();
    for animal in &farm {
        println!("{:?}", animal);
    }

    println!("{:?}", farm);
}