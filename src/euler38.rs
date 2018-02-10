
/*

Take the number 192 and multiply it by each of 1, 2, and 3:

192 × 1 = 192
192 × 2 = 384
192 × 3 = 576
By concatenating each product we get the 1 to 9 pandigital, 192384576. 
We will call 192384576 the concatenated product of 192 and (1,2,3)

The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4, and 5, 
giving the pandigital, 918273645, which is the concatenated product of 9 and (1,2,3,4,5).

What is the largest 1 to 9 pandigital 9-digit number that can be formed as 
the concatenated product of an integer with (1,2, ... , n) where n > 1?

*/

use utils::digits_count;
use utils::is_pandigital;

fn multipliers_count(number:u64, max_length:u64) -> u64 {
  max_length / digits_count(number) + 1
}
 
fn concatenated_product(number:u64, max_multiplier:u64) -> u64 {
  (1..max_multiplier + 1)
    .map(|m| number * m as u64)
    .fold(0, |sum, x| sum * 10u64.pow(digits_count(x) as u32) + x) 
}

pub fn euler38() {
  let max_length = 9;
  let mut max = 0;
  for x in 0..10_000+1 {
    let multipliers_count = multipliers_count(x, max_length);
    for y in 0..multipliers_count + 1 {
      let result = concatenated_product(x, y);
      if digits_count(result) == 9 && is_pandigital(result) {
        println!("{} -> {} -> {}", x, y, result);
        if result > max {
          max = result;
        }
      }
    }
  }
  println!("Max: {}", max);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn concatenated_product_works() {
    assert_eq!(192384576, concatenated_product(192, 3));
    assert_eq!(918273645, concatenated_product(9, 5));    
  }
}