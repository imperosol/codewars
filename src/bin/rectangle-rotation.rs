/// [kyu]
/// 4
///
/// [description]
/// # Task
/// A rectangle with sides equal to even integers a and b is drawn on the Cartesian plane.
/// Its center (the intersection point of its diagonals) coincides with the point (0, 0),
/// but the sides of the rectangle are not parallel to the axes;
/// instead, they are forming 45 degree angles with the axes.
///
/// How many points with integer coordinates are located
/// inside the given rectangle (including on its sides)?
///
/// # Example
/// For `a = 6` and `b = 4`, the output should be 23
///
///
/// # Input/Output
/// - `[input]` integer `a`
///
/// A positive even integer.
///
/// Constraints: `2 ≤ a ≤ 10000`.
///
/// - `[input]` integer `b`
///
/// A positive even integer.
///
/// Constraints: `2 ≤ b ≤ 10000`.
///
/// - `[output]` an integer
///
/// The number of inner points with integer coordinates.
///
use std::cmp::Ordering;

#[derive(Debug)]
struct DiagonalRect {
    top: (f32, f32),
    right: (f32, f32),
    bottom: (f32, f32),
    left: (f32, f32),
}

#[derive(Debug, Clone)]
struct RectRow<'a> {
    rect: &'a DiagonalRect,
    x: i32,
}

impl Iterator for RectRow<'_> {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.x as f32) < self.rect.bottom.0 {
            None
        } else {
            let res = self.clone();
            self.x -= 1;
            Some(res)
        }
    }
}

impl RectRow<'_> {
    fn nb_points(&self) -> Option<u32> {
        let x = self.x as f32;
        if !(self.rect.bottom.0..self.rect.top.0).contains(&x) {
            return None;
        }
        let left_y = match self.rect.left.0.partial_cmp(&x).unwrap() {
            Ordering::Less | Ordering::Equal => self.x - (self.rect.top.0 - self.rect.top.1) as i32,
            Ordering::Greater => -self.x + (self.rect.left.0 + self.rect.left.1) as i32,
        };
        let right_y = match self.rect.right.0.partial_cmp(&x).unwrap() {
            Ordering::Less | Ordering::Equal => {
                -self.x + (self.rect.top.0 + self.rect.top.1) as i32
            }
            Ordering::Greater => self.x + (self.rect.right.1 - self.rect.right.0) as i32,
        };
        Some((right_y - left_y + 1) as u32)
    }
}

impl DiagonalRect {
    fn from_dims(a: i32, b: i32) -> Self {
        let (a, b) = (
            ((a - b) as f32) * 2f32.sqrt() / 4f32,
            ((a + b) as f32) * 2f32.sqrt() / 4f32,
        );
        Self {
            top: (b, a),
            right: (a, b),
            bottom: (-b, -a),
            left: (-a, -b),
        }
    }

    fn iter_rows(&self) -> RectRow {
        RectRow {
            rect: self,
            x: self.top.0.floor() as i32,
        }
    }
}

// Personal opinion
// Ok, I read the top answers on this kata, and omg I feel dumb
// This is really over-engineered compared to how simple the
// right solution is.
// I keep this code because it's not that badly written,
// but the actual right answer takes only three lines of code.
fn rectangle_rotation(a: i32, b: i32) -> i32 {
    let rect = DiagonalRect::from_dims(a, b);
    rect.iter_rows()
        .map(|row| row.nb_points().unwrap() as i32)
        .sum()
}

fn main() {
    assert_eq!(rectangle_rotation(6, 4), 23);
    assert_eq!(rectangle_rotation(30, 2), 65);
    assert_eq!(rectangle_rotation(8, 6), 49);
    assert_eq!(rectangle_rotation(16, 20), 333);
}
