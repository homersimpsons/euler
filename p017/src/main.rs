fn a_number_letter_count(n: usize) -> usize {
    assert!(n < 1_000_000);
    if n >= 1000 {
        a_number_letter_count(n / 1000)
            + "thousand".chars().count()
            + a_number_letter_count(n % 1000)
    } else if n >= 100 {
        a_number_letter_count(n / 100)
            + "hundred".chars().count()
            + a_number_letter_count(n % 100)
            + if n % 100 != 0 {
                "and".chars().count()
            } else {
                0
            }
    } else if n >= 20 {
        a_number_letter_count(n % 10)
            + match n / 10 {
                2 => "twenty".chars().count(),
                3 => "thirty".chars().count(),
                4 => "forty".chars().count(),
                5 => "fifty".chars().count(),
                6 => "sixty".chars().count(),
                7 => "seventy".chars().count(),
                8 => "eighty".chars().count(),
                9 => "ninety".chars().count(),
                _ => unreachable!(),
            }
    } else {
        match n {
            0 => 0,
            1 => "one".chars().count(),
            2 => "two".chars().count(),
            3 => "three".chars().count(),
            4 => "four".chars().count(),
            5 => "five".chars().count(),
            6 => "six".chars().count(),
            7 => "seven".chars().count(),
            8 => "eight".chars().count(),
            9 => "nine".chars().count(),
            10 => "ten".chars().count(),
            11 => "eleven".chars().count(),
            12 => "twelve".chars().count(),
            13 => "thirteen".chars().count(),
            14 => "fourteen".chars().count(),
            15 => "fifteen".chars().count(),
            16 => "sixteen".chars().count(),
            17 => "seventeen".chars().count(),
            18 => "eighteen".chars().count(),
            19 => "nineteen".chars().count(),
            _ => unreachable!(),
        }
    }
}

fn number_letter_counts(start: usize, end: usize) -> usize {
    (start..=end).map(|i| a_number_letter_count(i)).sum()
}

fn main() {
    number_letter_counts(1, 5);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        assert_eq!(19, number_letter_counts(1, 5));
        assert_eq!(21124, number_letter_counts(1, 1000));
    }
}
