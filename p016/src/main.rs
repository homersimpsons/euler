extern crate num_bigint;
extern crate num_traits;
use num_bigint::ToBigUint;
use num_traits::pow::Pow;

fn power_digit_sum(number: usize, power: usize) -> usize {
    number.to_biguint().unwrap().pow(power.to_biguint().unwrap())
        .to_radix_be(10)
        .iter()
        .map(|&i| i as usize)
        .sum::<usize>()
}

fn main() {
    power_digit_sum(2, 15);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        assert_eq!(26, power_digit_sum(2, 15));
        assert_eq!(1366, power_digit_sum(2, 1000));
    }
}
