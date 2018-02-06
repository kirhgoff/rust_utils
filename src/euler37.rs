// The number 3797 has an interesting property. Being prime itself, 
// it is possible to continuously remove digits from left to right, 
// and remain prime at each stage: 3797, 797, 97, and 7. 
// Similarly we can work from right to left: 3797, 379, 37, and 3.

// Find the sum of the only eleven primes that are both truncatable 
// from left to right and right to left.

// NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.

// [3797, 0] -> [379, 7] -> [37, 9*10 + 7] -> [3, 7*100 + 9*10 + 7]

extern crate primal;

use self::primal::*;

// TODO: extract as crate
fn mod_rem(number: usize) -> (usize, usize) {
  (number / 10, number % 10)
}

fn split (base: usize, remainder: usize, order: usize) -> (usize, usize, usize) {
  let (m, r) = mod_rem(base);
  (m, remainder + order * r, order * 10)
}

fn is_truncatable_prime(x: usize) -> bool {
  if x < 10 || !is_prime(x as u64) { return false };
  let (mut left, mut right, mut order) = split(x, 0, 1);
  while left > 0 && right != x {
    if !is_prime(left as u64) || !is_prime(right as u64) { return false };
    let (new_left, new_right, new_order) = split(left, right, order);
    left = new_left;
    right = new_right;
    order = new_order;
  }
  true
}

pub fn euler37() {
    let primes = Primes::all()
        .filter(|x| is_truncatable_prime(*x))
        .take(11)
        .collect::<Vec<usize>>();

    println!("{:?}", primes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_rem_works() {
        assert_eq!((35, 6), mod_rem(356));
        assert_eq!((0, 6), mod_rem(6));
    }

    #[test]
    fn split_works() {
        assert_eq!((52, 3, 10), split(523, 0, 1));
        assert_eq!((5, 23, 100), split(52, 3, 10));
        assert_eq!((0, 523, 1000), split(5, 23, 100));
    }

    #[test]
    fn is_truncatable_prime_works() {
        assert!(!is_truncatable_prime(7));
        assert!(!is_truncatable_prime(22));
        assert!(!is_truncatable_prime(239));

        assert!(is_truncatable_prime(3797));
    }    
}