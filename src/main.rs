pub mod card;
pub mod calculator;
pub mod hand;
pub mod hands;

fn main() {
    println!("Hello, world! Welcome to my calculator!");
    println!("{}", {calculator::calculate(10.0, 15.0)});
}
