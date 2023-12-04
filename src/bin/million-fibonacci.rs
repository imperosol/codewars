/// [kyu] 3
///
/// [description]
/// The year is 1214. One night, Pope Innocent III awakens to find the the archangel
/// Gabriel floating before him. Gabriel thunders to the pope:
///
/// > Gather all of the learned men in Pisa, especially Leonardo Fibonacci.
/// > In order for the crusades in the holy lands to be successful,
/// > these men must calculate the millionth number in Fibonacci's recurrence.
/// > Fail to do this, and your armies will never reclaim the holy land. It is His will.
///
/// The angel then vanishes in an explosion of white light.
///
/// Pope Innocent III sits in his bed in awe.
/// How much is a million? he thinks to himself. He never was very good at math.
///
/// He tries writing the number down,
/// but because everyone in Europe is still using Roman numerals
/// at this moment in history, he cannot represent this number.
/// If he only knew about the invention of zero,
/// it might make this sort of thing easier.
///
/// He decides to go back to bed. He consoles himself,
/// The Lord would never challenge me thus;
/// this must have been some deceit by the devil.
/// A pretty horrendous nightmare, to be sure.
///
/// Pope Innocent III's armies would go on to conquer Constantinople
/// (now Istanbul), but they would never reclaim the holy land as he desired.
///
/// In this kata you will have to calculate fib(n) where:
/// ```
/// fib(0) := 0
/// fib(1) := 1
/// fib(n + 2) := fib(n + 1) + fib(n)
/// ```
///
/// Write an algorithm that can handle n up to 2000000.
///
/// Your algorithm must output the exact integer answer, to full precision. Also, it must correctly handle negative numbers as input.
///
/// HINT I: Can you rearrange the equation
/// `fib(n + 2) = fib(n + 1) + fib(n)` to find fib(n) if you already
/// know fib(n + 1) and fib(n + 2)?
/// Use this to reason what value fib has to have for negative values.
///
/// HINT II: See https://web.archive.org/web/20220614001843/https://mitpress.mit.edu/sites/default/files/sicp/full-text/book/book-Z-H-11.html#%_sec_1.2.4
use num::bigint::BigInt;
use num::pow::Pow;
use num::{One, Zero};
use std::ops::MulAssign;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct FibMatrix([[BigInt; 2]; 2]);
impl MulAssign<&FibMatrix> for FibMatrix {
    fn mul_assign(&mut self, rhs: &Self) {
        let a = &self.0[0][0] * &rhs.0[0][0] + &self.0[0][1] * &rhs.0[1][0];
        let b = &self.0[0][0] * &rhs.0[0][1] + &self.0[0][1] * &rhs.0[1][1];
        let c = &self.0[1][0] * &rhs.0[0][0] + &self.0[1][1] * &rhs.0[1][0];
        let d = &self.0[1][0] * &rhs.0[0][1] + &self.0[1][1] * &rhs.0[1][1];
        self.0[0][0] = a;
        self.0[0][1] = b;
        self.0[1][0] = c;
        self.0[1][1] = d;
    }
}

impl Pow<u32> for FibMatrix {
    type Output = Self;

    fn pow(self, n: u32) -> Self::Output {
        // inspired by the nalgebra code
        // https://docs.rs/nalgebra/latest/src/nalgebra/linalg/pow.rs.html
        if n == 0 || n == 1 {
            return FibMatrix::new();
        }
        let mut exp = n;
        let mut res = self.clone();
        let mut x = self.clone();
        if exp % 2 == 0 {
            res = FibMatrix([[1.into(), 0.into()], [0.into(), 1.into()]]);
        } else {
            exp -= 1
        }
        loop {
            if exp % 2 == 1 {
                res *= &x;
            }
            exp /= 2;
            if exp == 0 {
                break;
            }
            x *= &x.clone();
        }
        res
    }
}

impl FibMatrix {
    fn new() -> Self {
        FibMatrix([[1.into(), 1.into()], [1.into(), 0.into()]])
    }
}

/// The nth fibonacci number is the first element of the first row
/// of the n-th power of the fibonacci matrix.
/// The fibonacci matrix is :
/// ```
/// [ 1 1 ]
/// [ 1 0 ]
/// ```
/// This method is more efficient than the usual simple iterative one
/// because there exist methods to compute the n-th power of a matrix
/// with logarithmic complexity.
///
/// This is a fairly good solution, but there exists another one,
/// far more clever, that relies on some magic with bits and has a O(1)
/// complexity.
fn fib(n: i32) -> BigInt {
    let res = match n.abs() {
        0 => BigInt::zero(),
        1 => BigInt::one(),
        i => FibMatrix::new().pow(i as u32 - 1).0[0][0].clone(),
    };
    if n < 0 && n % 2 == 0 {
        -res
    } else {
        res
    }
}

fn main() {
    assert_eq!(fib(0), BigInt::zero());
    assert_eq!(fib(1), BigInt::one());
    assert_eq!(fib(2), BigInt::one());
    assert_eq!(fib(3), BigInt::from(2));
    assert_eq!(fib(4), BigInt::from(3));
    assert_eq!(fib(5), BigInt::from(5));

    assert_eq!(
        fib(-500),
        BigInt::from_str("-139423224561697880139724382870407283950070256587697307264108962948325571622863290691557658876222521294125")
            .unwrap()
    );

    assert_eq!(
        fib(1000),
        BigInt::from_str("43466557686937456435688527675040625802564660517371780402481729089536555417949051890403879840079255169295922593080322634775209689623239873322471161642996440906533187938298969649928516003704476137795166849228875")
            .unwrap()
    );
}

// #[test]
// fn small_negative_numbers() {
//     dotest(-1, BigInt::from(1));
//     dotest(-6, BigInt::from(-8));
//     dotest(-96, BigInt::from_str("-51680708854858323072").unwrap());
//
// }
