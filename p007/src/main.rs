fn nth_prime(n: usize) -> u128 {
    let mut primes = Vec::<u128>::with_capacity(n);
    primes.push(2);
    let mut tested = 3u128;
    loop {
        for prime in primes.iter() {
            if prime.pow(2) > tested {
                if primes.len() + 1 == n {
                    return tested;
                }
                primes.push(tested);
                break;
            }
            if tested % prime == 0 {
                break;
            }
        }
        tested += 2;
    }
}

fn main() {
    nth_prime(10);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        assert_eq!(13, nth_prime(6));
        assert_eq!(104743, nth_prime(10001));
    }
}
