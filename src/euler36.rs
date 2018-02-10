// The decimal number, 585 = 10010010012 (binary), 
// is palindromic in both bases.
// Find the sum of all numbers, less than one million, 
// which are palindromic in base 10 and base 2.
// (Please note that the palindromic number, in either base, 
// may not include leading zeros.)


use utils::is_palindrome;
use utils::decimal_digits;
use utils::binary_digits;
use utils::binary_string;

pub fn euler36() {
  let result = (0 as u64 .. 1000000 as u64)
      .filter(|x| is_palindrome(decimal_digits(*x)))
      .filter(|x| is_palindrome(binary_digits(*x)))
      .collect::<Vec<u64>>();

  for x in result.iter() {
    println!("{} {}", x, binary_string(*x));
  }
  println!("----------------");
  println!("Result: {}", result.iter().sum::<u64>());
}
