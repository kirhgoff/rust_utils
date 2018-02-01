// The fraction 49/98 is a curious fraction, 
// as an inexperienced mathematician in attempting 
// to simplify it may incorrectly believe 
// that 49/98 = 4/8, which is correct, is obtained by cancelling the 9s.

// We shall consider fractions like, 30/50 = 3/5, to be trivial examples.
// There are exactly four non-trivial examples of this type of fraction, 
// less than one in value, and containing two digits in the 
// numerator and denominator.

// If the product of these four fractions is given in its lowest common 
// terms, find the value of the denominator.

extern crate fraction;

use fraction::Fraction;

fn main() {
    let digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut numerators = vec![];
    let mut denominators = vec![];

    for i in &digits {
      for m in &digits {
        for n in &digits {
          let numerator = m*10 + i;
          let denominator = i*10 + n;
          let big = (numerator as f32)/(denominator as f32);
          let small = (*m as f32)/(*n as f32);
          if big == small && big != 1.0 && big != 0.0 && !big.is_infinite() {
            println!("{}{}/{}{} => {} vs {}", m, i, i, n, big, small);
            numerators.push(numerator);
            denominators.push(denominator);
          }
        }
      }
    }

    let numerators_mul: i32 = numerators.iter().product();
    let denominators_mul: i32 = denominators.iter().product();
    println!("Numbers: {}/{}", numerators_mul, denominators_mul);

    let fraction = Fraction::new (numerators_mul as u64, denominators_mul as u64);
    println!("Overall: {}", fraction);
}
