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

fn build() -> HashMap<u64, Vec<(u64, u64, u64)>> {
  let mut hash_map:HashMap<u64, Vec<(u64, u64, u64)>> = HashMap::new();
  for a in 1..500 {
    for b in 1..500 { // TODO could start from a
        match hypothenuse(a, b) {
          Some(c) => { 
            hash_map
              .entry(a + b + c).or_insert(vec![])
              .push((a, b, c));            
          }
          None => { }
        }      
    }
  }
  println!("Build data ~ {} elements", hash_map.capacity());  
  hash_map
}

fn max(hash_map:HashMap<u64, Vec<(u64, u64, u64)>>) -> u64 {
  let mut longest:&u64 = &0;
  let mut longest_key:&u64 = &0;
  let mut triangles:&Vec<(u64, u64, u64)> = &vec![];

  for (key, value) in hash_map.iter() {
    println!("{} => {:?}", key, value);
    let length = value.len() as u64;
    if length > *longest {
      longest = length;
      longest_key = key;
      triangles = value;
    }
  }
  *longest
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn hypothenuse_works() {
    assert_eq!(Some(5), hypothenuse(3, 4));
    assert_eq!(None, hypothenuse(3, 5));
  }
  
  // #[test]
  // fn try_problem() {
  //   let mut hash_map:HashMap<u64, Vec<(u64, u64)>> = HashMap::new();
  //   match hash_map.get(&3) {
  //      Some(vector) => { vector.push((1,2)) }
  //      None => { hash_map.insert(2, vec![(1,2)]); }
  //   }
  // }

  // #[test]
  // fn try_problem_fixed() {
  //   let mut hash_map:HashMap<u64, Vec<(u64, u64)>> = HashMap::new();
  //   hash_map.entry(3).or_insert(vec![]).push((1,2))
  // }
}