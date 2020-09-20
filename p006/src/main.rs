fn sum_square_difference(n: u128) -> u128 {
    (n.pow(2) * (n + 1).pow(2) / 4) - (n * (n + 1) * (2 * n + 1) / 6)
}

fn main() {
    sum_square_difference(10);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        assert_eq!(2640, sum_square_difference(10));
        assert_eq!(25164150, sum_square_difference(100));
    }
}
