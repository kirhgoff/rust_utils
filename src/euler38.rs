
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

fn multipliers(number:u64, multipliers:Vec<u64>) -> Vec<u64> {
  multipliers.iter().map(|x| x * number).collect()
}

fn digits_count(number:u64) -> u64 {
  return (number as f64).log10().abs() as u64 + 1;
}

fn join(products:&Vec<u64>) -> u64 {
  return products.iter().fold(0, |sum, &x| sum * 10u64.pow(digits_count(x) as u32) + x);
}

pub fn euler38() {
  let max_length = 9;
  let mut max = 0;
  for x in 0..1000 {
    let multipliers_count = max_length / digits_count(x) + 1;
    let multipliers = 1..multipliers_count;
    let products = multipliers.map(|y| x * y).collect::<Vec<u64>>();
    let result = join(&products);
    println!("{} =>({:?}) {}", x, products, result);
    if digits_count(result) > 9 {
      continue;
    }
    if result > max {
      max = result;
    }
  }
  println!("Max: {}", max);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn multipliers_works() {
    assert_eq!(vec![2,4,6], multipliers(2, vec![1,2,3]));
  }

  #[test]
  fn digits_count_works() {
    assert_eq!(1, digits_count(1));
    assert_eq!(3, digits_count(321));
    assert_eq!(4, digits_count(1000));
  }

  #[test]
  fn join_works() {
    assert_eq!(123456, join(vec![123,456]));
    assert_eq!(123456, join(vec![12, 34, 56]));
  }

}