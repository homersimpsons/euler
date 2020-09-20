fn lattice_paths(m0: usize, m1: usize) -> usize {
    // With analyse we can see that lattice(m0, m1) = C(m0 + m1, m0)
    let n = m0 + m1;
    let mut k = m0;
    let mut res = 1;

    if k > n - k {
        k = n - k;
    }
    (0..k).for_each(|i| {
        res *= n - i;
        res /= i + 1;
    });

    res
}

fn main() {
    lattice_paths(20, 20);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        assert_eq!(6, lattice_paths(2, 2));
        assert_eq!(137846528820, lattice_paths(20, 20));
    }
}
