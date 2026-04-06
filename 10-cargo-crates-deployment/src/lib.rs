//! Basic math formulas for quadratic equations and 2x2 matrices.
//!
//! This crate currently supports:
//! - quadratic equation solving (`ax^2 + bx + c = 0`)
//! - 2x2 matrix determinant
//! - other common 2x2 matrix operations: trace, transpose, inverse
//! - solving a 2x2 linear system using Cramer's rule

/// A 2x2 matrix represented as:
/// `[[a, b], [c, d]]`
pub type Matrix2x2 = [[f64; 2]; 2];

/// Errors returned by math operations in this crate.
#[derive(Debug, Clone, PartialEq)]
pub enum MathError {
    /// The `a` coefficient is zero, so the equation is not quadratic.
    NotQuadratic,
    /// The matrix cannot be inverted or used with Cramer's rule because
    /// its determinant is zero.
    SingularMatrix,
}

/// The root structure returned by [`solve_quadratic`].
#[derive(Debug, Clone, PartialEq)]
pub enum QuadraticRoots {
    /// Two distinct real roots.
    TwoReal(f64, f64),
    /// One repeated real root.
    OneReal(f64),
    /// A pair of complex-conjugate roots `(real, imaginary)`.
    Complex((f64, f64), (f64, f64)),
}

/// Computes the discriminant of a quadratic equation: `b^2 - 4ac`.
///
/// The sign of the discriminant tells the root type:
/// - positive: two distinct real roots
/// - zero: one repeated real root
/// - negative: two complex-conjugate roots
pub fn discriminant(a: f64, b: f64, c: f64) -> f64 {
    b * b - 4.0 * a * c
}

/// Solves the quadratic equation `ax^2 + bx + c = 0`.
///
/// Returns:
/// - [`QuadraticRoots::TwoReal`] when discriminant is positive
/// - [`QuadraticRoots::OneReal`] when discriminant is zero
/// - [`QuadraticRoots::Complex`] when discriminant is negative
///
/// # Errors
/// Returns [`MathError::NotQuadratic`] when `a == 0.0`.
pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Result<QuadraticRoots, MathError> {
    if a == 0.0 {
        return Err(MathError::NotQuadratic);
    }

    let d = discriminant(a, b, c);
    let two_a = 2.0 * a;

    if d > 0.0 {
        let sqrt_d = d.sqrt();
        Ok(QuadraticRoots::TwoReal(
            (-b + sqrt_d) / two_a,
            (-b - sqrt_d) / two_a,
        ))
    } else if d == 0.0 {
        Ok(QuadraticRoots::OneReal(-b / two_a))
    } else {
        let real = -b / two_a;
        let imag = (-d).sqrt() / two_a;
        Ok(QuadraticRoots::Complex((real, imag), (real, -imag)))
    }
}

/// Computes the determinant of a 2x2 matrix.
///
/// For `[[a, b], [c, d]]`, the determinant is `ad - bc`.
pub fn determinant_2x2(m: Matrix2x2) -> f64 {
    m[0][0] * m[1][1] - m[0][1] * m[1][0]
}

/// Computes the trace of a 2x2 matrix (`a + d`).
///
/// This is one of the other common matrix quantities used in linear algebra.
pub fn trace_2x2(m: Matrix2x2) -> f64 {
    m[0][0] + m[1][1]
}

/// Computes the transpose of a 2x2 matrix.
pub fn transpose_2x2(m: Matrix2x2) -> Matrix2x2 {
    [[m[0][0], m[1][0]], [m[0][1], m[1][1]]]
}

/// Computes the inverse of a 2x2 matrix.
///
/// For `[[a, b], [c, d]]`, inverse is:
/// `(1/det) * [[d, -b], [-c, a]]`.
///
/// # Errors
/// Returns [`MathError::SingularMatrix`] when determinant is zero.
pub fn inverse_2x2(m: Matrix2x2) -> Result<Matrix2x2, MathError> {
    let det = determinant_2x2(m);
    if det == 0.0 {
        return Err(MathError::SingularMatrix);
    }

    Ok([
        [m[1][1] / det, -m[0][1] / det],
        [-m[1][0] / det, m[0][0] / det],
    ])
}

/// Solves a 2x2 linear system `A * x = b` using Cramer's rule.
///
/// `a` is the coefficient matrix and `b` is `[b1, b2]`.
/// Returns solution `[x, y]`.
///
/// # Errors
/// Returns [`MathError::SingularMatrix`] when `det(A) == 0.0`.
pub fn solve_linear_2x2(a: Matrix2x2, b: [f64; 2]) -> Result<[f64; 2], MathError> {
    let det_a = determinant_2x2(a);
    if det_a == 0.0 {
        return Err(MathError::SingularMatrix);
    }

    let det_x = determinant_2x2([[b[0], a[0][1]], [b[1], a[1][1]]]);
    let det_y = determinant_2x2([[a[0][0], b[0]], [a[1][0], b[1]]]);

    Ok([det_x / det_a, det_y / det_a])
}
