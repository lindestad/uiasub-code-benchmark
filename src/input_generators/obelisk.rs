use rand::prelude::*;

pub fn gen_obelisks() -> String {
    let mut numbers: Vec<u128> = vec![2, 72, 8949, 0, 981038, 86311]; // initial numbers
    let mut rng = rand::rng();

    // Generate and push three random u64 numbers
    numbers.extend((0..3).map(|_| rand::random_range(4126..8921)));

    // Shuffle the vector
    numbers.shuffle(&mut rng);

    let stones = numbers
        .iter()
        .map(|stone| stone.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    stones
}
