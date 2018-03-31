use rand::{thread_rng, Rng};

pub mod cooling_schedules;

/// Returns a fitness.
/// The lower the number the better.
pub trait Fitness {
    fn fitness(&self) -> f32;
}

/// Type T is your iterator of neigbours.
/// Fitness of the neighbour is checked with
/// another fitness function allowing optimization.
///
/// For example your `neighbour_fitness` can return a heuristic answer.
pub trait Neighbours<T: Iterator<Item = Self::Neighbour>> {
    /// This is a neighbour.
    /// A type that when applied to the solution, modifies the solution in some way.
    type Neighbour;

    /// Returns an iterator over neighbours.
    fn neighbours(&self) -> T;

    /// Modifies the solution.
    fn apply_neighbour(&mut self, Self::Neighbour);
    fn neighbour_fitness(&self, &Self::Neighbour) -> f32;
}

/// An object with the `Temperature` trait has a temperature
/// which can be updated.
///
/// This is also called a cooling schedule.
///
/// If the solution hasn't experienced enough diversity, the
/// temperature can be increased.
pub trait Temperature<T> {
    /// Updates the temperature, allowing you to move the temperature.
    /// The reference to the solution allows non-monotonic cooling schedules.
    /// Thus you can increase the temperature if the solutions are lacking diversity.
    fn update(self, &T) -> Self;

    /// Returns a temperature.
    fn temperature(&self) -> f32;

    /// Stops the algorithm.
    fn stop(&self) -> bool;
}

/// Traditional simulated annealing algorithm.
/// Algorithm 2.1, from text "Metaheuristics, from design to implementation".
///
pub fn simulated_annealing<T, V, U, N>(
    intial_solution: T,
    initial_temperature: V,
    acceptance: U,
    max_loop: u32,
) -> Option<T>
where
    T: Fitness + Clone + Neighbours<N>,
    V: Temperature<T>,
    U: Fn(f32, f32) -> f32,
    N: Iterator<Item = T::Neighbour>,
{
    let mut s = intial_solution;
    let mut t = initial_temperature;
    let mut old_fitness = s.fitness();
    for _ in 0..max_loop {
        let (new_fitness, n) = {
            let mut iter = s.neighbours();
            loop {
                if let Some(n) = iter.next() {
                    let new_fitness = s.neighbour_fitness(&n);
                    let energy_diff = new_fitness - old_fitness;
                    // Always accept neighbours that are better.
                    // Otherwise decide via the acceptance function.
                    if energy_diff < 0.0
                        || acceptance(energy_diff, t.temperature())
                            < thread_rng().gen_range::<f32>(0.0, 1.0)
                    {
                        break (new_fitness, n);
                    }

                    if t.stop() {
                        return Some(s);
                    }
                } else {
                    // No more neighbours so return best answer.
                    return Some(s);
                }
            }
        };
        s.apply_neighbour(n);
        t = t.update(&s);
        old_fitness = new_fitness;
        if old_fitness == 0.0 {
            return Some(s);
        }
    }
    Some(s)
}
