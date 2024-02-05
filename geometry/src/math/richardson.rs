use std::ops::{Div, Mul, Sub};

use super::{Coincidence, Scalar};

pub fn richardson_extrapolate<R, F, D, S: Scalar>(
    func: F,
    diff: D,
    start: S,
    end: S,
    max_rows: usize,
    tolerance: f64,
) -> Option<R>
where
    R: Default + Clone + Sub<R, Output = R> + Mul<S, Output = R> + Div<S, Output = R>,
    F: Fn(S) -> R,
    D: Fn(R, R) -> S,
{
    let mut h = (end - start).abs();

    let mut a = vec![vec![R::default(); max_rows]; max_rows];

    let test_param = start.lerp(end, S::ONE - h);
    a[0][0] = func(test_param);

    let mut solution = None;

    for i in 0..max_rows - 1 {
        h = h / S::TWO;

        let test_param = start.lerp(end, S::ONE - h);
        a[i + 1][0] = func(test_param);

        for j in 0..=i {
            let num = (a[i + 1][j].clone() * S::FOUR.powi(j as i32 + 1)) - a[i][j].clone();
            let den = S::FOUR.powi(j as i32 + 1) - S::ONE;
            a[i + 1][j + 1] = num / den;
        }

        let latest = a[i + 1][i + 1].clone();
        let previous = a[i][i].clone();

        if diff(latest.clone(), previous).cc_tol(S::ZERO, tolerance) {
            solution = Some(latest.clone());
            break;
        }
    }

    solution
}
