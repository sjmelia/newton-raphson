// vim: set ft=javascript ts=4 et sw=4 tw=80:

/// This function finds the root of the function f
///
/// # Examples
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

/// This function runs through one iteration of the Newton-Raphson method
///
/// # Examples
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
