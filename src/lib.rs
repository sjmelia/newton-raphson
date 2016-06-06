//! A minimal module for calculating roots using the Newton-Raphson method,
//! as described by [Wikipedia](https://en.wikipedia.org/wiki/Newton's_method)
//!
//! # Examples
//!
//! [Solution of cos(c) = x^3^]
//! (https://en.wikipedia.org/wiki/Newton%27s_method#Solution_of_cos.28x.29_.3D_x3)
//!
//! ```
//! use std::f64;
//! use newton_raphson::find_root;
//! fn cosx(x: f64) -> f64 {
//!     x.cos() - (x * x * x)
//! }
//!
//! fn cosx_d(x: f64) -> f64 {
//!     -x.sin() - 3.0 * (x * x)
//! }
//!
//! assert_eq!(0.8654740331016144, find_root(cosx, cosx_d, 0.5, 6));
//! ```

/// Finds the root of the function f
///
/// # Examples
///
/// [Finding the square root of a number]
/// (https://en.wikipedia.org/wiki/Newton%27s_method#Square_root_of_a_number)
///
/// ```
/// use newton_raphson::find_root;
/// fn sqrt_of_612(x: f64) -> f64 {
///     (x * x) - 612.0
/// }
///
/// fn sqrt_of_612_d(x: f64) -> f64 {
///     2.0 * x
/// }
///
/// assert_eq!(24.738633753766084, find_root(sqrt_of_612, sqrt_of_612_d, 10.0, 5))
/// ```
pub fn find_root(f: fn(f64) -> f64, fd: fn(f64) -> f64, guess: f64, iterations: i32) -> f64 {
    let mut result = guess;
    for _ in 0..iterations {
        result = iteration(f, fd, result);
    }
    result
}

/// Runs through one iteration of the Newton-Raphson method
///
/// # Examples
///
/// [Finding the square root of a number]
/// (https://en.wikipedia.org/wiki/Newton%27s_method#Square_root_of_a_number)
///
/// ```
/// use newton_raphson::iteration;
/// fn sqrt_of_612(x: f64) -> f64 {
///     (x * x) - 612.0
/// }
///
/// fn sqrt_of_612_d(x: f64) -> f64 {
///     2.0 * x
/// }
///
/// assert_eq!(35.6, iteration(sqrt_of_612, sqrt_of_612_d, 10.0));
/// ```
pub fn iteration(f: fn(f64) -> f64, fd: fn(f64) -> f64, guess: f64) -> f64 {
    guess - f(guess) / fd(guess)
}
