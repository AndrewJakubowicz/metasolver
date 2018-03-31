//! Provides some acceptance functions that can be used with simulated annealing.
//! These functions will accept a move if they return a number over a
//! uniform value between 0 and 1.
//! 
//! Therefore any results over 1 will always be accepted, no matter how bad the move.
//! Any results below 0 will never be accepted.

pub fn create_boltzmann(constant: f32) -> Box<Fn(f32, f32) -> f32>
{
    Box::new(move |energy_diff: f32, temp: f32| ((-1.0 * energy_diff) / (constant * temp)).exp())
}

/// This will never allow a non improving solution to be accepted.
/// Useful for hill climbing without diversification.
pub fn never_accept(_: f32, _: f32) -> f32 {
    -1.0
}