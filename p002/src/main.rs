fn sum_fibonnaci_even(n: u128) -> u128 {
    let mut res = 0;
    let mut i = 1;
    let mut j = 2;
    while j < n {
        if j % 2 == 0 {
            res += j
        }
        let k = i + j;
        i = j;
        j = k;
    }
    res
}

fn main() {
    sum_fibonnaci_even(100);
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        assert_eq!(44, sum_fibonnaci_even(100));
        assert_eq!(4613732, sum_fibonnaci_even(4_000_000));
    }
}
