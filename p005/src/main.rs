pub fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b > 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn smallest_multiple(n: u128) -> u128 {
    if n < 2 {
        n
    } else {
        let m = smallest_multiple(n - 1);
        println!("{} => {}", n - 1, m);
        if m % n == 0 {
            m
        } else {
            m * (n / gcd(m, n))
        }
    }
}

fn main() {
    smallest_multiple(10);
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        assert_eq!(2520, smallest_multiple(10));
        assert_eq!(232792560, smallest_multiple(20));
    }
}
