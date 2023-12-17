use space::{lerp, Vec3};

use crate::Curve3Impl;

/// Validates the derivative of a function with a 1-dimensional input space
pub fn validate_der_1d<F: Fn(f64) -> Vec3, D: Fn(f64) -> Vec3>(
    function: F,
    derivative: D,
    u_min: f64,
    u_max: f64,
    samples: usize,
    tolerance: f64,
    name: &str,
) {
    let u_len = u_max - u_min;

    for i in 0..samples {
        // Parameter deviation that we start checking each sample with
        let mut deviation = u_len / 10.0;

        // Define u for the current sample
        let u = lerp(u_min, u_max, i as f64 / (samples - 1) as f64);

        // Calculate the exact first derivative at u
        let computed_derivative = derivative(u);

        // Flag for whether the approximated first derivative below gets close enough to actual_der1
        let mut converged = false;

        // Last approximate derivative that does not contain NaNs
        let mut last_notnan_approx = Vec3::ZERO;

        // Iteratively approximate the derivative by getting the vector between two points on the curve
        // centered around u, decreasing their distance from u each time.
        for i in 0..64 {
            // Get parameters above and below u, clamped between 0 and 1
            let u_lo = (u - deviation).clamp(u_min, u_max);
            let u_hi = (u + deviation).clamp(u_min, u_max);

            // Evaluate the curve at those parameters
            let lo_pos = function(u_lo);
            let hi_pos = function(u_hi);

            // Approximate the derivative by getting a vector between those two points
            // and scaling it by the parameter distance between them
            let estimated_derivative = (hi_pos - lo_pos) / (u_hi - u_lo);

            /*
            println!("\nsample = {i}");
            println!("v_lo, v_hi = {u_lo}, {u_hi}");
            println!("actual deviation = {}", u_hi - u_lo);
            println!("lo_pos, hi_pos = {lo_pos}, {hi_pos}");
            println!("estimated_derivative = {estimated_derivative}");
             */

            if !estimated_derivative.has_nan() {
                last_notnan_approx = estimated_derivative;
            }

            // Get the difference between the exact derivative vector and the approximated one
            let error = (computed_derivative - estimated_derivative).magnitude();

            // If the distance is within tolerance, we consider the exact derivative
            // calculation to be valid and stop iteration for this sample.
            if error < tolerance {
                converged = true;
                break;
            }

            // If we haven't converged yet, reduce the deviation from u.
            deviation /= 2.0;
        }

        // Panic if we never got close enough to the exact derivative calculation.
        if !converged {
            panic!(
                "{} failed to converge at {} (computed = {}, converged = {})",
                name, u, computed_derivative, last_notnan_approx
            );
        }
    }
}

pub fn validate_ders_curve<C: Curve3Impl>(curve: &C, samples: usize, tolerance: f64) {
    validate_der1_curve(curve, samples, tolerance);
    validate_der2_curve(curve, samples, tolerance);
    validate_der3_curve(curve, samples, tolerance);
}

pub fn validate_der1_curve<C: Curve3Impl>(curve: &C, samples: usize, tolerance: f64) {
    validate_der_1d(
        |u| curve.eval(u).into_vec(),
        |u| curve.der1(u),
        curve.u_min(),
        curve.u_max(),
        samples,
        tolerance,
        "First derivative with respect to U",
    );
}

pub fn validate_der2_curve<C: Curve3Impl>(curve: &C, samples: usize, tolerance: f64) {
    validate_der_1d(
        |u| curve.der1(u),
        |u| curve.der2(u),
        curve.u_min(),
        curve.u_max(),
        samples,
        tolerance,
        "Second derivative with respect to U",
    );
}

pub fn validate_der3_curve<C: Curve3Impl>(curve: &C, samples: usize, tolerance: f64) {
    validate_der_1d(
        |u| curve.der2(u),
        |u| curve.der3(u),
        curve.u_min(),
        curve.u_max(),
        samples,
        tolerance,
        "Third derivative with respect to U",
    );
}
