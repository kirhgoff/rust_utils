use std::collections::HashSet;
use std::iter::FromIterator;

struct DigitsGenerator {
  number: u64,
  base: u64,
}

impl DigitsGenerator {
  fn decimal(number: u64) -> DigitsGenerator {
    DigitsGenerator::with_base(number, 10)
  }

  fn binary(number: u64) -> DigitsGenerator {
    DigitsGenerator::with_base(number, 2)
  }

  fn with_base(number: u64, base: u64) -> DigitsGenerator {
    DigitsGenerator { number: number, base: base }
  }
}

impl Iterator for DigitsGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.number == 0 { return None; }

        let (new_number, digit) = mod_rem(self.number, self.base);
        self.number = new_number;

        return Some(digit);
    }
}

pub fn mod_rem(number:u64, divider:u64) -> (u64, u64) {
  (number / divider as u64, number % divider as u64)
}

pub fn mod_rem_10(number: usize) -> (usize, usize) {
  (number / 10, number % 10)
}

pub fn is_palindrome(data:Vec<u64>) -> bool {
  data.iter().eq(data.iter().rev())
}

pub fn decimal_digits(number: u64) -> Vec<u64> {
  return DigitsGenerator::decimal(number).collect::<Vec<_>>();
}

pub fn binary_digits(number: u64) -> Vec<u64> {
  return DigitsGenerator::binary(number).collect::<Vec<_>>();
}

pub fn binary_string(number: u64) -> String {
  return binary_digits(number).iter().rev().map(|x| x.to_string()).collect();
}

pub fn digits_count(number:u64) -> u64 {
  return (number as f64).log10().abs() as u64 + 1;
}

pub fn is_pandigital(number:u64) -> bool {
  if number % 9 != 0 { return false; }
  let set:HashSet<u64> = HashSet::from_iter(decimal_digits(number).iter().cloned());
  for i in 1..10 {
    if !set.contains(&i) { return false; }
  }
  return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_count_works() {
      assert_eq!(1, digits_count(1));
      assert_eq!(3, digits_count(321));
      assert_eq!(4, digits_count(1000));
    }

    #[test]
    fn is_pandigital_works() {
      assert!(!is_pandigital(123));
      assert!(!is_pandigital(145678999));
      assert!(is_pandigital(123456789));
    }

    #[test]
    fn mod_rem_works() {
        assert_eq!((52, 1), mod_rem(521, 10));
        assert_eq!((0, 2), mod_rem(2, 10));
        assert_eq!((35, 6), mod_rem_10(356));
        assert_eq!((0, 6), mod_rem_10(6));
    }

    #[test]
    fn digits_iterator_works() {
        assert_eq!(vec![3, 2, 1], decimal_digits(123));
        assert_eq!(vec![1, 0, 0, 1, 0, 0, 1, 0, 0, 1], binary_digits(585));        
    }

    #[test]
    fn is_palindrome_works() {
        assert!(is_palindrome(vec![1, 2, 1]));
        assert!(!is_palindrome(vec![1, 2, 3]));
    }

    #[test]
    fn binary_string_works() {
        assert_eq!("1010", binary_string(10));
    }

}