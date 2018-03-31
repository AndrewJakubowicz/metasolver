extern crate tsp_solver;

extern crate rand;

use tsp_solver::simulated_annealing::acceptance::boltzmann;
use tsp_solver::simulated_annealing::cooling_schedules;
use tsp_solver::simulated_annealing::{simulated_annealing, Fitness, Neighbours};

use rand::{thread_rng, Rng};

/// Find the maximum in the formula: `f(x) = x^3 - 60* x^2 + 900x + 100`
/// By changing the first 5 bits.
#[derive(Clone, Debug)]
struct solution_formula {
    min_x: u8,
}

impl Iterator for solution_formula {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        let mut r = thread_rng();
        Some(solution_formula {
            min_x: r.gen_range(0, 17),
        })
    }
}

impl Fitness for solution_formula {
    fn fitness(&self) -> f32 {
        let x = self.min_x as f32;
        
        let fit = x.powi(3) - 60.0 * x.powi(2) + 900.0 * x + 100.0;
        -1.0 * fit
    }
}

impl Neighbours<Self> for solution_formula {
    type Neighbour = solution_formula;

    fn neighbours(&self) -> Self {
        self.clone()
    }

    fn apply_neighbour(&mut self, n: Self::Neighbour) {
        self.min_x = n.min_x;
    }

    fn neighbour_fitness(&self, n: &Self::Neighbour) -> f32 {
        n.fitness()
    }
}

fn main() {
    println!("Min solution found: {:?}",
    simulated_annealing(
        solution_formula { min_x: thread_rng().gen_range(0, 17) },
        cooling_schedules::Geometric::new(800.0, 0.99, 0.001),
        boltzmann,
    ).expect("Solution failed"));
}
