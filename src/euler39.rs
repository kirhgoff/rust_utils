// If p is the perimeter of a right angle triangle with 
//integral length sides, {a,b,c}, there are exactly three solutions for p = 120.

// {20,48,52}, {24,45,51}, {30,40,50}

// For which value of p ≤ 1000, is the number of solutions maximised?

// a^2  + b^2 = c^2
// a + b + c = p

// a,b => c = sqr(a^2 + b^2)

// p < a + b + c  | c < a + b
// p < 2 * (a + b)
// a + b > p/2 = 500

use std::collections::HashMap;

fn hypothenuse(a:u64, b:u64) -> Option<u64> {
  let sqrt = ((a*a + b*b) as f64).sqrt();
  if (sqrt - (sqrt as u64) as f64).abs() > 0. {
    None
  } else {
    Some(sqrt as u64)
  }
}

pub fn euler39() {
  build();
}

fn build() -> HashMap<u64, Vec<(u64, u64)>> {
  let mut hash_map:HashMap<u64, Vec<(u64, u64)>> = HashMap::new();
  for a in 1..500 {
    for b in 1..500 { // TODO could start from a
        match hypothenuse(a, b) {
          Some(c) => { 
            let p = a + b + c;
            let sides = (a, b);
            match hash_map.get(&p) {
              Some(vector) => { vector.push(sides) }
              None => { hash_map.insert(p, vec![(a,b)]); }
            }
          }
          None => { }
        }      
    }
  }
  println!("Build data ~ {} elements", hash_map.capacity());  
  hash_map
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn hypothenuse_works() {
    assert_eq!(Some(5), hypothenuse(3, 4));
    assert_eq!(None, hypothenuse(3, 5));
  }
}