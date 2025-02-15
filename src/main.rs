use rand::Rng; //import: random number generator
use std::io; //import: standard input handling

fn main() {
    println!("Pick your die:");

    let mut input = String::new(); //store user input in string
    io::stdin().read_line(&mut input).expect("Failed to read input"); //read input

    let sides: u8 = match input.trim().parse() {
        Ok(num) if [4, 6, 8, 10, 12, 20, 100].contains(&num) => num, //validate dice sides
        _ => { //else
            println!("Invalid input. Defaulting to d20.");
            20 //default do d20 if input is invalid
        }
    };

    let mut rng = rand::rng(); // mut = "mutable" => not fixed
    let roll: u8 =  rng.random_range(1..=sides); //generate a number between 1 and 20;
    // u8 limits the variable to values between 0 and 255
    println!("You rolled a {}.", roll);
}