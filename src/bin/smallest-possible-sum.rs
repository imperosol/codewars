use itertools::Itertools;
/// [kyu]
/// 4
///
/// [description]
/// Given an array X of positive integers, its elements are to be transformed by running
/// the following operation on them as many times as required:
///
/// ```
/// if X[i] > X[j] then X[i] = X[i] - X[j]
/// ```
///
/// When no more transformations are possible, return its sum ("smallest possible sum").
///
/// For instance, the successive transformation of the elements of
/// input `X = [6, 9, 21]` is detailed below:
///
/// ```
/// X_1 = [6, 9, 12] # -> X_1[2] = X[2] - X[1] = 21 - 9
/// X_2 = [6, 9, 6]  # -> X_2[2] = X_1[2] - X_1[0] = 12 - 6
/// X_3 = [6, 3, 6]  # -> X_3[1] = X_2[1] - X_2[0] = 9 - 6
/// X_4 = [6, 3, 3]  # -> X_4[2] = X_3[2] - X_3[1] = 6 - 3
/// X_5 = [3, 3, 3]  # -> X_5[1] = X_4[0] - X_4[1] = 6 - 3
/// ```
/// The returning output is the sum of the final transformation (here 9).
///
/// # Example
/// ```
/// solution([6, 9, 21]) #-> 9
/// ```
/// # Solution steps:
/// ```
/// [6, 9, 12] #-> X[2] = 21 - 9
/// [6, 9, 6] #-> X[2] = 12 - 6
/// [6, 3, 6] #-> X[1] = 9 - 6
/// [6, 3, 3] #-> X[2] = 6 - 3
/// [3, 3, 3] #-> X[1] = 6 - 3
/// ```
use std::mem;

fn gcd(mut u: u64, mut v: u64) -> u64 {
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }

    // Using identity 2
    let shift = (u | v).trailing_zeros();

    u >>= shift;
    v >>= shift;
    u >>= u.trailing_zeros();
    loop {
        v >>= v.trailing_zeros();

        if u > v {
            mem::swap(&mut u, &mut v);
        }

        v -= u; // here v >= u

        if v == 0 {
            break;
        }
    }

    u << shift
}

fn solution(arr: &[u64]) -> u128 {
    let min_gcd = arr
        .iter()
        .circular_tuple_windows()
        .map(|(&i, &j)| gcd(i, j))
        .min()
        .unwrap();
    min_gcd as u128 * arr.len() as u128
}

fn main() {
    assert_eq!(solution(&[1, 21, 55]), 3);
    assert_eq!(solution(&[3, 13, 23, 7, 83]), 5);
    assert_eq!(solution(&[4, 16, 24]), 12);
    assert_eq!(solution(&[30, 12]), 12);
    assert_eq!(solution(&[60, 12, 96, 48, 60, 24, 72, 36, 72, 72, 48]), 132);
    assert_eq!(
        solution(&[71, 71, 71, 71, 71, 71, 71, 71, 71, 71, 71, 71, 71]),
        923
    );
    assert_eq!(solution(&[11, 22]), 22);
    assert_eq!(solution(&[9]), 9);
    assert_eq!(solution(&[1]), 1);
    assert_eq!(solution(&[9, 9]), 18);
}
