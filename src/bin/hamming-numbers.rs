/// [kyu]
/// 4
///
/// [description]
/// A Hamming number is a positive integer of the form 2i3j5k,
/// for some non-negative integers i, j, and k.
///
/// Write a function that computes the nth smallest Hamming number.
///
/// Specifically:
///
/// The first smallest Hamming number is 1 = 203050
/// The second smallest Hamming number is 2 = 213050
/// The third smallest Hamming number is 3 = 203150
/// The fourth smallest Hamming number is 4 = 223050
/// The fifth smallest Hamming number is 5 = 203051
/// The 20 smallest Hamming numbers are given in the Example test fixture.
///
/// Your code should be able to compute the first 13 000 Hamming numbers without timing out.

fn hamming(n: usize) -> u64 {
    let mut hamming = Vec::with_capacity(n);
    hamming.push(1);
    let (mut i2, mut i3, mut i5) = (0, 0, 0);
    let (mut next_multiple_of_2, mut next_multiple_of_3, mut next_multiple_of_5) = (2, 3, 5);
    for _ in 1..n {
        // Choose the minimum value of all available multiples
        let min_next_mutliple = next_multiple_of_2
            .min(next_multiple_of_3)
            .min(next_multiple_of_5);
        hamming.push(min_next_mutliple);
        if min_next_mutliple == next_multiple_of_2 {
            i2 += 1;
            next_multiple_of_2 = hamming[i2] * 2;
        }
        if min_next_mutliple == next_multiple_of_3 {
            i3 += 1;
            next_multiple_of_3 = hamming[i3] * 3;
        }
        if min_next_mutliple == next_multiple_of_5 {
            i5 += 1;
            next_multiple_of_5 = hamming[i5] * 5;
        }
    }
    *hamming.last().unwrap()
}

fn main() {
    assert_eq!(hamming(1), 1);
    assert_eq!(hamming(2), 2);
    assert_eq!(hamming(3), 3);
    assert_eq!(hamming(4), 4);
    assert_eq!(hamming(5), 5);
    assert_eq!(hamming(6), 6);
    assert_eq!(hamming(7), 8);
    assert_eq!(hamming(8), 9);
    assert_eq!(hamming(9), 10);
    assert_eq!(hamming(10), 12);
    assert_eq!(hamming(11), 15);
    assert_eq!(hamming(12), 16);
    assert_eq!(hamming(13), 18);
    assert_eq!(hamming(14), 20);
    assert_eq!(hamming(15), 24);
    assert_eq!(hamming(16), 25);
    assert_eq!(hamming(17), 27);
    assert_eq!(hamming(18), 30);
    assert_eq!(hamming(19), 32);
}
