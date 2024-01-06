use std::ops::{Div, Mul, Sub};

use crate::math::lerp;

use super::Coincidence;

pub fn richardson_extrapolate<R, F, D>(
    func: F,
    diff: D,
    start: f64,
    end: f64,
    max_rows: usize,
    tolerance: f64,
) -> Option<R>
where
    R: Default + Clone + Sub<R, Output = R> + Mul<f64, Output = R> + Div<f64, Output = R>,
    F: Fn(f64) -> R,
    D: Fn(R, R) -> f64,
{
    let mut h = (end - start).abs();

    let mut a = vec![vec![R::default(); max_rows]; max_rows];

    let test_param = lerp(start, end, 1.0 - h);
    a[0][0] = func(test_param);

    let mut solution = None;

    for i in 0..max_rows - 1 {
        h /= 2.0;

        let test_param = lerp(start, end, 1.0 - h);
        a[i + 1][0] = func(test_param);

        for j in 0..=i {
            let num = (a[i + 1][j].clone() * 4f64.powi(j as i32 + 1)) - a[i][j].clone();
            let den = 4f64.powi(j as i32 + 1) - 1.0;
            a[i + 1][j + 1] = num / den;
        }

        let latest = a[i + 1][i + 1].clone();
        let previous = a[i][i].clone();

        if diff(latest.clone(), previous).coincident(0.0, tolerance) {
            solution = Some(latest.clone());
            break;
        }
    }

    solution
}
