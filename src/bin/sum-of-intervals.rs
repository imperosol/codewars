/// [kyu]
/// 4
///
/// [description]
/// Write a function called sumIntervals/sum_intervals that accepts an array of intervals,
/// and returns the sum of all the interval lengths.
/// Overlapping intervals should only be counted once.
///
/// # Intervals
/// Intervals are represented by a pair of integers in the form of an array.
/// The first value of the interval will always be less than the second value.
/// Interval example: [1, 5] is an interval from 1 to 5.
/// The length of this interval is 4.
///
/// # Overlapping Intervals
/// List containing overlapping intervals:
/// [
///    [1, 4],
///    [7, 10],
///    [3, 5]
/// ]
/// The sum of the lengths of these intervals is 7.
/// Since [1, 4] and [3, 5] overlap, we can treat the interval
/// as [1, 5], which has a length of 4.
///
use itertools::Itertools;
use std::cmp::{max, min};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct MyRange {
    pub min: i32,
    pub max: i32,
}

impl From<&(i32, i32)> for MyRange {
    fn from(value: &(i32, i32)) -> Self {
        Self {
            min: value.0,
            max: value.1,
        }
    }
}

impl MyRange {
    fn merge(&mut self, other: &Self) -> Result<(), ()> {
        if self.min > other.max || self.max < other.min {
            Err(())
        } else {
            self.min = min(self.min, other.min);
            self.max = max(self.max, other.max);
            Ok(())
        }
    }
}

fn sum_intervals(intervals: &[(i32, i32)]) -> i32 {
    if intervals.is_empty() {
        return 0;
    }
    let mut intervals = intervals.iter().map(MyRange::from).sorted();
    let mut merged = vec![intervals.next().unwrap()];
    for interval in intervals {
        if merged.last_mut().unwrap().merge(&interval).is_err() {
            merged.push(interval);
        }
    }
    merged.iter().map(|i| i.max - i.min).sum()
}

fn main() {
    // non-overlapping intervals
    assert_eq!(sum_intervals(&[(1, 5)]), 4);
    assert_eq!(sum_intervals(&[(1, 5), (6, 10)]), 8);

    // overlapping intervals
    assert_eq!(sum_intervals(&[(1, 5), (1, 5)]), 4);
    assert_eq!(sum_intervals(&[(1, 4), (7, 10), (3, 5)]), 7);

    // large intervals
    assert_eq!(
        sum_intervals(&[(-1_000_000_000, 1_000_000_000)]),
        2_000_000_000,
    );
    assert_eq!(
        sum_intervals(&[(0, 20), (-100_000_000, 10), (30, 40)]),
        100_000_030,
    );
}
