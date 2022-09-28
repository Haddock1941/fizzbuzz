/// fizzbuzz
/// ---
/// returns “Fizz” if n is divisible by 3
/// returns “Buzz” if n is divisible by 5
/// returns “FizzBuzz” if n is divisible by both 3 and 5
pub fn fizzbuzz(n: usize) -> String {
    match n {
        // n divisible by 15 <=> n divisible by 3 AND 5
        _ if n % 15 == 0 => "FizzBuzz".to_string(),
        _ if n % 3 == 0 => "Fizz".to_string(),
        _ if n % 5 == 0 => "Buzz".to_string(),
        // not in specification, maybe add a default return?
        _ => panic!(
            "{} is not divisible by 3 OR 5.\nNot specified what is supposed to happen.",
            n
        ),
    }
}

fn main() {
    println!("{}Buzz", fizzbuzz(3));
}

#[cfg(test)]
mod tests {
    use crate::fizzbuzz;

    #[test]
    // test basic Fizz
    fn test_fizz() {
        assert_eq!(fizzbuzz(3), "Fizz");
    }

    #[test]
    // test basic Buzz
    fn test_buzz() {
        assert_eq!(fizzbuzz(5), "Buzz");
    }

    #[test]
    // test basic FizzBuzz
    fn test_fizzbuzz() {
        assert_eq!(fizzbuzz(15), "FizzBuzz");
    }

    #[test]
    // test more and larger numbers for Fizz
    fn test_larger_fizz() {
        assert_eq!(fizzbuzz(9), "Fizz");
        assert_eq!(fizzbuzz(12), "Fizz");
        assert_eq!(fizzbuzz(33), "Fizz");
        assert_eq!(fizzbuzz(333), "Fizz");
        assert_eq!(fizzbuzz(387129), "Fizz");
    }

    #[test]
    // test more and larger numbers for Buzz
    fn test_larger_buzz() {
        assert_eq!(fizzbuzz(10), "Buzz");
        assert_eq!(fizzbuzz(20), "Buzz");
        assert_eq!(fizzbuzz(55), "Buzz");
        assert_eq!(fizzbuzz(54790), "Buzz");
    }

    #[test]
    // test more and larger numbers for FizzBuzz
    fn test_larger_fizzbuzz() {
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(30), "FizzBuzz");
        assert_eq!(fizzbuzz(555), "FizzBuzz");
        assert_eq!(fizzbuzz(164370), "FizzBuzz");
    }
}
