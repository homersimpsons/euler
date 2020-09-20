fn sum3and5(n: u128) -> u128 {
    let res3 = (0..n).step_by(3).sum::<u128>();
    let res5 = (0..n).step_by(5).sum::<u128>();
    let res15 = (0..n).step_by(15).sum::<u128>();
    res3 + res5 - res15
}

fn main() {
    sum3and5(100);
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        assert_eq!(23, sum3and5(10));
        assert_eq!(233168, sum3and5(1000));
    }
}
