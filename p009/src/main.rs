fn pythagorean_triplet(sum: u128) -> u128 {
    for n in 1u128.. {
        for m in (n+1).. {
            let a = m.pow(2) - n.pow(2);
            let b = 2 * m * n;
            let c = m.pow(2) + n.pow(2);
            let sumabc = a + b + c;
            if sum % sumabc == 0 {
                return (sum / sumabc).pow(3) * a * b * c;
            }
        }
    }
    unreachable!();
}

fn main() {
    pythagorean_triplet(1000);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        assert_eq!(60, pythagorean_triplet(12));
        assert_eq!(31875000, pythagorean_triplet(1000));
    }
}
