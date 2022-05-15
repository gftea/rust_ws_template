use mylib1;
use mylib2;
use rand;

fn main() {
    println!("workspace rand: {}", rand::random::<i32>());
    println!("access {}", mylib1::who_am_i());
    println!("access {}", mylib2::who_am_i());
}
