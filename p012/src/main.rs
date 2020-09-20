#![feature(iter_map_while)]

struct Primes {
    next: usize,
    primes: Vec<usize>,
}

impl Primes {
    fn new() -> Primes {
        Primes {
            next: 0,
            primes: vec![2, 3],
        }
    }

    fn rewind(&mut self) {
        self.next = 0;
    }

    fn find_next_prime(&mut self) {
        let last = match self.primes.last() {
            Some(&i) => i,
            None => {
                self.primes.push(2);
                2
            }
        };
        let prime = ((last + 1)..)
            .find(|&i| {
                let max = ((i as f64).sqrt() as usize) + 1;
                self.primes
                    .iter()
                    .take_while(|&&prime| prime < max)
                    .all(|prime| i % prime != 0)
            })
            .unwrap();
        self.primes.push(prime);
    }
}

impl Iterator for Primes {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        while let None = self.primes.get(self.next) {
            self.find_next_prime();
        }
        self.next += 1;
        Some(*self.primes.get(self.next - 1).unwrap())
    }
}

fn number_of_divisor(mut n: usize, primes: &mut Primes) -> usize {
    primes.rewind();
    primes
        .map_while(|i| {
            if i <= n {
                let mut exp = 1;
                while n % i == 0 {
                    n /= i;
                    exp += 1;
                }
                Some(exp)
            } else {
                None
            }
        })
        .product::<usize>()
}

fn triangular_number_divisible(divisor: usize) -> usize {
    let mut primes = Primes::new();
    let mut triangle = 0;
    for i in 1.. {
        triangle += i;
        if number_of_divisor(triangle, &mut primes) >= divisor {
            return triangle;
        }
    }
    unreachable!()
}

fn main() {
    triangular_number_divisible(500);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn primes_works() {
        let mut primes = Primes::new();
        assert_eq!(primes.next(), Some(2));
        primes.rewind();
        assert_eq!(primes.next(), Some(2));
        assert_eq!(primes.next(), Some(3));
        assert_eq!(primes.next(), Some(5));
        assert_eq!(primes.next(), Some(7));
        assert_eq!(primes.next(), Some(11));
        assert_eq!(primes.next(), Some(13));
        assert_eq!(primes.next(), Some(17));
    }

    #[test]
    fn number_of_divisor_works() {
        let mut primes = Primes::new();
        assert_eq!(6, number_of_divisor(28, &mut primes));
    }

    #[test]
    fn it_works() {
        assert_eq!(28, triangular_number_divisible(5));
        assert_eq!(76576500, triangular_number_divisible(500));
    }
}
