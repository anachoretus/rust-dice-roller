use rand::Rng; //import: random number generator

fn main() {
    let mut rng = rand::rng(); // mut = "mutable" => not fixed
    let roll: u8 =  rng.random_range(1..=20); //generate a number between 1 and 20;
    // u8 limits the variable to values between 0 and 255
    println!("You rolled a {}.", roll);
}