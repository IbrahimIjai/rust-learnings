use algebra_basics_rs::{
    MathError, QuadraticRoots, determinant_2x2, discriminant, inverse_2x2, solve_linear_2x2,
    solve_quadratic, trace_2x2, transpose_2x2,
};

fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() < eps
}

#[test]
fn test_discriminant() {
    assert!(approx_eq(discriminant(1.0, -3.0, 2.0), 1.0, 1e-12));
}

#[test]
fn test_quadratic_two_real_roots() {
    let roots = solve_quadratic(1.0, -3.0, 2.0).unwrap();
    match roots {
        QuadraticRoots::TwoReal(r1, r2) => {
            assert!(approx_eq(r1, 2.0, 1e-12) || approx_eq(r1, 1.0, 1e-12));
            assert!(approx_eq(r2, 2.0, 1e-12) || approx_eq(r2, 1.0, 1e-12));
        }
        _ => panic!("expected two real roots"),
    }
}

#[test]
fn test_quadratic_one_real_root() {
    let roots = solve_quadratic(1.0, -2.0, 1.0).unwrap();
    match roots {
        QuadraticRoots::OneReal(r) => assert!(approx_eq(r, 1.0, 1e-12)),
        _ => panic!("expected one real root"),
    }
}

#[test]
fn test_quadratic_complex_roots() {
    let roots = solve_quadratic(1.0, 2.0, 5.0).unwrap();
    match roots {
        QuadraticRoots::Complex((re1, im1), (re2, im2)) => {
            assert!(approx_eq(re1, -1.0, 1e-12));
            assert!(approx_eq(re2, -1.0, 1e-12));
            assert!(approx_eq(im1, 2.0, 1e-12));
            assert!(approx_eq(im2, -2.0, 1e-12));
        }
        _ => panic!("expected complex roots"),
    }
}

#[test]
fn test_quadratic_not_quadratic() {
    let result = solve_quadratic(0.0, 2.0, 1.0);
    assert_eq!(result, Err(MathError::NotQuadratic));
}

#[test]
fn test_determinant_2x2() {
    let m = [[1.0, 2.0], [3.0, 4.0]];
    assert!(approx_eq(determinant_2x2(m), -2.0, 1e-12));
}

#[test]
fn test_trace_2x2() {
    let m = [[1.0, 2.0], [3.0, 4.0]];
    assert!(approx_eq(trace_2x2(m), 5.0, 1e-12));
}

#[test]
fn test_transpose_2x2() {
    let m = [[1.0, 2.0], [3.0, 4.0]];
    let t = transpose_2x2(m);
    assert_eq!(t, [[1.0, 3.0], [2.0, 4.0]]);
}

#[test]
fn test_inverse_2x2() {
    let m = [[4.0, 7.0], [2.0, 6.0]];
    let inv = inverse_2x2(m).unwrap();
    assert!(approx_eq(inv[0][0], 0.6, 1e-12));
    assert!(approx_eq(inv[0][1], -0.7, 1e-12));
    assert!(approx_eq(inv[1][0], -0.2, 1e-12));
    assert!(approx_eq(inv[1][1], 0.4, 1e-12));
}

#[test]
fn test_inverse_2x2_singular() {
    let m = [[1.0, 2.0], [2.0, 4.0]];
    let res = inverse_2x2(m);
    assert_eq!(res, Err(MathError::SingularMatrix));
}

#[test]
fn test_solve_linear_2x2() {
    // 2x + y = 11
    // 5x + 7y = 13
    let a = [[2.0, 1.0], [5.0, 7.0]];
    let b = [11.0, 13.0];
    let sol = solve_linear_2x2(a, b).unwrap();
    assert!(approx_eq(sol[0], 7.111_111_111_111_111, 1e-12));
    assert!(approx_eq(sol[1], -3.222_222_222_222_222_3, 1e-12));
}

#[test]
fn test_solve_linear_2x2_singular() {
    let a = [[1.0, 2.0], [2.0, 4.0]];
    let b = [3.0, 6.0];
    let res = solve_linear_2x2(a, b);
    assert_eq!(res, Err(MathError::SingularMatrix));
}
