//! Pre made simple monotonic cooling schedules to use with your
//! simulated annealing algorithms.
use super::Temperature;

/// A linear cooling schedule where the temperature is updated
/// by substracting a constant value.
pub struct Linear {
    temperature: f32,
    constant: f32,
    stopping: f32,
}

impl Linear {
    /// Create a new `Linear` cooling schedule.
    pub fn new(temperature: f32, constant: f32, stopping_value: f32) -> Self {
        Linear {
            temperature,
            constant,
            stopping: stopping_value,
        }
    }
}

impl<T> Temperature<T> for Linear {
    fn update(self, _: &T) -> Self {
        Linear::new(
            self.temperature - self.constant,
            self.constant,
            self.stopping,
        )
    }

    fn temperature(&self) -> f32 {
        self.temperature
    }

    fn stop(&self) -> bool {
        self.temperature < self.stopping
    }
}

/// A geometric cooling schedule, where the temperature is updated,
/// by multiplying by some value between 0 and 1 (exclusive).
/// The most popular cooling schedule and gets good results when used
/// with values between `0.5` and `0.99`.
pub struct Geometric {
    temperature: f32,
    constant: f32,
    stopping: f32,
}

impl Geometric {
    /// Create a new `Geometric` cooling schedule.
    pub fn new(temperature: f32, constant: f32, stopping_value: f32) -> Self {
        Geometric {
            temperature,
            constant,
            stopping: stopping_value,
        }
    }
}

impl<T> Temperature<T> for Geometric {
    fn update(self, _: &T) -> Self {
        Geometric::new(
            self.temperature * self.constant,
            self.constant,
            self.stopping,
        )
    }

    fn temperature(&self) -> f32 {
        self.temperature
    }

    fn stop(&self) -> bool {
        self.temperature < self.stopping
    }
}
