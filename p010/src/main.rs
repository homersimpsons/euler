fn sum_primes(max: usize) -> usize {
    let mut primes: Vec<bool> = vec![true; max];
    let mut sum = 0;
    for i in 2..max {
        if primes[i] {
            sum += i;
            for not_prime in (i..max).step_by(i) {
                primes[not_prime] = false;
            }
        }
    };
    sum
}

fn main() {
    sum_primes(1000);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        assert_eq!(17, sum_primes(10));
        assert_eq!(76127, sum_primes(1_000));
        assert_eq!(142913828922, sum_primes(2_000_000));
    }
}
