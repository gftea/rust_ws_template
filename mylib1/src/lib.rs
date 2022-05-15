use rand;

pub fn who_am_i() -> &'static str {
    let me = env!("CARGO_CRATE_NAME");
    println!("I am from {}, rand: {}", me, rand::random::<i32>());
    me
}

#[cfg(test)]
mod tests {
    use crate::who_am_i;

    #[test]
    fn it_works() {
        assert_eq!("mylib1", who_am_i());
    }
}
