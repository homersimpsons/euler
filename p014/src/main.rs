struct Collatz {
    current: u128,
}

impl Collatz {
    fn new(start: u128) -> Self {
        Collatz{current: start}
    }
}

impl Iterator for Collatz {
    type Item = u128;
    fn next(&mut self) -> Option<u128> {
        if self.current == 1 {
            None
        } else {
            if self.current % 2 == 0 {
                self.current /= 2;
            } else {
                self.current = self.current * 3 + 1;
            }
            Some(self.current)
        }
    }
}

fn longest_collatz_sequence(max: u128) -> u128 {
    let mut max_length = 1;
    let mut max_start = 1;
    for i in 2..max {
        let length = Collatz::new(i).count();
        if length > max_length {
            max_length = length;
            max_start = i;
        }
    }
    max_start
}

fn main() {
    longest_collatz_sequence(1000);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        assert_eq!(18, longest_collatz_sequence(20));
        assert_eq!(837799, longest_collatz_sequence(1_000_000));
    }
}
