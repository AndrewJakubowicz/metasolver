extern crate rand;

pub mod simulated_annealing;

pub fn a() -> i32 {
    2
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
