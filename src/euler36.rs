// The decimal number, 585 = 10010010012 (binary), 
// is palindromic in both bases.
// Find the sum of all numbers, less than one million, 
// which are palindromic in base 10 and base 2.
// (Please note that the palindromic number, in either base, 
// may not include leading zeros.)

struct DigitsGenerator {
  number: u32,
  base: u32,
}

impl DigitsGenerator {
  fn decimal(number: u32) -> DigitsGenerator {
    DigitsGenerator::with_base(number, 10)
  }

  fn binary(number: u32) -> DigitsGenerator {
    DigitsGenerator::with_base(number, 2)
  }

  fn with_base(number: u32, base: u32) -> DigitsGenerator {
    DigitsGenerator { number: number, base: base }
  }
}

impl Iterator for DigitsGenerator {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.number == 0 { return None; }

        let (new_number, digit) = mod_rem(self.number, self.base);
        self.number = new_number;

        return Some(digit);
    }
}

fn mod_rem(number:u32, divider:u32) -> (u32, u32) {
  (number / divider as u32, number % divider as u32)
}

fn is_palindrome(data:Vec<u32>) -> bool {
  data.iter().eq(data.iter().rev())
}

fn decimal_digits(number: u32) -> Vec<u32> {
  return DigitsGenerator::decimal(number).collect::<Vec<_>>();
}

fn binary_digits(number: u32) -> Vec<u32> {
  return DigitsGenerator::binary(number).collect::<Vec<_>>();
}

fn binary_string(number: u32) -> String {
  return binary_digits(number).iter().rev().map(|x| x.to_string()).collect();
}


pub fn euler36() {
  let result = (0 as u32 .. 1000000 as u32)
      .filter(|x| is_palindrome(decimal_digits(*x)))
      .filter(|x| is_palindrome(binary_digits(*x)))
      .collect::<Vec<u32>>();

  for x in result.iter() {
    println!("{} {}", x, binary_string(*x));
  }
  println!("----------------");
  println!("Result: {}", result.iter().sum::<u32>());
}

// --------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_rem_works() {
        assert_eq!((52, 1), mod_rem(521, 10));
        assert_eq!((0, 2), mod_rem(2, 10));
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