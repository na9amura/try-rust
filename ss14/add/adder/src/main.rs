extern crate add_one;
extern crate add_two;
extern crate rand;

fn main() {
    let num = 10;
    println!("Hello, world! {} plus two is {}!", num, add_two::add_two(num));
}
