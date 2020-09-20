fn is_palindrome(n: u128) -> bool {
    let string = n.to_string();
    string == string.chars().rev().collect::<String>()
}

fn largest_palindrome_product(n: u8) -> u128 {
    let max = 10u128.pow(n as u32);
    let mut res = 0;
    let mut i = max;
    while i > 0 {
        let mut j = max;
        while j > 0{
            let prod = i * j;
            if prod > res && is_palindrome(prod) {
                res = prod;
                break;
            } else if res > prod {
                break;
            }
            j -= 1;
        }
        i -= 1;
    }
    res
}

fn main() {
    largest_palindrome_product(100);
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn palindrome() {
        assert_eq!(true, is_palindrome(909));
        assert_eq!(true, is_palindrome(9009));
        assert_eq!(true, is_palindrome(9));
        assert_eq!(false, is_palindrome(9109));
    }
    #[test]
    fn it_works() {
        assert_eq!(9009, largest_palindrome_product(2));
        assert_eq!(906609, largest_palindrome_product(3));
    }
}
