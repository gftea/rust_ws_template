use mylib1;
use mylib2;
use rand;

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)] // generate implementation of trait HelloMacro for Pancakes
struct Pancakes;

fn main() {
    println!("workspace rand: {}", rand::random::<i32>());
    println!("access {}", mylib1::who_am_i());
    println!("access {}", mylib2::who_am_i());

    // call the generated implementation of trait HelloMacro
    Pancakes::hello_macro();
}
