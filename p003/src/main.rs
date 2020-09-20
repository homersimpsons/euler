fn largest_prime_factor(mut n: u128) -> u128 {
    let test = ((n as f64).sqrt() as u128) + 1;
    for x in 2..test {
        while n % x == 0 {
            if n == x {
                return n;
            }
            n /= x;
        }
    }
    unreachable!();
}

fn main() {
    largest_prime_factor(100);
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        assert_eq!(5, largest_prime_factor(100));
        assert_eq!(29, largest_prime_factor(13195));
        assert_eq!(6857, largest_prime_factor(600_851_475_143));
    }
}
